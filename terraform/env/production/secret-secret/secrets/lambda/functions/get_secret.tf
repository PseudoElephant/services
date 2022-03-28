module "get_secret_lambda" {
  source = "./../../../../../../modules/lambda"

  project_name    = "secret-secret"
  function_name   = "get_secret_service"
  function_dir    = "get-secret"
  role            = var.iam_module.get_secret_role
  repository_url  = var.repository_url
  repository_name = var.repository_name

  region     = var.region
  account_id = var.account_id
}

output "get_secret_service" {
  value = module.get_secret_lambda.data
}
