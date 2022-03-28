module "rate_secret_lambda" {
  source = "./../../../../../../modules/lambda"

  project_name    = "secret-secret"
  function_name   = "rate_secret_service"
  function_dir    = "rate-secret"
  role            = var.iam_module.rate_secret_role
  repository_url  = var.repository_url
  repository_name = var.repository_name

  region     = var.region
  account_id = var.account_id
}

output "rate_secret_service" {
  value = module.rate_secret_lambda.data
}
