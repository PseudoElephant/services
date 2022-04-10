use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_sdk_dynamodb::Client;

use futures;
use http::header::HeaderMap;
use http::StatusCode;
use lambda_runtime::{handler_fn, Context, Error};
use once_cell::sync::OnceCell;
use rand::Rng;

const SECRET_TABLE_NAME: &str = "secrets-table";

const ERR_SECRET_NOT_FOUND: &str = "no secret found";

struct MockClientData {
  config: aws_sdk_dynamodb::Config,
  conn: aws_smithy_client::test_connection::TestConnection<aws_smithy_http::body::SdkBody>,
}

fn dynamo_client(mock: Option<MockClientData>) -> &'static Client {
  static INSTANCE: OnceCell<Client> = OnceCell::new();
  INSTANCE.get_or_init(|| {
    let client = futures::executor::block_on(async {
      match mock {
        Some(mock_data) => return Client::from_conf_conn(mock_data.config, mock_data.conn),
        None => {
          let conf = aws_config::load_from_env().await;
          return Client::new(&conf);
        }
      }
    });
    client
  })
}

fn error_response(code: StatusCode, err: &str) -> ApiGatewayProxyResponse {
  let mut headers = HeaderMap::new();
  headers.insert("Content-Type", "application/json".parse().unwrap());

  let json_data = serde_json::json!({ "message": err }).to_string();
  return ApiGatewayProxyResponse {
    status_code: i64::from(code.as_u16()),
    headers: headers,
    multi_value_headers: HeaderMap::new(),
    body: Some(Body::Text(json_data)),
    is_base64_encoded: Some(false),
  };
}

fn response(code: StatusCode, secret: models::secrets::Secret) -> ApiGatewayProxyResponse {
  let mut headers = HeaderMap::new();
  headers.insert("Content-Type", "application/json".parse().unwrap());

  let json_data = serde_json::json!(secret).to_string();
  return ApiGatewayProxyResponse {
    status_code: i64::from(code.as_u16()),
    headers: headers,
    multi_value_headers: HeaderMap::new(),
    body: Some(Body::Text(json_data)),
    is_base64_encoded: Some(false),
  };
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  simple_logger::SimpleLogger::new()
    .with_utc_timestamps()
    .init()
    .unwrap();
  // init db client
  dynamo_client(None);

  let func = handler_fn(handler);
  lambda_runtime::run(func).await?;
  Ok(())
}

async fn handler(_: ApiGatewayProxyRequest, _: Context) -> Result<ApiGatewayProxyResponse, Error> {
  let items_count_data: aws_sdk_dynamodb::output::ScanOutput = dynamo_client(None)
    .scan()
    .table_name(SECRET_TABLE_NAME)
    .attributes_to_get("secret_id")
    .select(aws_sdk_dynamodb::model::Select::SpecificAttributes)
    .send()
    .await?;

  if items_count_data.count == 0 {
    return Ok(error_response(StatusCode::NOT_FOUND, ERR_SECRET_NOT_FOUND));
  }
  let max_bound: usize = (items_count_data.count) as usize;
  let query_index: usize = rand::thread_rng().gen_range(0..max_bound);

  let items = items_count_data.items().unwrap();
  let item_id: &aws_sdk_dynamodb::model::AttributeValue =
    items.get(query_index).unwrap().get("secret_id").unwrap();
  let item_query: aws_sdk_dynamodb::output::GetItemOutput = dynamo_client(None)
    .get_item()
    .table_name(SECRET_TABLE_NAME)
    .key("secret_id", item_id.clone())
    .send()
    .await?;

  let item_db = item_query.item().unwrap().clone();
  let secret: models::secrets::Secret = serde_dynamo::from_item(item_db)?;
  log::info!("Received secret {:?}", secret);
  Ok(response(http::StatusCode::OK, secret))
}

#[cfg(test)]
mod tests {
  use super::*;
  use aws_smithy_client::test_connection::TestConnection;
  use aws_types::Credentials;
  use std::collections::HashMap;
  /// A mock for a client is represented
  struct MockFile<'a> {
    /// Request body that the client will send
    request_body: &'a str,
    /// Points to a file that conatains the response that the service should expects
    response_file: &'a str,
  }

  /// Returns a mock sqs client
  fn mock_client(events: &[MockFile]) -> MockClientData {
    let creds = Credentials::new(
      "ATESTCLIENT",
      "atestsecretkey",
      Some("atestsessiontoken".to_string()),
      None,
      "",
    );
    let conf = aws_sdk_dynamodb::config::Builder::new()
      .credentials_provider(creds)
      .region(aws_sdk_dynamodb::Region::new("us-east-1"))
      .build();

    let conn_events = events
      .iter()
      .map(|d| {
        let path = std::path::Path::new("test-data").join(d.response_file);
        let data = std::fs::read_to_string(path).unwrap();

        // Events
        (
          // Request
          http::Request::builder()
            .body(aws_smithy_http::body::SdkBody::from(d.request_body))
            .unwrap(),
          // Response
          http::Response::builder()
            .status(200)
            .body(aws_smithy_http::body::SdkBody::from(data))
            .unwrap(),
        )
      })
      .collect();
    let conn = TestConnection::new(conn_events);

    return MockClientData {
      config: conf,
      conn: conn,
    };
  }

  #[tokio::test]
  async fn test_handler_success() {
    let apigw = ApiGatewayProxyRequest {
      headers: HeaderMap::new(),
      body: Some(
        serde_json::json!({
            "message" : "Hello"
        })
        .to_string(),
      ),
      http_method: http::Method::POST,
      is_base64_encoded: Some(false),
      multi_value_headers: HeaderMap::new(),
      multi_value_query_string_parameters: HashMap::new(),
      path_parameters: HashMap::new(),
      query_string_parameters: HashMap::new(),
      path: Some("/secrets".to_string()),
      request_context: aws_lambda_events::event::apigw::ApiGatewayProxyRequestContext::default(),
      resource: None,
      stage_variables: HashMap::new(),
    };
    // SQS Mocks
    let mocks = [
      MockFile {
        request_body: "{
        \"TableName\": \"secrets-table\",
        \"AttributesToGet\": [ \"secret_id\" ],
        \"Select\": \"SPECIFIC_ATTRIBUTES\"
      }",
        response_file: "scan.json",
      },
      MockFile {
        request_body: "{
        \"TableName\": \"secrets-table\",
        \"Key\": { \"secret_id\" : { \"S\" : \"another-id\" } }
      }",
        response_file: "get_item.json",
      },
    ];

    let mock = mock_client(&mocks);

    dynamo_client(Some(mock));
    let result = handler(apigw, lambda_runtime::Context::default()).await;
    assert!(result.is_ok());
    let response: ApiGatewayProxyResponse = result.unwrap();
    assert_eq!(
      response.status_code.to_string(),
      http::StatusCode::OK.as_str()
    );

    let body = response.body.unwrap();
    assert_ne!(body, Body::Text(serde_json::json!({}).to_string()));

    assert_eq!(
      response.headers.get("Content-Type").unwrap(),
      "application/json"
    );

    assert_eq!(
      body,
      Body::Text(
        serde_json::json!({
            "secret_id" : "94cc0092-d6c2-412f-9f51-0e40079d1bb3",
            "message": "some message",
            "user_id": "00000000-0000-0000-0000-000000000000",
            "likes": 0,
            "dislikes": 0
        })
        .to_string()
      )
    )
  }
}
