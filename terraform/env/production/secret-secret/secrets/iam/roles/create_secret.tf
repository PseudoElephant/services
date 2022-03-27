resource "aws_iam_role" "create_secret_role" {
  name = "create-secret-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid    = ""
        Effect = "Allow"
        Action = "sts:AssumeRole"
        Principal = {
          Service = "lambda.amazonaws.com"
        }
      }
    ]
  })
}

module "policies" {
  source = "./policies"

  role = aws_iam_role.create_secret_role
}

output "create_secret_role" {
  value = aws_iam_role.create_secret_role
}
