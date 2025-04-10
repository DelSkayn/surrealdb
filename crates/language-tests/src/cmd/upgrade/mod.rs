mod binaries;
mod process;
mod protocol;

use std::{
	collections::HashMap,
	net::{Ipv4Addr, SocketAddr},
	sync::Arc,
	thread,
};

use anyhow::{bail, Context, Result};
use clap::ArgMatches;
use process::SurrealProcess;
use protocol::{ProxyObject, ProxyValue};
use semver::Version;
use surrealdb_core::kvs::Datastore;
use tokio::task::JoinSet;

use crate::{
	cli::{ColorMode, DsVersion, ResultsMode, UpgradeBackend},
	format::Progress,
	temp_dir::TempDir,
	tests::{
		report::{TestGrade, TestReport, TestTaskResult},
		set::TestId,
		TestSet,
	},
};

pub struct Task {
	from: DsVersion,
	to: DsVersion,
	test: TestId,
	path: String,
}

impl Task {
	pub fn name(&self, set: &TestSet) -> String {
		let mut res = String::new();

		res.push_str(&set[self.test].path);
		res.push_str(" ");

		match self.from {
			DsVersion::Path(ref x) => {
				res.push_str(x);
			}
			DsVersion::Version(ref v) => {
				res.push_str(&v.to_string());
			}
		}
		res.push_str(" => ");
		match self.to {
			DsVersion::Path(ref x) => {
				res.push_str(x);
			}
			DsVersion::Version(ref v) => {
				res.push_str(&v.to_string());
			}
		}

		res
	}
}

pub struct Config {
	test_path: String,
	jobs: u32,
	download_permission: bool,
	backend: UpgradeBackend,
}

impl Config {
	pub fn from_matches(matches: &ArgMatches) -> Self {
		Config {
			backend: *matches.get_one::<UpgradeBackend>("backend").unwrap(),
			test_path: matches.get_one::<String>("path").unwrap().clone(),
			download_permission: matches.get_one("allow-download").copied().unwrap_or(false),
			jobs: matches.get_one::<u32>("jobs").copied().unwrap_or_else(|| {
				thread::available_parallelism().map(|x| x.get() as u32).unwrap_or(1)
			}),
		}
	}
}

