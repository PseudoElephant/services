resource "aws_dynamodb_table" "secrets_table" {
  name         = "secrets-table"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "secret_id"

  attribute {
    name = "secret_id"
    type = "S"
  }

}
