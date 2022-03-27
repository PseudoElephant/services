module "secrets" {
  source = "./secrets"

  lambda_module     = var.lambda_module
  apigateway_module = var.apigateway_module
  region            = var.region
  account_id        = var.account_id
}
