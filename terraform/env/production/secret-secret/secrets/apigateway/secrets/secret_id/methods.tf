module "rate" {
  source = "./rate"

  apigateway_module = var.apigateway_module
  lambda_module     = var.lambda_module

  region     = var.region
  account_id = var.account_id
}