pub async fn run(color: ColorMode, matches: &ArgMatches) -> Result<()> {
	let config = Config::from_matches(matches);
	let config = Arc::new(config);

	let (testset, load_errors) = TestSet::collect_directory(&config.test_path).await?;

	let results_mode = matches.get_one::<ResultsMode>("results").unwrap();

	let from_versions = matches.get_many::<DsVersion>("from").unwrap().cloned().collect::<Vec<_>>();
	let to_versions = matches.get_many::<DsVersion>("to").unwrap().cloned().collect::<Vec<_>>();
	let mut all_versions: Vec<_> =
		from_versions.iter().cloned().chain(to_versions.iter().cloned()).collect();
	all_versions.sort_unstable();
	all_versions.dedup();

	// Check if the backend is supported by the enabled features.
	match config.backend {
		#[cfg(feature = "backend-rocksdb")]
		UpgradeBackend::RocksDb => {}
		#[cfg(not(feature = "backend-rocksdb"))]
		UpgradeBackend::RocksDb => bail!("RocksDb backend feature is not enabled"),
		#[cfg(feature = "backend-surrealkv")]
		UpgradeBackend::SurrealKv => {}
		#[cfg(not(feature = "backend-surrealkv"))]
		UpgradeBackend::SurrealKv => bail!("SurrealKV backend feature is not enabled"),
		#[cfg(any(feature = "backend-foundation-7_1", feature = "backend-foundation-7_1"))]
		UpgradeBackend::Foundation => {}
		#[cfg(not(any(feature = "backend-foundation-7_1", feature = "backend-foundation-7_1")))]
		UpgradeBackend::Foundation => bail!("FoundationDB backend features is not enabled"),
	}

	if UpgradeBackend::SurrealKv == config.backend {
		if let Some(DsVersion::Version(v)) =
			all_versions.iter().find(|x| **x < DsVersion::Version(Version::new(2, 0, 0)))
		{
			bail!("Cannot run with backend surrealkv and version {v}, surrealkv was not yet available on this version")
		}
	}

	let subset =
		testset.filter_map(|_, test| test.config.test.as_ref().map(|x| x.upgrade).unwrap_or(false));

	let subset = if let Some(x) = matches.get_one::<String>("filter") {
		subset.filter_map(|name, _| name.contains(x))
	} else {
		subset
	};

	let subset = if matches.get_flag("no-wip") {
		subset.filter_map(|_, set| !set.config.is_wip())
	} else {
		subset
	};

	let subset = if matches.get_flag("no-results") {
		subset.filter_map(|_, set| {
			!set.config.test.as_ref().map(|x| x.results.is_some()).unwrap_or(false)
		})
	} else {
		subset
	};

	println!("Preparing used versions of surrealdb");
	for v in all_versions {
		binaries::prepare(v, config.download_permission).await?;
	}

	let temp_dir = TempDir::new("surreal_upgrade_tests")
		.await
		.context("Failed to create temporary directory for datastore")?;

	let mut tasks = Vec::new();
	for from in from_versions.iter() {
		for to in to_versions.iter() {
			if let DsVersion::Version(ref from) = from {
				match to {
					DsVersion::Path(_) => {}
					DsVersion::Version(to) => {
						if from >= to {
							continue;
						}
					}
				}
			}

			for (t, _) in subset.iter_ids() {
				tasks.push(Task {
					from: from.clone(),
					to: to.clone(),
					test: t,
					path: temp_dir
						.sub_dir_path()
						.to_str()
						.expect("Paths should be utf-8")
						.to_owned(),
				})
			}
		}
	}

	println!("Running {} tasks for {} tests", tasks.len(), subset.len());

	let mut progress = Progress::from_stderr(tasks.len(), color);
	let mut task_iter = tasks.into_iter();

	let mut reports = Vec::new();

	let ds = Datastore::new("memory")
		.await
		.expect("failed to create datastore for running matching expressions");

	// Port distribution variables.
	let mut start_port = 9000u16;
	let mut reuse_port = Vec::<u16>::new();
	let mut used_ports = HashMap::new();

	let mut join_set = JoinSet::<(TestId, TestTaskResult)>::new();
	loop {
		while join_set.len() < config.jobs as usize {
			if let Some(task) = task_iter.next() {
				let port = reuse_port.pop().or_else(|| {
					while let Some(x) = start_port.checked_add(1) {
						start_port = x;
						if is_port_available(x) {
							return Some(x);
						}
					}
					None
				});
				let Some(port) = port else {
					// wait for some more ports to be available.
					break;
				};

				let name = task.name(&testset);
				let id = join_set.spawn(run_task(task, testset.clone(), port, config.clone())).id();

				used_ports.insert(id, port);

				progress.start_item(id, &name).unwrap();
			} else {
				break;
			}
		}

		let Some(res) = join_set.join_next_with_id().await else {
			break;
		};

		let (id, (test_id, result)) = res.unwrap();

		let used_port =
			used_ports.remove(&id).expect("previously used ports should be in the ports map");
		reuse_port.push(used_port);

		let report = TestReport::from_test_result(test_id, &testset, result, &ds).await;
		let grade = report.grade();
		reports.push(report);
		progress.finish_item(id, grade).unwrap();
	}

	if let Err(e) = temp_dir.cleanup().await {
		println!();
		println!("Failed to clean up temporary directory:{e}");
	}

	println!();

	for v in reports.iter() {
		v.display(&subset, color);
	}

	println!();
	// Report test case loading errors.
	for e in load_errors.iter() {
		e.display(color)
	}

	match *results_mode {
		ResultsMode::Default => {}
		ResultsMode::Accept => {
			for report in reports.iter().filter(|x| x.is_unspecified_test() && !x.is_wip()) {
				report.update_config_results(&subset).await?;
			}
		}
		ResultsMode::Overwrite => {
			for report in reports.iter().filter(|x| {
				matches!(x.grade(), TestGrade::Failed | TestGrade::Warning) && !x.is_wip()
			}) {
				report.update_config_results(&subset).await?;
			}
		}
	}

	if !load_errors.is_empty() {
		bail!("Could not load all tests")
	}

	if reports.iter().any(|x| x.grade() == TestGrade::Failed) {
		bail!("Not all tests where successfull")
	}

	Ok(())
}

fn is_port_available(port: u16) -> bool {
	std::net::TcpListener::bind(SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), port)).is_ok()
}

async fn run_task(
	task: Task,
	test_set: TestSet,
	port: u16,
	config: Arc<Config>,
) -> (TestId, TestTaskResult) {
	let id = task.test;
	match run_task_inner(task, test_set, port, config).await {
		Ok(x) => (id, x),
		Err(e) => (id, TestTaskResult::RunningError(e)),
	}
}

