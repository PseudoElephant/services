module "create_secret_lambda" {
  source = "./../../../../../../modules/lambda"

  project_name    = "secret-secret"
  function_name   = "create_secret_service"
  function_dir    = "create-secret"
  role            = var.iam_module.create_secret_role
  repository_url  = var.repository_url
  repository_name = var.repository_name

  region     = var.region
  account_id = var.account_id
}

output "create_secret_service" {
  value = module.create_secret_lambda.data
}
