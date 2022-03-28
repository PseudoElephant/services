/*
  METHOD: GET
  PATH: /secrets/{secret_id}
*/
module "post" {
  source = "./../../../../../../../modules/apigateway/http/base-method"

  description     = "Gets a secret"
  method          = "GET"
  path            = "/secrets/{secret_id}"
  integration_uri = var.lambda_module.get_secret_service.invoke_arn
  function_name   = var.lambda_module.get_secret_service.function_name
  http_api_id     = var.apigateway_module.configuration.http_api_id

  region     = var.region
  account_id = var.account_id
}


module "rate" {
  source = "./rate"

  apigateway_module = var.apigateway_module
  lambda_module     = var.lambda_module

  region     = var.region
  account_id = var.account_id
}
