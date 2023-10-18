use crate::{
	sql::{statements::UpdateStatement, Values},
	syn::{
		parser::{ParseResult, Parser},
		token::t,
	},
};

impl Parser<'_> {
	pub fn parse_update_stmt(&mut self) -> ParseResult<UpdateStatement> {
		let keyword = self.next();
		debug_assert_eq!(keyword.kind, t!("UPDATE"));

		let only = self.eat(t!("ONLY"));
		let what = Values(self.parse_what_list()?);
		let data = self.try_parse_data()?;
		let cond = self.try_parse_condition()?;
		let output = self.try_parse_output()?;
		let timeout = self.try_parse_timeout()?;
		let parallel = self.eat(t!("PARALLEL"));

		Ok(UpdateStatement {
			only,
			what,
			data,
			cond,
			output,
			timeout,
			parallel,
		})
	}
}

#[cfg(test)]
mod test {}
