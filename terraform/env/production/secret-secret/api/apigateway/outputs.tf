
output "configuration" {
  value = {
    http_api_id = aws_apigatewayv2_api.http_api.id
  }
}
