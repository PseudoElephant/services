resource "aws_iam_role" "get_secret_role" {
  name = "get-secret-role"

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

module "get_secret_policies" {
  source = "./policies"

  role = aws_iam_role.get_secret_role
}

output "get_secret_role" {
  value = aws_iam_role.get_secret_role
}
