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

# Configures API Gateway
module "api_gateway" {
  source      = "./apigateway"
  name        = var.name
  description = var.description
  stage_name  = var.stage_name
}
