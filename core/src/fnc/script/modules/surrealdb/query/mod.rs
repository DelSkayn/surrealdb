use std::cell::RefCell;

use js::{
	class::{ClassId, JsClass, OwnedBorrow, Readable, Trace},
	prelude::{Coerced, Opt},
	Ctx, Exception, FromJs, Promise, Result, Value,
};
use reblessive::tree::Stk;

use crate::{
	ctx::Context,
	dbs::{Attach, Options},
	doc::CursorDoc,
};

#[allow(clippy::module_inception)]
mod classes;

use crate::ctx::MutableContext;
pub use classes::Query;

pub const QUERY_DATA_PROP_NAME: &str = "__query_context__";

/// A class to carry the data to run subqueries.
#[non_exhaustive]
#[derive(Clone)]
pub struct QueryContext<'js> {
	pub context: &'js Context,
	pub opt: &'js Options,
	pub doc: Option<&'js CursorDoc>,
	pub pending: RefCell<Option<Promise<'js>>>,
}

impl<'js> Trace<'js> for QueryContext<'js> {
	fn trace<'a>(&self, _tracer: js::class::Tracer<'a, 'js>) {}
}

impl<'js> JsClass<'js> for QueryContext<'js> {
	const NAME: &'static str = "QueryContext";

	type Mutable = Readable;

	fn class_id() -> &'static js::class::ClassId {
		static ID: ClassId = ClassId::new();
		&ID
	}

	fn prototype(_ctx: &js::Ctx<'js>) -> Result<Option<js::Object<'js>>> {
		Ok(None)
	}

	fn constructor(_ctx: &js::Ctx<'js>) -> Result<Option<js::function::Constructor<'js>>> {
		Ok(None)
	}
}

#[js::function]
pub fn query<'js>(
	ctx: Ctx<'js>,
	query: Value<'js>,
	variables: Opt<classes::QueryVariables>,
) -> Result<Promise<'js>> {
	let ctx_clone = ctx.clone();
	let query_ctx =
		ctx.globals().get::<_, OwnedBorrow<'js, QueryContext<'js>>>(QUERY_DATA_PROP_NAME)?;

	let mut pending_borrow = query_ctx.pending.borrow_mut();
	let pending_query_future = pending_borrow.as_ref().map(|x| x.clone().into_future::<()>());

	let promise = Promise::wrap_future(&ctx_clone, async move {
		// Wait on existing query ctx so that we can't spawn more then one query at the same time.
		if let Some(x) = pending_query_future {
			let _ = x.await;
		}

		let query_ctx =
			ctx.globals().get::<_, OwnedBorrow<'js, QueryContext<'js>>>(QUERY_DATA_PROP_NAME)?;

		let mut borrow_store = None;
		let mut query_store = None;

		let res = async {
			let query = if query.is_object() {
				let borrow = OwnedBorrow::<Query>::from_js(&ctx, query)?;
				&**borrow_store.insert(borrow)
			} else {
				let Coerced(query_text) = Coerced::<String>::from_js(&ctx, query)?;
				query_store.insert(Query::new(ctx.clone(), query_text, variables)?)
			};

			let mut context = MutableContext::new(query_ctx.context);
			query
				.clone()
				.vars
				.attach(&mut context)
				.map_err(|e| Exception::throw_message(&ctx, &e.to_string()))?;
			let context = context.freeze();

			Stk::enter_scope(|stk| {
				stk.run(|stk| query.query.compute(stk, &context, query_ctx.opt, query_ctx.doc))
			})
			.await
			.map_err(|e| Exception::throw_message(&ctx, &e.to_string()))
		}
		.await;

		*query_ctx.pending.borrow_mut() = None;

		res
	})?;

	*pending_borrow = Some(promise.clone());

	Ok(promise)
}
