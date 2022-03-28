resource "aws_apigatewayv2_integration" "lambda_integration" {
  api_id           = var.http_api_id
  integration_type = "AWS_PROXY"

  connection_type    = "INTERNET"
  description        = var.description
  integration_method = "POST"
  integration_uri    = var.integration_uri
}

resource "aws_apigatewayv2_route" "api_route" {
  api_id    = var.http_api_id
  route_key = "${var.method} ${var.path}"

  target = "integrations/${aws_apigatewayv2_integration.lambda_integration.id}"
}

resource "aws_lambda_permission" "allow_api_gateway" {
  function_name = var.function_name
  action        = "lambda:InvokeFunction"
  principal     = "apigateway.amazonaws.com"
  source_arn    = "arn:aws:execute-api:${var.region}:${var.account_id}:${var.http_api_id}/*/${var.method}${var.path}"
}
