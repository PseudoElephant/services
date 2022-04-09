use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_sdk_sqs::Client;
use http::header::HeaderMap;
use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};

/// SQS_REVIEW_TOPIC topic to send messages to review queue
const SQS_REVIEW_TOPIC: &str =
    "https://sqs.us-east-1.amazonaws.com/493379917756/review-secret-queue";

#[derive(Debug)]
struct Request {
    event: ApiGatewayProxyRequest,
    client: Client,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct RequestPayload {
    message: String,
    user_id: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::SimpleLogger::new()
        .with_utc_timestamps()
        .init()
        .unwrap();

    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

fn handle_err<T>(err: T) -> Result<ApiGatewayProxyResponse, Error>
where
    T: std::error::Error,
{
    log::info!("Data received: {} ", err);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let json_response = serde_json::json!({
        "message" : err.to_string()
    });
    let resp = ApiGatewayProxyResponse {
        status_code: 400,
        headers: headers,
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(json_response.to_string())),
        is_base64_encoded: Some(false),
    };

    Ok(resp)
}

async fn init(event: ApiGatewayProxyRequest) -> Request {
    let config = aws_config::load_from_env().await;

    let request = Request {
        event: event,
        client: Client::new(&config),
    };

    return request;
}

async fn process(req: Request) -> Result<ApiGatewayProxyResponse, Error> {
    let data = req.event.body.unwrap_or_default();

    // Log body
    log::info!("Data received: {} ", data);

    let unmarshal: Result<RequestPayload, serde_json::Error> = serde_json::from_str(&data);
    if unmarshal.is_err() {
        return handle_err(unmarshal.err().unwrap());
    }

    let payload: RequestPayload = unmarshal.unwrap();
    let filled_secret = models::secrets::new(payload.message, payload.user_id);

    let secret_json = serde_json::json!(filled_secret).to_string();

    log::info!("Received secret {:?}", filled_secret);

    req.client
        .send_message()
        .queue_url(SQS_REVIEW_TOPIC)
        .message_body(&secret_json)
        .send()
        .await?;

    log::info!("Message sento to {}", SQS_REVIEW_TOPIC);
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: headers,
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(secret_json)),
        is_base64_encoded: Some(false),
    };

    Ok(resp)
}
async fn handler(
    req: ApiGatewayProxyRequest,
    _: Context,
) -> Result<ApiGatewayProxyResponse, Error> {
    let request = init(req).await;
    return process(request).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqs::mock;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_init() {
        let apigw = &ApiGatewayProxyRequest {
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
            request_context:
                aws_lambda_events::event::apigw::ApiGatewayProxyRequestContext::default(),
            resource: None,
            stage_variables: HashMap::new(),
        };
        let req = init(apigw.clone()).await;

        assert_eq!(
            req.event.body,
            Some(
                serde_json::json!({
                    "message" : "Hello"
                })
                .to_string(),
            )
        );
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
            request_context:
                aws_lambda_events::event::apigw::ApiGatewayProxyRequestContext::default(),
            resource: None,
            stage_variables: HashMap::new(),
        };

        // SQS Mocks
        let mocks = [mock::MockFile {
            request_body: "{ \"message\": \"Hello World\" }",
            response_file: "send-message.xml",
        }];

        let sqs_client = mock::mock_client(&mocks);

        let req = Request {
            event: apigw,
            client: sqs_client,
        };

        let result = process(req).await;
        assert!(result.is_ok());
        let response: ApiGatewayProxyResponse = result.unwrap();
        assert_eq!(
            response.status_code.to_string(),
            http::StatusCode::OK.as_str()
        );

        assert_eq!(
            response.headers.get("Content-Type").unwrap(),
            "application/json"
        )
    }

    #[tokio::test]
    async fn test_handler_no_body() {
        let apigw = ApiGatewayProxyRequest {
            headers: HeaderMap::new(),
            body: None,
            http_method: http::Method::POST,
            is_base64_encoded: Some(false),
            multi_value_headers: HeaderMap::new(),
            multi_value_query_string_parameters: HashMap::new(),
            path_parameters: HashMap::new(),
            query_string_parameters: HashMap::new(),
            path: Some("/secrets".to_string()),
            request_context:
                aws_lambda_events::event::apigw::ApiGatewayProxyRequestContext::default(),
            resource: None,
            stage_variables: HashMap::new(),
        };

        // SQS Mocks
        let mocks = [mock::MockFile {
            request_body: "{ \"message\": \"Hello World\" }",
            response_file: "send-message.xml",
        }];

        let sqs_client = mock::mock_client(&mocks);

        let req = Request {
            event: apigw,
            client: sqs_client,
        };

        let result = process(req).await;
        assert!(result.is_ok());
        let response: ApiGatewayProxyResponse = result.unwrap();
        assert_eq!(
            response.status_code.to_string(),
            http::StatusCode::from_u16(400).unwrap().as_str()
        );

        assert_eq!(
            response.headers.get("Content-Type").unwrap(),
            "application/json"
        )
    }

    #[tokio::test]
    async fn test_handler_missing_message() {
        let apigw = ApiGatewayProxyRequest {
            headers: HeaderMap::new(),
            body: Some("{}".to_string()),
            http_method: http::Method::POST,
            is_base64_encoded: Some(false),
            multi_value_headers: HeaderMap::new(),
            multi_value_query_string_parameters: HashMap::new(),
            path_parameters: HashMap::new(),
            query_string_parameters: HashMap::new(),
            path: Some("/secrets".to_string()),
            request_context:
                aws_lambda_events::event::apigw::ApiGatewayProxyRequestContext::default(),
            resource: None,
            stage_variables: HashMap::new(),
        };

        // SQS Mocks
        let mocks = [mock::MockFile {
            request_body: "{ \"message\": \"Hello World\" }",
            response_file: "send-message.xml",
        }];

        let sqs_client = mock::mock_client(&mocks);

        let req = Request {
            event: apigw,
            client: sqs_client,
        };

        let result = process(req).await;
        assert!(result.is_ok());
        let response: ApiGatewayProxyResponse = result.unwrap();
        assert_eq!(
            response.status_code.to_string(),
            http::StatusCode::from_u16(400).unwrap().as_str()
        );
        assert_eq!(
            response.body.unwrap(),
            Body::Text(
                serde_json::json!({
                    "message" : "missing field `message` at line 1 column 2"
                })
                .to_string()
            )
        );
        assert_eq!(
            response.headers.get("Content-Type").unwrap(),
            "application/json"
        )
    }
}
