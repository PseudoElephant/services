module "functions" {
  source = "./functions"

  region          = var.region
  account_id      = var.account_id
  iam_module      = var.iam_module
  repository_url  = var.repository_url
  repository_name = var.repository_name
}
