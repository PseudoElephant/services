use lambda_sqs::SqsEvent;
use lambda_sqs::{handler_fn, Context, Error};
use models::secrets::Secret;

#[tokio::main]
async fn main() -> Result<(), Error> {
  simple_logger::SimpleLogger::new()
    .with_utc_timestamps()
    .init()
    .unwrap();

  lambda_sqs::run(handler_fn(handler)).await?;

  Ok(())
}

async fn handler(e: SqsEvent, _: Context) -> Result<(), Error> {
  let events: Vec<Secret> = e.into_t();

  log::info!("Events: {:?}", events);
  Ok(())
}
