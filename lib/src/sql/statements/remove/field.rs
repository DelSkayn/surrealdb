use crate::ctx::Context;
use crate::dbs::{Options, Transaction};
use crate::err::Error;
use crate::iam::{Action, ResourceKind};
use crate::sql::{Base, Ident, Idiom, Value};
use derive::Store;
use revision::revisioned;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Store, Hash)]
#[revisioned(revision = 1)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct RemoveFieldStatement {
	pub name: Idiom,
	pub what: Ident,
}

impl RemoveFieldStatement {
	/// Process this type returning a computed simple Value
	pub(crate) async fn compute(
		&self,
		_ctx: &Context<'_>,
		opt: &Options,
		txn: &Transaction,
	) -> Result<Value, Error> {
		// Allowed to run?
		opt.is_allowed(Action::Edit, ResourceKind::Field, &Base::Db)?;
		// Claim transaction
		let mut run = txn.lock().await;
		// Clear the cache
		run.clear_cache();
		// Delete the definition
		let fd = self.name.to_string();
		let start =
			crate::key::table::fd::field_prefix(opt.ns(), opt.db(), dbg!(&self.what), dbg!(&fd));
		let end =
			crate::key::table::fd::field_suffix(opt.ns(), opt.db(), dbg!(&self.what), dbg!(&fd));
		run.delr(start..end, u32::MAX).await?;
		// Clear the cache
		let key = crate::key::table::fd::prefix(opt.ns(), opt.db(), &self.what);
		run.clr(key).await?;
		// Ok all good
		Ok(Value::None)
	}
}

impl Display for RemoveFieldStatement {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "REMOVE FIELD {} ON {}", self.name, self.what)
	}
}
