resource "aws_apigatewayv2_integration" "create_secret_lambda_integration" {
  api_id           = var.apigateway_module.configuration.http_api_id
  integration_type = "AWS_PROXY"

  connection_type    = "INTERNET"
  description        = "Creates a secret"
  integration_method = "POST"
  integration_uri    = var.lambda_module.create_secret_service.invoke_arn
}

/*
  METHOD: POST
  PATH: /secrets
*/
resource "aws_apigatewayv2_route" "create_secret_route" {
  api_id    = var.apigateway_module.configuration.http_api_id
  route_key = "POST /secrets"

  target = "integrations/${aws_apigatewayv2_integration.create_secret_lambda_integration.id}"
}

resource "aws_lambda_permission" "allow_api_gateway" {
  function_name = var.lambda_module.create_secret_service.function_name
  action        = "lambda:InvokeFunction"
  principal     = "apigateway.amazonaws.com"
  source_arn    = "arn:aws:execute-api:${var.region}:${var.account_id}:${var.apigateway_module.configuration.http_api_id}/*/POST/secrets"
}


module "secret_id" {
  source = "./secret_id"

  apigateway_module = var.apigateway_module
  lambda_module     = var.lambda_module
}
