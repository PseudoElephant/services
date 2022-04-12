use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_sdk_sqs::Client;
use cookie::Cookie;
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

fn handle_err(err: Error) -> Result<ApiGatewayProxyResponse, Error> {
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

fn create_cookie<'a>(value: String) -> Cookie<'a> {
    let cookie = Cookie::build("user_id", value)
        .path("/secrets")
        .secure(true)
        .http_only(true) // Two weeks
        .max_age(time::Duration::days(14))
        .finish();

    return cookie;
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

    let default = &http::HeaderValue::from_str("invalid=FFF").unwrap();
    // try to get cookie | could use get_all to get many cookies
    let user_cookies: &http::HeaderValue = req.event.headers.get("set-cookie").unwrap_or(default);

    let cookie: cookie::Cookie = cookie::Cookie::parse(user_cookies.to_str().unwrap_or(""))?;

    let user_id: Option<String> = if cookie.name() == "user_id" {
        Some(String::from(cookie.value()))
    } else {
        None
    };
    let payload: RequestPayload = serde_json::from_str(&data)?;

    let filled_secret = models::secrets::new(payload.message, user_id.clone());

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

    if user_id.is_none() {
        let cookie: String = create_cookie(filled_secret.user_id.unwrap().to_string()).to_string();
        headers.insert("set-cookie", cookie.parse().unwrap());
    }

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
    let response = process(request).await;

    match response {
        Ok(r) => Ok(r),
        Err(e) => handle_err(e),
    }
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
    async fn test_handler_success_w_cookie() {
        let mut headers = HeaderMap::new();
        headers.insert("set-cookie", "user_id=98981fb6-a128-459e-95c6-4d6d84f23bca; HttpOnly; Secure; Path=/secrets; Max-Age=1209600".parse().unwrap());
        let apigw = ApiGatewayProxyRequest {
            headers: headers,
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
        );

        assert_eq!(response.headers.get("set-cookie"), None)
    }

    #[tokio::test]
    async fn test_handler_success_n_cookie() {
        let headers = HeaderMap::new();
        let apigw = ApiGatewayProxyRequest {
            headers: headers,
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
        );

        assert_ne!(response.headers.get("set-cookie"), None)
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
        let err = result.err().unwrap();
        assert_eq!(
            err.to_string(),
            "EOF while parsing a value at line 1 column 0"
        );
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
        let err = result.err().unwrap();
        assert_eq!(
            err.to_string(),
            "missing field `message` at line 1 column 2"
        );
    }
}
