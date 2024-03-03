use aws_config::load_from_env;
use aws_sdk_dynamodb::{Client, model::AttributeValue};
use lambda_runtime::{Error as LambdaError, LambdaEvent, service_fn};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use simple_logger::SimpleLogger;

#[derive(Deserialize)]
struct Request {
    DukeStudenName: String,
}

#[derive(Serialize)]
struct Response {
    // response with the StudentAge, which is a integer
    StudentAge: i32,
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    SimpleLogger::new().with_utc_timestamps().init()?;
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: LambdaEvent<Request>) -> Result<Value, LambdaError> {
    let request = event.payload;
    let config = load_from_env().await;
    let client = Client::new(&config);

    let response = get_age(&client, request.DukeStudenName).await?;

    Ok(json!({ "Age": response }))
}

async fn get_age(client: &Client, DukeStudenName: String) -> Result<Response, LambdaError> {
    let table_name = "rust_IDS721";

    let resp = client.get_item()
        .table_name(table_name)
        .key("DukeStudenName", AttributeValue::S(DukeStudenName))
        .send()
        .await?;

    if let Some(item) = resp.item() {
        let StudentAge: i32 = item.get("age").and_then(|v| v.as_n().ok()).and_then(|n| n.parse().ok()).unwrap_or(0);
        // let math_score: i32 = item.get("math").and_then(|v| v.as_n().ok()).and_then(|n| n.parse().ok()).unwrap_or(0);
        // let cloud_score: i32 = item.get("cloud").and_then(|v| v.as_n().ok()).and_then(|n| n.parse().ok()).unwrap_or(0);
        // let average = (math_score + cloud_score) as f32 / 2.0;

        Ok(Response { StudentAge })
    } else {
        Err(LambdaError::from("No data found for the provided Student Name"))
    }
}
