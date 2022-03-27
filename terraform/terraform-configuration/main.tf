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

module "s3" {
  source = "./s3"
}

module "dynamodb" {
  source = "./dynamodb"
}
