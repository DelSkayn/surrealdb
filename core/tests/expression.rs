mod helpers;
use helpers::Test;
use surrealdb_core::err::Error;

#[tokio::test]
async fn modulo() -> Result<(), Error> {
	Test::new("8 % 3").await?.expect_val("2")?;
	Ok(())
}
