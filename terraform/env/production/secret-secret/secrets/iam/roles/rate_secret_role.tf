resource "aws_iam_role" "rate_secret_role" {
  name = "rate-secret-role"

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

module "rate_secret_policies" {
  source = "./policies"

  role = aws_iam_role.rate_secret_role
}

output "rate_secret_role" {
  value = aws_iam_role.rate_secret_role
}
