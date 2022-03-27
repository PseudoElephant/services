data "terraform_remote_state" "api" {
  backend = "s3"

  config = {
    bucket = "pseudo-production-terraform-state"
    key    = "production/secret-secret/api/terraform.tfstate"
    region = var.region
  }
}

output "api" {
  value = data.terraform_remote_state.api.outputs
}
