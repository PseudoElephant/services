/*
  METHOD: POST
  PATH: /secrets
*/
module "post" {
  source = "./../../../../../../modules/apigateway/http/base-method"

  description     = "Creates a secret"
  method          = "POST"
  path            = "/secrets"
  integration_uri = var.lambda_module.create_secret_service.invoke_arn
  function_name   = var.lambda_module.create_secret_service.function_name
  http_api_id     = var.apigateway_module.configuration.http_api_id

  region     = var.region
  account_id = var.account_id
}

module "secret_id" {
  source = "./secret_id"

  apigateway_module = var.apigateway_module
  lambda_module     = var.lambda_module

  region     = var.region
  account_id = var.account_id
}
