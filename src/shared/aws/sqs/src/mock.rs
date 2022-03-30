use aws_sdk_sqs as sqs;
use aws_sdk_sqs::Client;
use aws_smithy_client::test_connection::TestConnection;
use aws_types::Credentials;

/// A mock for a client is represented
pub struct MockFile<'a> {
  /// Request body that the client will send
  pub request_body: &'a str,
  /// Points to a file that conatains the response that the service should expects
  pub response_file: &'a str,
}

/// Returns a mock sqs client
pub fn mock_client(events: &[MockFile]) -> Client {
  let creds = Credentials::new(
    "ATESTCLIENT",
    "atestsecretkey",
    Some("atestsessiontoken".to_string()),
    None,
    "",
  );
  let conf = sqs::config::Builder::new()
    .credentials_provider(creds)
    .region(sqs::Region::new("us-east-1"))
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

  let client = sqs::Client::from_conf_conn(conf, conn);

  return client;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_mock_client() {
    let mocks = [MockFile {
      request_body: "{ \"message\": \"Hello World\" }",
      response_file: "send-message.xml",
    }];

    let client = mock_client(&mocks);

    let response = client
      .send_message()
      .queue_url("SOME_TOPIC_URL")
      .message_body("{ \"message\": \"Hello World\" }")
      .send()
      .await;

    let res: aws_sdk_sqs::output::SendMessageOutput = response.unwrap();

    assert_eq!(
      "5fea7756-0ea4-451a-a703-a558b933e274",
      res.message_id().unwrap()
    );
  }
}
