use anyhow::Result;
use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};

async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let result = mlb_table::run("hitting", "2024")?;
    //Ok(json!({ "message": format!("Hello, world!") }))
    Ok(json!({
        "statusCode": 200,
        "headers": {
            "Content-Type": "application/CSV"
        },
        "body": result,
    }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await
}
