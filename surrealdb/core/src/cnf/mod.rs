pub(crate) mod dynamic;

use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::LazyLock;
use std::time::Duration;

use crate::iam::file::extract_allowed_paths;

/// The publicly visible name of the server
pub const SERVER_NAME: &str = "SurrealDB";

/// The characters which are supported in server record IDs
pub const ID_CHARS: [char; 36] = [
	'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
	'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// Specifies the names of parameters which can not be specified in a query
pub const PROTECTED_PARAM_NAMES: &[&str] = &["access", "auth", "token", "session"];

/// A map with a set of configuration values stored as pairs of strings.
#[derive(Clone, Debug)]
pub struct ConfigMap {
	values: HashMap<String, String>,
}

impl ConfigMap {
	/// Returns an empty map.
	pub fn empty() -> Self {
		ConfigMap {
			values: HashMap::new(),
		}
	}

	/// Adds a new key and value into the config map, will overwrite existing values.
	pub fn with_key_value<K, V>(mut self, key: K, value: V) -> Self
	where
		String: From<K>,
		String: From<V>,
	{
		self.values.insert(key.into(), value.into());
		self
	}

	/// Creates a config map from all the environment variables prefixed with `SURREAL_`
	pub fn from_env() -> Self {
		Self::from_env_prefix("SURREAL_")
	}

	/// Creates a config map from all the environment variables prefixed with the specific prefix.
	pub fn from_env_prefix(prefix: &str) -> Self {
		let mut values = HashMap::new();
		for (k, v) in std::env::vars() {
			let Some(x) = k.strip_prefix(prefix) else {
				continue;
			};

			let key_name = x.to_lowercase();
			values.insert(key_name, v);
		}
		ConfigMap {
			values,
		}
	}

	/// Map all the keys in the config map with the given closure.
	pub fn map_keys<F: FnMut(String) -> String>(self, mut f: F) -> Self {
		Self {
			values: self.values.into_iter().map(|(k, v)| (f(k), v)).collect(),
		}
	}

	/// Creates a config map from all the environment variables
	pub fn from_config_string(s: &str) -> Self {
		let values = s
			.split('&')
			.filter_map(|x| {
				let (k, v) = x.split_once('=')?;
				Some((k.to_lowercase(), v.to_string()))
			})
			.collect();

		ConfigMap {
			values,
		}
	}

	/// Join two config maps together prefering the values not in self.
	pub fn join(mut self, other: ConfigMap) -> Self {
		for (k, v) in other.values {
			self.values.insert(k, v);
		}
		self
	}

	/// Load a config type from the map.
	pub fn load<C: Config>(&self) -> C {
		let mut def = C::default();
		def.parse(self);
		def
	}

	/// Parse a value out of the map if it exists.
	///
	/// If either the key does not exist or the parsing values the value is unaltered.
	pub fn parse_key<S: FromStr>(&self, key: &str, value: &mut S) -> &Self {
		self.parse_key_with(key, value, |x| S::from_str(x).ok())
	}

	/// Parse a boolean out of the map if it exists.
	///
	/// If either the key does not exist or the parsing values the value is unaltered.
	pub fn parse_key_bool(&self, key: &str, value: &mut bool) -> &Self {
		self.parse_key_with(key, value, |x| {
			if x.eq_ignore_ascii_case("true") || x == "1" {
				Some(true)
			} else if x.eq_ignore_ascii_case("false") || x == "0" {
				Some(false)
			} else {
				None
			}
		})
	}

	/// Parse a value out of the map if it exists.
	/// Takes a closure which can be used to define how to parse the string
	///
	/// If either the key does not exist or the parsing closure returns `None` the value is unaltered.
	pub fn parse_key_with<R, F: FnOnce(&str) -> Option<R>>(
		&self,
		key: &str,
		value: &mut R,
		f: F,
	) -> &Self {
		let Some(v) = self.values.get(key) else {
			return self;
		};

		let Some(v) = f(v) else {
			return self;
		};

		*value = v;
		self
	}
}

/// Trait for types which contain configureation information.
pub trait Config: Default {
	fn parse(&mut self, map: &ConfigMap);
}

#[derive(Debug)]
pub struct CommonConfig {
	pub memory_threshold: usize,
	/// Specifies how many concurrent jobs can be buffered in the worker channel
	pub max_concurrent_tasks: usize,
	/// Specifies how deep recursive computation will go before erroring (default:
	/// 120)
	pub max_computation_depth: u32,
	/// Specifies how deep the parser will parse nested objects and arrays (default:
	/// 100)
	pub max_object_parsing_depth: u32,
	/// Specifies how deep the parser will parse recursive queries (default: 20)
	pub max_query_parsing_depth: u32,
	/// The maximum recursive idiom path depth allowed (default: 256)
	pub idiom_recursion_limit: u32,
	/// The maximum size of a compiled regular expression (default: 10 MiB)
	pub regex_size_limit: usize,
	/// Specifies the number of computed regexes which can be cached in the engine
	/// (default: 1000)
	pub regex_cache_size: usize,
	/// Specifies the number of items which can be cached within a single
	/// transaction (default: 10,000)
	pub transaction_cache_size: usize,
	/// Specifies the number of definitions which can be cached across transactions
	/// (default: 1,000)
	pub datastore_cache_size: usize,
	/// Specifies the number of surrealism modules which can be cached across transactions
	/// (default: 100)
	pub surrealism_cache_size: usize,
	/// The maximum number of keys that should be scanned at once in general queries
	/// (default: 500)
	pub normal_fetch_size: u32,
	/// The maximum number of keys that should be scanned at once for export queries
	/// (default: 1000)
	pub export_batch_size: u32,
	/// The maximum number of keys that should be scanned at once for count queries
	/// (default: 50,000)
	pub count_batch_size: u32,
	/// The maximum number of keys to scan at once per concurrent indexing batch
	/// (default: 250)
	pub indexing_batch_size: u32,
	/// The number of batches each operator buffers ahead of downstream demand.
	/// Set to 0 to disable operator-level pipeline buffering.
	/// (default: 2)
	pub operator_buffer_size: usize,
	/// The maximum size of the priority queue triggering usage of the priority
	/// queue for the result collector.
	pub max_order_limit_priority_queue_size: u32,
	/// The maximum stack size of the JavaScript function runtime (default: 256 KiB)
	pub scripting_max_stack_size: usize,
	/// The maximum memory limit of the JavaScript function runtime (default: 2 MiB)
	pub scripting_max_memory_limit: usize,
	/// The maximum amount of time that a JavaScript function can run (default: 5
	/// seconds)
	pub scripting_max_time_limit: Duration,
	/// The maximum number of HTTP redirects allowed within http functions (default:
	/// 10)
	pub max_http_redirects: usize,
	/// The maximum number of idle HTTP connections to maintain per host (default: 128)
	pub max_http_idle_connections_per_host: usize,
	/// The maximum number of total idle HTTP connections to maintain (default: 1000)
	pub max_http_idle_connections: usize,
	/// The timeout for idle HTTP connections before closing (default: 90 seconds)
	pub http_idle_timeout_secs: u64,
	/// The timeout for connecting to HTTP endpoints (default: 30 seconds)
	pub http_connect_timeout_secs: u64,
	/// Forward all authentication errors to the client. Do not use in production
	/// (default: false)
	pub insecure_forward_access_errors: bool,
	/// The number of result records which will trigger on-disk sorting (default:
	/// 50,000)
	pub external_sorting_buffer_limit: usize,
	/// Used to limit allocation for builtin functions. Default: 2^20 (1 MiB),
	/// can be as large as 28 (2^28, 256 MiB)
	pub generation_allocation_limit: usize,
	/// The maximum input string length for similarity/distance functions (default:
	/// 16384 bytes)
	pub string_similarity_limit: usize,
	/// Specifies a list of paths in which files can be accessed (default: empty)
	pub file_allowlist: Vec<PathBuf>,
	/// Specifies a list of paths in which files can be accessed (default: empty)
	pub bucket_folder_allowlist: Vec<PathBuf>,
	/// Specify the name of a global bucket for file data (default: None)
	pub global_bucket: Option<String>,
	/// Whether to enforce a global bucket for file data (default: false)
	pub global_bucket_enforced: bool,
	/// Specify the USER-AGENT string used by HTTP requests
	pub surrealdb_user_agent: String,
	/// The maximum size of the HNSW vector cache (default: 256 MiB)
	pub hnsw_cache_size: u64,
}

impl Default for CommonConfig {
	fn default() -> Self {
		Self {
			memory_threshold: 0,
			#[cfg(not(target_family = "wasm"))]
			max_concurrent_tasks: 64,
			#[cfg(target_family = "wasm")]
			max_concurrent_tasks: 1,
			max_computation_depth: 120,
			max_object_parsing_depth: 100,
			max_query_parsing_depth: 20,
			idiom_recursion_limit: 256,
			regex_size_limit: 10 * 1024 * 1024,
			regex_cache_size: 1_000,
			transaction_cache_size: 10_000,
			datastore_cache_size: 1_000,
			surrealism_cache_size: 100,
			normal_fetch_size: 500,
			export_batch_size: 1000,
			count_batch_size: 50_000,
			indexing_batch_size: 250,
			operator_buffer_size: 2,
			max_order_limit_priority_queue_size: 1000,
			scripting_max_stack_size: 256 * 1024,
			scripting_max_memory_limit: 2 << 20,
			scripting_max_time_limit: Duration::from_secs(5),
			max_http_redirects: 10,
			max_http_idle_connections_per_host: 128,
			max_http_idle_connections: 1000,
			http_idle_timeout_secs: 90,
			http_connect_timeout_secs: 30,
			insecure_forward_access_errors: false,
			external_sorting_buffer_limit: 50_000,
			generation_allocation_limit: 2 << 20,
			string_similarity_limit: 16384,
			file_allowlist: Vec::new(),
			bucket_folder_allowlist: Vec::new(),
			global_bucket: None,
			global_bucket_enforced: false,
			surrealdb_user_agent: "SurrealDB".to_string(),
			hnsw_cache_size: 256 * 1024 * 1024,
		}
	}
}

impl Config for CommonConfig {
	fn parse(&mut self, map: &ConfigMap) {
		map.parse_key_with("memory_threshold", &mut self.memory_threshold, |x| {
			x.parse::<usize>().map(|x| x.max(1024 * 1024)).ok()
		})
		.parse_key("max_concurrent_tasks", &mut self.max_concurrent_tasks)
		.parse_key("max_computation_depth", &mut self.max_computation_depth)
		.parse_key("max_object_parsing_depth", &mut self.max_object_parsing_depth)
		.parse_key("max_query_parsing_depth", &mut self.max_query_parsing_depth)
		.parse_key("regex_size_limit", &mut self.regex_size_limit)
		.parse_key("regex_cache_size", &mut self.regex_cache_size)
		.parse_key("transaction_cache_size", &mut self.transaction_cache_size)
		.parse_key("datastore_cache_size", &mut self.datastore_cache_size)
		.parse_key("surrealism_cache_size", &mut self.surrealism_cache_size)
		.parse_key("normal_fetch_size", &mut self.normal_fetch_size)
		.parse_key("export_batch_size", &mut self.export_batch_size)
		.parse_key("count_batch_size", &mut self.count_batch_size)
		.parse_key("indexing_batch_size", &mut self.indexing_batch_size)
		.parse_key("operator_buffer_size", &mut self.operator_buffer_size)
		.parse_key(
			"max_order_limit_priority_queue_size",
			&mut self.max_order_limit_priority_queue_size,
		)
		.parse_key("scripting_max_stack_size", &mut self.scripting_max_stack_size)
		.parse_key("scripting_max_memory_limit", &mut self.scripting_max_memory_limit)
		.parse_key_with("scripting_max_time_limit", &mut self.scripting_max_time_limit, |x| {
			x.parse().map(Duration::from_millis).ok()
		})
		.parse_key("max_http_redirects", &mut self.max_http_redirects)
		.parse_key(
			"max_http_idle_connections_per_host",
			&mut self.max_http_idle_connections_per_host,
		)
		.parse_key("max_http_idle_connections", &mut self.max_http_idle_connections)
		.parse_key("http_idle_timeout_secs", &mut self.http_idle_timeout_secs)
		.parse_key("http_connect_timeout_secs", &mut self.http_connect_timeout_secs)
		.parse_key("insecure_forward_access_errors", &mut self.insecure_forward_access_errors)
		.parse_key("external_sorting_buffer_limit", &mut self.external_sorting_buffer_limit)
		.parse_key_with("generation_allocation_limit", &mut self.generation_allocation_limit, |x| {
			x.parse::<usize>().ok().map(|x| 2 << x.min(28))
		})
		.parse_key("string_similarity_limit", &mut self.string_similarity_limit)
		.parse_key_with("file_allowlist", &mut self.file_allowlist, |x| {
			Some(extract_allowed_paths(x, true, "file"))
		})
		.parse_key_with("bucket_folder_allowlist", &mut self.file_allowlist, |x| {
			Some(extract_allowed_paths(x, false, "bucket folder"))
		})
		.parse_key_with("global_bucket", &mut self.global_bucket, |x| Some(Some(x.to_owned())))
		.parse_key("global_bucket_enforced", &mut self.global_bucket_enforced)
		.parse_key("surrealdb_user_agent", &mut self.surrealdb_user_agent)
		.parse_key("hnsw_cache_size", &mut self.hnsw_cache_size);
	}
}

/// The memory usage threshold before tasks are forced to exit (default: 0
/// bytes). The default 0 bytes means that there is no memory threshold.
/// Any other user-set memory threshold will default to at least 1 MiB.
pub static MEMORY_THRESHOLD: LazyLock<usize> = LazyLock::new(|| {
	let n = std::env::var("SURREAL_MEMORY_THRESHOLD")
		.map(|s| s.parse::<usize>().unwrap_or(0))
		.unwrap_or(0);
	match n {
		default @ 0 => default,
		specified => std::cmp::max(specified, 1024 * 1024),
	}
});

/// The maximum size of a compiled regular expression (default: 10 MiB)
pub static REGEX_SIZE_LIMIT: LazyLock<usize> =
	lazy_env_parse!("SURREAL_REGEX_SIZE_LIMIT", usize, 10 * 1024 * 1024);

/// Specifies the number of computed regexes which can be cached in the engine
/// (default: 1000)
pub static REGEX_CACHE_SIZE: LazyLock<usize> =
	lazy_env_parse!("SURREAL_REGEX_CACHE_SIZE", usize, 1_000);

/// Specifies the number of items which can be cached within a single
/// transaction (default: 10,000)
pub static TRANSACTION_CACHE_SIZE: LazyLock<usize> =
	lazy_env_parse!("SURREAL_TRANSACTION_CACHE_SIZE", usize, 10_000);

/// Specifies the number of definitions which can be cached across transactions
/// (default: 1,000)
pub static DATASTORE_CACHE_SIZE: LazyLock<usize> =
	lazy_env_parse!("SURREAL_DATASTORE_CACHE_SIZE", usize, 1_000);

/// Specifies the number of surrealism modules which can be cached across transactions
/// (default: 100)
pub static SURREALISM_CACHE_SIZE: LazyLock<usize> =
	lazy_env_parse!("SURREAL_SURREALISM_CACHE_SIZE", usize, 100);

/// The maximum number of keys that should be scanned at once in general queries
/// (default: 500)
pub static NORMAL_FETCH_SIZE: LazyLock<u32> =
	lazy_env_parse!("SURREAL_NORMAL_FETCH_SIZE", u32, 500);

/// The maximum number of keys that should be scanned at once for export queries
/// (default: 1000)
pub static EXPORT_BATCH_SIZE: LazyLock<u32> =
	lazy_env_parse!("SURREAL_EXPORT_BATCH_SIZE", u32, 1000);

/// The maximum number of keys that should be scanned at once for count queries
/// (default: 50,000)
pub static COUNT_BATCH_SIZE: LazyLock<u32> =
	lazy_env_parse!("SURREAL_COUNT_BATCH_SIZE", u32, 50_000);

/// The maximum number of keys to scan at once per concurrent indexing batch
/// (default: 250)
pub static INDEXING_BATCH_SIZE: LazyLock<u32> =
	lazy_env_parse!("SURREAL_INDEXING_BATCH_SIZE", u32, 250);

/// The number of batches each operator buffers ahead of downstream demand.
/// Set to 0 to disable operator-level pipeline buffering.
/// (default: 2)
#[cfg(not(target_family = "wasm"))]
pub static OPERATOR_BUFFER_SIZE: LazyLock<usize> =
	lazy_env_parse!("SURREAL_OPERATOR_BUFFER_SIZE", usize, 2);

/// The maximum size of the priority queue triggering usage of the priority
/// queue for the result collector.
pub static MAX_ORDER_LIMIT_PRIORITY_QUEUE_SIZE: LazyLock<u32> =
	lazy_env_parse!("SURREAL_MAX_ORDER_LIMIT_PRIORITY_QUEUE_SIZE", u32, 1000);

/// The maximum stack size of the JavaScript function runtime (default: 256 KiB)
pub static SCRIPTING_MAX_STACK_SIZE: LazyLock<usize> =
	lazy_env_parse!("SURREAL_SCRIPTING_MAX_STACK_SIZE", usize, 256 * 1024);

/// The maximum memory limit of the JavaScript function runtime (default: 2 MiB)
pub static SCRIPTING_MAX_MEMORY_LIMIT: LazyLock<usize> =
	lazy_env_parse!("SURREAL_SCRIPTING_MAX_MEMORY_LIMIT", usize, 2 << 20);

/// The maximum amount of time that a JavaScript function can run (default: 5
/// seconds)
pub static SCRIPTING_MAX_TIME_LIMIT: LazyLock<usize> =
	lazy_env_parse!("SURREAL_SCRIPTING_MAX_TIME_LIMIT", usize, 5 * 1000);

/// The maximum number of HTTP redirects allowed within http functions (default:
/// 10)
pub static MAX_HTTP_REDIRECTS: LazyLock<usize> =
	lazy_env_parse!("SURREAL_MAX_HTTP_REDIRECTS", usize, 10);

/// The maximum number of idle HTTP connections to maintain per host (default: 128)
pub static MAX_HTTP_IDLE_CONNECTIONS_PER_HOST: LazyLock<usize> =
	lazy_env_parse!("SURREAL_MAX_HTTP_IDLE_CONNECTIONS_PER_HOST", usize, 128);

/// The maximum number of total idle HTTP connections to maintain (default: 1000)
pub static MAX_HTTP_IDLE_CONNECTIONS: LazyLock<usize> =
	lazy_env_parse!("SURREAL_MAX_HTTP_IDLE_CONNECTIONS", usize, 1000);

/// The timeout for idle HTTP connections before closing (default: 90 seconds)
pub static HTTP_IDLE_TIMEOUT_SECS: LazyLock<u64> =
	lazy_env_parse!("SURREAL_HTTP_IDLE_TIMEOUT_SECS", u64, 90);

/// The timeout for connecting to HTTP endpoints (default: 30 seconds)
pub static HTTP_CONNECT_TIMEOUT_SECS: LazyLock<u64> =
	lazy_env_parse!("SURREAL_HTTP_CONNECT_TIMEOUT_SECS", u64, 30);

/// Forward all authentication errors to the client. Do not use in production
/// (default: false)
pub static INSECURE_FORWARD_ACCESS_ERRORS: LazyLock<bool> =
	lazy_env_parse!("SURREAL_INSECURE_FORWARD_ACCESS_ERRORS", bool, false);

/// The number of result records which will trigger on-disk sorting (default:
/// 50,000)
#[cfg(storage)]
pub static EXTERNAL_SORTING_BUFFER_LIMIT: LazyLock<usize> =
	lazy_env_parse!("SURREAL_EXTERNAL_SORTING_BUFFER_LIMIT", usize, 50_000);

/// Used to limit allocation for builtin functions. Default: 2^20 (1 MiB),
/// can be as large as 28 (2^28, 256 MiB)
pub static GENERATION_ALLOCATION_LIMIT: LazyLock<usize> = LazyLock::new(|| {
	let n = std::env::var("SURREAL_GENERATION_ALLOCATION_LIMIT")
		.map(|s| s.parse::<u32>().unwrap_or(20))
		.unwrap_or(20);
	2usize.pow(n.min(28))
});

/// The maximum input string length for similarity/distance functions (default:
/// 16384 bytes)
pub static STRING_SIMILARITY_LIMIT: LazyLock<usize> =
	lazy_env_parse!("SURREAL_STRING_SIMILARITY_LIMIT", usize, 16384);

/// Specifies a list of paths in which files can be accessed (default: empty)
pub static FILE_ALLOWLIST: LazyLock<Vec<PathBuf>> = LazyLock::new(|| {
	std::env::var("SURREAL_FILE_ALLOWLIST")
		.map(|input| extract_allowed_paths(&input, true, "file"))
		.unwrap_or_default()
});

/// Specifies a list of paths in which files can be accessed (default: empty)
pub static BUCKET_FOLDER_ALLOWLIST: LazyLock<Vec<PathBuf>> = LazyLock::new(|| {
	std::env::var("SURREAL_BUCKET_FOLDER_ALLOWLIST")
		.map(|input| extract_allowed_paths(&input, false, "bucket folder"))
		.unwrap_or_default()
});

/// Specify the name of a global bucket for file data (default: None)
pub static GLOBAL_BUCKET: LazyLock<Option<String>> =
	lazy_env_parse!("SURREAL_GLOBAL_BUCKET", Option<String>);

/// Whether to enforce a global bucket for file data (default: false)
pub static GLOBAL_BUCKET_ENFORCED: LazyLock<bool> =
	lazy_env_parse!("SURREAL_GLOBAL_BUCKET_ENFORCED", bool, false);

/// Specify the USER-AGENT string used by HTTP requests
pub static SURREALDB_USER_AGENT: LazyLock<String> =
	LazyLock::new(|| std::env::var("SURREAL_USER_AGENT").unwrap_or("SurrealDB".to_string()));

/// The maximum size of the HNSW vector cache (default: 256 MiB)
pub static HNSW_CACHE_SIZE: LazyLock<u64> =
	lazy_env_parse!("SURREAL_HNSW_CACHE_SIZE", u64, 256 * 1024 * 1024);
