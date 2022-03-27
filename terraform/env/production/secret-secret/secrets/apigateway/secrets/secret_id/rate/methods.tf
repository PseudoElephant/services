# resource "aws_apigatewayv2_integration" "rate_secret_lambda_integration" {
#   api_id           = var.apigateway_module.configuration.http_api_id
#   integration_type = "AWS_PROXY"

#   connection_type    = "INTERNET"
#   description        = "Rate a secret"
#   integration_method = "POST"
#   integration_uri    = var.lambda_module.rate_secret_service.function_name
# }

/*
  METHOD: POST
  PATH: /secrets/secret_id/rate
*/
# resource "aws_apigatewayv2_route" "rate_secret_route" {
#   api_id    = var.apigateway_module.configuration.http_api_id
#   route_key = "POST /secrets/{secret_id}/rate"

#   target = "integrations/${aws_apigatewayv2_integration.rate_secret_lambda_integration.id}"
# }
