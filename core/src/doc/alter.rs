use crate::ctx::{Context, MutableContext};
use crate::dbs::Options;
use crate::dbs::Statement;
use crate::dbs::Workable;
use crate::doc::Document;
use crate::err::Error;
use crate::sql::data::Data;
use crate::sql::operator::Operator;
use crate::sql::value::Value;
use reblessive::tree::Stk;

impl Document {
	pub async fn alter(
		&mut self,
		stk: &mut Stk,
		ctx: &Context,
		opt: &Options,
		stm: &Statement<'_>,
	) -> Result<(), Error> {
		// Get the record id
		let rid = self.id.as_ref().unwrap();
		// Set default field values
		self.current.doc.to_mut().def(rid);
		// The statement has a data clause
		if let Some(v) = stm.data() {
			match v {
				Data::PatchExpression(data) => {
					let data = data.compute(stk, ctx, opt, Some(&self.current)).await?;
					self.current.doc.to_mut().patch(data)?
				}
				Data::MergeExpression(data) => {
					let data = data.compute(stk, ctx, opt, Some(&self.current)).await?;
					self.current.doc.to_mut().merge(data)?
				}
				Data::ReplaceExpression(data) => {
					let data = data.compute(stk, ctx, opt, Some(&self.current)).await?;
					self.current.doc.to_mut().replace(data)?
				}
				Data::ContentExpression(data) => {
					let data = data.compute(stk, ctx, opt, Some(&self.current)).await?;
					self.current.doc.to_mut().replace(data)?
				}
				Data::UnsetExpression(i) => {
					for i in i.iter() {
						self.current.doc.to_mut().del(stk, ctx, opt, i).await?
					}
				}
				Data::SetExpression(x) => {
					for x in x.iter() {
						let v = x.2.compute(stk, ctx, opt, Some(&self.current)).await?;
						match &x.1 {
							Operator::Equal => match v {
								Value::None => {
									self.current.doc.to_mut().del(stk, ctx, opt, &x.0).await?
								}
								_ => self.current.doc.to_mut().set(stk, ctx, opt, &x.0, v).await?,
							},
							Operator::Inc => {
								self.current.doc.to_mut().increment(stk, ctx, opt, &x.0, v).await?
							}
							Operator::Dec => {
								self.current.doc.to_mut().decrement(stk, ctx, opt, &x.0, v).await?
							}
							Operator::Ext => {
								self.current.doc.to_mut().extend(stk, ctx, opt, &x.0, v).await?
							}
							o => {
								return Err(fail!("Unexpected operator in SET clause: {o:?}"));
							}
						}
					}
				}
				Data::UpdateExpression(x) => {
					// Duplicate context
					let mut ctx = MutableContext::new(ctx);
					// Add insertable value
					if let Workable::Insert(value) = &self.extras {
						ctx.add_value("input", value.clone());
					}
					if let Workable::Relate(_, _, Some(value)) = &self.extras {
						ctx.add_value("input", value.clone());
					}
					// Freeze the context
					let ctx: Context = ctx.into();
					// Process ON DUPLICATE KEY clause
					for x in x.iter() {
						let v = x.2.compute(stk, &ctx, opt, Some(&self.current)).await?;
						match &x.1 {
							Operator::Equal => match v {
								Value::None => {
									self.current.doc.to_mut().del(stk, &ctx, opt, &x.0).await?
								}
								_ => self.current.doc.to_mut().set(stk, &ctx, opt, &x.0, v).await?,
							},
							Operator::Inc => {
								self.current.doc.to_mut().increment(stk, &ctx, opt, &x.0, v).await?
							}
							Operator::Dec => {
								self.current.doc.to_mut().decrement(stk, &ctx, opt, &x.0, v).await?
							}
							Operator::Ext => {
								self.current.doc.to_mut().extend(stk, &ctx, opt, &x.0, v).await?
							}
							o => {
								return Err(fail!("Unexpected operator in UPDATE clause: {o:?}"));
							}
						}
					}
				}
				x => return Err(fail!("Unexpected data clause type: {x:?}")),
			};
		};
		// Set default field values
		self.current.doc.to_mut().def(rid);
		// Carry on
		Ok(())
	}
}
