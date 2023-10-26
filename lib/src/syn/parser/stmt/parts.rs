//! Contains parsing code for smaller common parts of statements.

use crate::{
	sql::{
		changefeed::ChangeFeed, index::Distance, Base, Cond, Data, Fetch, Fetchs, Group, Groups,
		Ident, Operator, Output, Permission, Permissions, Table, Tables, Timeout, View,
	},
	syn::{
		parser::{
			mac::{expected, unexpected},
			ParseResult, Parser,
		},
		token::{t, DistanceKind, TokenKind},
	},
};

impl Parser<'_> {
	/// Parses a data production if the next token is a data keyword.
	/// Otherwise returns None
	pub fn try_parse_data(&mut self) -> ParseResult<Option<Data>> {
		let res = match self.next().kind {
			t!("SET") => {
				let mut set_list = Vec::new();
				loop {
					let idiom = self.parse_plain_idiom()?;
					let operator = match self.next().kind {
						t!("=") => Operator::Equal,
						t!("+=") => Operator::Sub,
						t!("-=") => Operator::Add,
						t!("+?=") => Operator::Ext,
						x => unexpected!(self, x, "a assign operator"),
					};
					let value = self.parse_value()?;
					set_list.push((idiom, operator, value));
					if !self.eat(t!(",")) {
						break;
					}
				}
				Data::SetExpression(set_list)
			}
			t!("UNSET") => {
				let idiom_list = self.parse_idiom_list()?;
				Data::UnsetExpression(idiom_list)
			}
			t!("PATCH") => Data::PatchExpression(self.parse_value()?),
			t!("MERGE") => Data::MergeExpression(self.parse_value()?),
			t!("REPLACE") => Data::ReplaceExpression(self.parse_value()?),
			t!("CONTENT") => Data::ContentExpression(self.parse_value()?),
			_ => return Ok(None),
		};
		Ok(Some(res))
	}

	pub fn try_parse_output(&mut self) -> ParseResult<Option<Output>> {
		if !self.eat(t!("RETURN")) {
			return Ok(None);
		}
		let res = match self.peek_kind() {
			t!("NONE") => {
				self.pop_peek();
				Output::None
			}
			t!("NULL") => {
				self.pop_peek();
				Output::Null
			}
			t!("DIFF") => {
				self.pop_peek();
				Output::Diff
			}
			t!("AFTER") => {
				self.pop_peek();
				Output::After
			}
			t!("BEFORE") => {
				self.pop_peek();
				Output::Before
			}
			_ => Output::Fields(self.parse_fields()?),
		};
		Ok(Some(res))
	}

	pub fn try_parse_timeout(&mut self) -> ParseResult<Option<Timeout>> {
		if !self.eat(t!("TIMEOUT")) {
			return Ok(None);
		}
		let duration = self.parse_duration()?;
		Ok(Some(Timeout(duration)))
	}

	pub fn try_parse_fetch(&mut self) -> ParseResult<Option<Fetchs>> {
		if !self.eat(t!("FETCH")) {
			return Ok(None);
		}
		let v = self.parse_idiom_list()?.into_iter().map(Fetch).collect();
		Ok(Some(Fetchs(v)))
	}

	pub fn try_parse_condition(&mut self) -> ParseResult<Option<Cond>> {
		if !self.eat(t!("WHERE")) {
			return Ok(None);
		}
		let v = self.parse_value()?;
		Ok(Some(Cond(v)))
	}

	pub fn try_parse_group(&mut self) -> ParseResult<Option<Groups>> {
		if !self.eat(t!("GROUP")) {
			return Ok(None);
		}

		let res = match self.peek_kind() {
			t!("ALL") => {
				self.pop_peek();
				Groups(Vec::new())
			}
			t!("BY") => {
				self.pop_peek();
				let mut groups = Groups(vec![Group(self.parse_basic_idiom()?)]);
				while self.eat(t!(",")) {
					groups.0.push(Group(self.parse_basic_idiom()?));
				}
				groups
			}
			_ => {
				let mut groups = Groups(vec![Group(self.parse_basic_idiom()?)]);
				while self.eat(t!(",")) {
					groups.0.push(Group(self.parse_basic_idiom()?));
				}
				groups
			}
		};

		Ok(Some(res))
	}

	/// Parse a permissions production
	///
	/// # Parser State
	/// Expects the parser to have just eaten the `PERMISSIONS` keyword.
	pub fn parse_permission(&mut self) -> ParseResult<Permissions> {
		match self.next().kind {
			t!("NONE") => Ok(Permissions::none()),
			t!("FULL") => Ok(Permissions::full()),
			t!("FOR") => {
				let mut permission = Permissions::default();
				self.parse_specific_permission(&mut permission)?;
				while self.eat(t!(",")) {
					self.parse_specific_permission(&mut permission)?;
				}
				Ok(permission)
			}
			x => unexpected!(self, x, "'NONE', 'FULL' or 'FOR'"),
		}
	}

	/// Parse a specific permission for a type of query
	///
	/// Sets the permission for a specific query on the given permission keyword.
	///
	/// # Parser State
	/// Expects the parser to just have eaten the `FOR` keyword.
	pub fn parse_specific_permission(&mut self, permissions: &mut Permissions) -> ParseResult<()> {
		match self.next().kind {
			t!("SELECT") => {
				permissions.select = self.parse_permission_value()?;
			}
			t!("CREATE") => {
				permissions.create = self.parse_permission_value()?;
			}
			t!("UPDATE") => {
				permissions.update = self.parse_permission_value()?;
			}
			t!("DELETE") => {
				permissions.delete = self.parse_permission_value()?;
			}
			x => unexpected!(self, x, "'SELECT', 'CREATE', 'UPDATE' or 'DELETE'"),
		}
		Ok(())
	}

	/// Parses a the value for a permission for a type of query
	///
	/// # Parser State
	///
	/// Expects the parser to just have eaten either `SELECT`, `CREATE`, `UPDATE` or `DELETE`.
	pub fn parse_permission_value(&mut self) -> ParseResult<Permission> {
		match self.next().kind {
			t!("NONE") => Ok(Permission::None),
			t!("FULL") => Ok(Permission::Full),
			t!("WHERE") => Ok(Permission::Specific(self.parse_value()?)),
			x => unexpected!(self, x, "'NONE', 'FULL', or 'WHERE'"),
		}
	}

	/// Parses a base
	///
	/// So either `NAMESPACE`, ~DATABASE`, `ROOT`, or `SCOPE` if `scope_allowed` is true.
	///
	/// # Parser state
	/// Expects the next keyword to be a base.
	pub fn parse_base(&mut self, scope_allowed: bool) -> ParseResult<Base> {
		match self.next().kind {
			t!("NAMESPACE") => Ok(Base::Ns),
			t!("DATABASE") => Ok(Base::Db),
			t!("ROOT") => Ok(Base::Root),
			t!("SCOPE") => {
				if !scope_allowed {
					unexpected!(self, t!("SCOPE"), "a scope is not allowed here");
				}
				let name = self.parse_ident()?;
				Ok(Base::Sc(name))
			}
			x => {
				if scope_allowed {
					unexpected!(self, x, "'NAMEPSPACE', 'DATABASE', 'ROOT', 'SCOPE' or 'KV'")
				} else {
					unexpected!(self, x, "'NAMEPSPACE', 'DATABASE', 'ROOT', or 'KV'")
				}
			}
		}
	}

	/// Parses a changefeed production
	///
	/// # Parser State
	/// Expects the parser to have already eating the `CHANGEFEED` keyword
	pub fn parse_changefeed(&mut self) -> ParseResult<ChangeFeed> {
		let expiry = self.parse_duration()?.0;
		Ok(ChangeFeed {
			expiry,
		})
	}

	/// Parses a view production
	///
	/// # Parse State
	/// Expects the parser to have already eaten the possible `(` if the view was wrapped in
	/// parens. Expects the next keyword to be `SELECT`.
	pub fn parse_view(&mut self) -> ParseResult<View> {
		expected!(self, "SELECT");
		let fields = self.parse_fields()?;
		expected!(self, "FROM");
		let mut from = vec![Table(self.parse_raw_ident()?)];
		while self.eat(t!(",")) {
			from.push(Table(self.parse_raw_ident()?));
		}

		let cond = self.try_parse_condition()?;
		let group = self.try_parse_group()?;

		Ok(View {
			expr: fields,
			what: Tables(from),
			cond,
			group,
		})
	}

	pub fn try_parse_distance(&mut self) -> ParseResult<Option<Distance>> {
		if !self.eat(t!("DISTANCE")) {
			return Ok(None);
		}

		let dist = match self.next().kind {
			TokenKind::Distance(x) => match x {
				DistanceKind::Euclidean => Distance::Euclidean,
				DistanceKind::Manhattan => Distance::Manhattan,
				DistanceKind::Cosine => Distance::Cosine,
				DistanceKind::Hamming => Distance::Hamming,
				DistanceKind::Mahalanobis => Distance::Mahalanobis,
				DistanceKind::Minkowski => {
					let distance = self.parse_number()?;
					Distance::Minkowski(distance)
				}
			},
			x => unexpected!(self, x, "a distance measure"),
		};

		Ok(Some(dist))
	}

	pub fn parse_custom_function_name(&mut self) -> ParseResult<Ident> {
		expected!(self, "fn");
		expected!(self, "::");
		let mut name = self.parse_ident()?;
		while self.eat(t!("::")) {
			let part = self.parse_ident()?;
			name.0.push(':');
			name.0.push(':');
			name.0.push_str(part.as_str());
		}
		Ok(name)
	}
}
