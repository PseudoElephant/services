terraform {
  backend "s3" {
    bucket = "pseudo-production-terraform-state"
    key    = "production/secret-secret/api/secrets/terraform.tfstate"
    region = "us-east-1"

    dynamodb_table = "terraform-state-locking"
    encrypt        = true
  }
}
