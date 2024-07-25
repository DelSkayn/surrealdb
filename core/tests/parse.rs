use surrealdb_core::sql::thing;
use surrealdb_core::sql::value;
use surrealdb_core::sql::Thing;
use surrealdb_core::sql::Value;

#[allow(dead_code)]
pub trait Parse<T> {
	fn parse(val: &str) -> T;
}

impl Parse<Value> for Value {
	fn parse(val: &str) -> Value {
		value(val).unwrap()
	}
}

impl Parse<Thing> for Thing {
	fn parse(val: &str) -> Thing {
		thing(val).unwrap()
	}
}
