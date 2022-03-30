module "role" {
  source = "./roles"

  region     = var.region
  account_id = var.account_id
}
