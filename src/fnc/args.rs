use crate::dbs::Runtime;
use crate::err::Error;
use crate::sql::value::Value;

pub enum Args {
	None,
	Any,
	One,
	Two,
	Three,
	NoneOne,
	NoneTwo,
	NoneOneTwo,
	OneTwo,
	OneTwoThree,
}

pub fn check(
	ctx: &Runtime,
	name: &String,
	args: Vec<Value>,
	size: Args,
	func: fn(&Runtime, Vec<Value>) -> Result<Value, Error>,
) -> Result<Value, Error> {
	match size {
		Args::None => match args.len() {
			0 => func(ctx, args),
			_ => Err(Error::ArgumentsError {
				name: name.to_owned(),
				message: String::from("The function does not expect any arguments."),
			}),
		},
		Args::One => match args.len() {
			1 => func(ctx, args),
			_ => Err(Error::ArgumentsError {
				name: name.to_owned(),
				message: String::from("The function expects 1 argument."),
			}),
		},
		Args::Two => match args.len() {
			2 => func(ctx, args),
			_ => Err(Error::ArgumentsError {
				name: name.to_owned(),
				message: String::from("The function expects 2 arguments."),
			}),
		},
		Args::Three => match args.len() {
			3 => func(ctx, args),
			_ => Err(Error::ArgumentsError {
				name: name.to_owned(),
				message: String::from("The function expects 3 arguments."),
			}),
		},
		Args::NoneOne => match args.len() {
			0 | 1 => func(ctx, args),
			_ => Err(Error::ArgumentsError {
				name: name.to_owned(),
				message: String::from("The function expects 0 or 1 arguments."),
			}),
		},
		Args::NoneTwo => match args.len() {
			0 | 2 => func(ctx, args),
			_ => Err(Error::ArgumentsError {
				name: name.to_owned(),
				message: String::from("The function expects 0 or 2 arguments."),
			}),
		},
		Args::NoneOneTwo => match args.len() {
			0 | 1 | 2 => func(ctx, args),
			_ => Err(Error::ArgumentsError {
				name: name.to_owned(),
				message: String::from("The function expects 0, 1, or 2 arguments."),
			}),
		},
		Args::OneTwo => match args.len() {
			1 | 2 => func(ctx, args),
			_ => Err(Error::ArgumentsError {
				name: name.to_owned(),
				message: String::from("The function expects 1 or 2 arguments."),
			}),
		},
		Args::OneTwoThree => match args.len() {
			1 | 2 | 3 => func(ctx, args),
			_ => Err(Error::ArgumentsError {
				name: name.to_owned(),
				message: String::from("The function expects 1, 2, or 3 arguments."),
			}),
		},
		Args::Any => func(ctx, args),
	}
}
