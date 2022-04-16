module "review_secret_lambda" {
  source = "./../../../../../../modules/lambda"

  project_name    = "secret-secret"
  function_name   = "review_secret_service"
  function_dir    = "review-secret"
  role            = var.iam_module.review_secret_role
  repository_url  = var.repository_url
  repository_name = var.repository_name
  language        = "python"
  region          = var.region
  account_id      = var.account_id
}

resource "aws_lambda_event_source_mapping" "event_source_mapping" {
  batch_size       = 10
  event_source_arn = var.sqs_module.review_secret.arn
  enabled          = true
  function_name    = module.review_secret_lambda.data.function_name
}

output "review_secret_service" {
  value = module.review_secret_lambda.data
}
