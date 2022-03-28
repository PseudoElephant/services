terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.8.0"
    }
  }
}

# TODO: have a special user for this and assume a role 
provider "aws" {
  profile = var.profile
  region  = var.region
}

module "ecr" {
  source = "./ecr"
}

module "lambda" {
  source = "./lambda"

  depends_on = [module.ecr]

  region          = var.region
  account_id      = var.account_id
  iam_module      = module.iam.data
  repository_url  = module.ecr.data.repository_url
  repository_name = module.ecr.data.name
}

module "iam" {
  source = "./iam"
}

module "dynamodb" {
  source = "./dynamodb"
}

module "sqs" {
  source = "./sqs"
}

module "dependencies" {
  source = "./dependencies"

  region = var.region
}

module "apigateway" {
  source = "./apigateway"

  depends_on = [module.lambda]

  apigateway_module = module.dependencies.api.apigateway
  lambda_module     = module.lambda.data
  region            = var.region
  account_id        = var.account_id
}