async fn run_imports(
	task: &Task,
	set: &TestSet,
	process: &mut SurrealProcess,
	namespace: Option<&str>,
	database: Option<&str>,
) -> anyhow::Result<Option<TestTaskResult>> {
	let Some(imports) = set[task.test].config.env.as_ref().and_then(|x| x.imports.as_ref()) else {
		bail!("Upgrade test did not specify any imports, can't run upgrade tests without first importing data")
	};

	let mut connection = process
		.open_connection()
		.await
		.context("Failed to open connection to upgrade from database")?;

	let mut params = Vec::new();
	if let Some(ns) = namespace {
		params.push(ProxyValue::from(ns))
	}
	if let Some(db) = database {
		params.push(ProxyValue::from(db))
	}

	if !params.is_empty() {
		let mut req = ProxyObject::default();
		req.insert("method".to_owned(), ProxyValue::from("use"));
		req.insert("params".to_owned(), ProxyValue::from(params));

		let resp = connection
			.request(req)
			.await
			.context("Failed to set namespace/database on importing database")?;
		if let Err(e) = resp.result {
			bail!("Failed to set namespace/database on importing database: {}", e.message)
		}
	}

	let mut credentials = ProxyObject::default();
	credentials.insert("user".to_owned(), ProxyValue::from("root"));
	credentials.insert("pass".to_owned(), ProxyValue::from("root"));

	let mut req = ProxyObject::default();
	req.insert("method".to_owned(), ProxyValue::from("signin"));
	req.insert("params".to_owned(), ProxyValue::from(vec![ProxyValue::from(credentials)]));

	let resp =
		connection.request(req).await.context("Failed to authenticate on importing database")?;
	if let Err(e) = resp.result {
		bail!("Failed to authenticate on importing database: {}", e.message)
	}

	for import in imports {
		let Some(test) = set.find_all(import) else {
			return Ok(Some(TestTaskResult::Import(
				import.to_owned(),
				format!("Could not find import."),
			)));
		};

		let Ok(source) = std::str::from_utf8(&set[test].source) else {
			return Ok(Some(TestTaskResult::Import(
				import.to_owned(),
				format!("Import file was not valid utf-8."),
			)));
		};

		match connection.query(source).await {
			Ok(TestTaskResult::RunningError(e)) => {
				return Ok(Some(TestTaskResult::Import(
					import.to_owned(),
					format!("Failed to run import: {e:?}"),
				)))
			}
			Err(e) => {
				return Ok(Some(TestTaskResult::Import(
					import.to_owned(),
					format!("Failed to run import: {e:?}."),
				)));
			}
			_ => {}
		}
	}

	Ok(None)
}

async fn run_task_inner(
	task: Task,
	test_set: TestSet,
	port: u16,
	config: Arc<Config>,
) -> Result<TestTaskResult> {
	let dir = &task.path;
	tokio::fs::create_dir(dir).await.context("Failed to create tempory directory for datastore")?;

	if test_set[task.test].config.env.as_ref().and_then(|x| x.capabilities.as_ref()).is_some() {
		bail!("Setting capabilities are not supported for upgrade tests")
	}

	let mut process = process::SurrealProcess::new(&config, &task.from, dir, port).await?;

	let namespace =
		test_set[task.test].config.env.as_ref().map(|x| x.namespace()).unwrap_or(Some("test"));
	let database =
		test_set[task.test].config.env.as_ref().map(|x| x.database()).unwrap_or(Some("test"));

	if database.is_some() && namespace.is_none() {
		bail!("Cannot have a database set but not a namespace.")
	}

	match run_imports(&task, &test_set, &mut process, namespace, database).await? {
		Some(x) => return Ok(x),
		None => {}
	};

	process.quit().await?;

	let mut process = process::SurrealProcess::new(&config, &task.to, dir, port).await?;

	let mut connection = process
		.open_connection()
		.await
		.context("Failed to open connection to upgrading datastore")?;

	let mut params = Vec::new();
	if let Some(ns) = namespace {
		params.push(ProxyValue::from(ns))
	}
	if let Some(db) = database {
		params.push(ProxyValue::from(db))
	}

	if !params.is_empty() {
		let mut req = ProxyObject::default();
		req.insert("method".to_owned(), ProxyValue::from("use"));
		req.insert("params".to_owned(), ProxyValue::from(params));

		let resp = connection
			.request(req)
			.await
			.context("Failed to set namespace/database on upgrading database")?;
		if let Err(e) = resp.result {
			bail!("Failed to set namespace/database on upgrading database: {}", e.message)
		}
	}

	let mut credentials = ProxyObject::default();
	credentials.insert("user".to_owned(), ProxyValue::from("root"));
	credentials.insert("pass".to_owned(), ProxyValue::from("root"));

	let mut req = ProxyObject::default();
	req.insert("method".to_owned(), ProxyValue::from("signin"));
	req.insert("params".to_owned(), ProxyValue::from(vec![ProxyValue::from(credentials)]));

	let resp =
		connection.request(req).await.context("Failed to authenticate on upgrading database")?;
	if let Err(e) = resp.result {
		bail!("Failed to authenticate on upgrading database: {}", e.message)
	}

	let source = &test_set[task.test].source;
	let source = std::str::from_utf8(source).context("Text source was not valid utf-8")?;
	let result = match connection.query(&source).await {
		Ok(x) => x,
		Err(e) => {
			let output = process.quit_with_output().await?;
			bail!(
				"{e:?}\n> process stdout:\n{}\n\n> process stderr: \n{}",
				output.stdout,
				output.stderr
			)
		}
	};

	let _ = tokio::fs::remove_dir_all(dir).await;

	Ok(result)
}
