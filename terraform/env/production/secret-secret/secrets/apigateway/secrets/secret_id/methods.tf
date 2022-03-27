# resource "aws_apigatewayv2_integration" "get_secret_lambda_integration" {
#   api_id           = var.apigateway_module.configuration.http_api_id
#   integration_type = "AWS_PROXY"

#   connection_type    = "INTERNET"
#   description        = "Get a secret"
#   integration_method = "GET"
#   integration_uri    = var.lambda_module.get_secret_service.function_name
# }

/*
  METHOD: GET
  PATH: /secrets/secret_id
*/
# resource "aws_apigatewayv2_route" "get_secret_route" {
#   api_id    = var.apigateway_module.configuration.http_api_id
#   route_key = "GET /secrets/{secret_id}"

#   target = "integrations/${aws_apigatewayv2_integration.get_secret_lambda_integration.id}"
# }

# module "rate" {
#   source = "./rate"

#   apigateway_module = var.apigateway_module
#   lambda_module     = var.lambda_module
# }
