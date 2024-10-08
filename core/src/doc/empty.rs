use crate::ctx::Context;
use crate::dbs::Options;
use crate::dbs::Statement;
use crate::doc::Document;
use crate::err::Error;

impl Document {
	pub async fn empty(
		&self,
		_ctx: &Context,
		_opt: &Options,
		_stm: &Statement<'_>,
	) -> Result<(), Error> {
		// Check if this record exists
		if self.id.is_some() {
			// There is no current record
			if self.current.doc.as_ref().is_none() {
				// Ignore this requested record
				return Err(Error::Ignore);
			}
		}
		// Carry on
		Ok(())
	}
}
