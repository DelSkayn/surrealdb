use crate::{api::context::InvocationContext, expr::Value, fnc::args};
use anyhow::Result;

use super::api;

pub trait InvokeMiddleware<'a> {
	fn invoke(self, context: &'a mut InvocationContext) -> Result<()>;
}

macro_rules! dispatch {
	($name: ident, $args: expr_2021, $context: expr_2021, $($function_name: literal => $(($wrapper: tt))* $($function_path: ident)::+,)+) => {
		{
			match $name {
				$($function_name => {
					let args = args::FromArgs::from_args($name, $args)?;
					#[expect(clippy::redundant_closure_call)]
					$($wrapper)*(|| $($function_path)::+($context, args))()
				},)+
				_ => {
					Err(::anyhow::Error::new($crate::err::Error::InvalidFunction{
						name: String::from($name),
						message: "unknown middleware".to_string()
					}))
				}
			}
		}
	};
}

impl<'a> InvokeMiddleware<'a> for (&'a String, &'a Vec<Value>) {
	fn invoke(self, context: &'a mut InvocationContext) -> Result<()> {
		let name = self.0.as_str();

		dispatch!(
			name,
			self.1.to_owned(),
			context,
			//
			"api::req::max_body" => api::req::max_body,
			"api::req::raw_body" => api::req::raw_body,
			//
			"api::res::raw_body" => api::res::raw_body,
			"api::res::headers" => api::res::headers,
			"api::res::header" => api::res::header,
			//
			"api::timeout" => api::timeout,
		)
	}
}
