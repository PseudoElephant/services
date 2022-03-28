/*
  METHOD: POST
  PATH: /secrets/{secret_id}/rate
*/
module "post" {
  source = "./../../../../../../../../modules/apigateway/http/base-method"

  description     = "Rate a secret"
  method          = "POST"
  path            = "/secrets/{secret_id}/rate"
  integration_uri = var.lambda_module.rate_secret_service.invoke_arn
  function_name   = var.lambda_module.rate_secret_service.function_name
  http_api_id     = var.apigateway_module.configuration.http_api_id

  region     = var.region
  account_id = var.account_id
}
