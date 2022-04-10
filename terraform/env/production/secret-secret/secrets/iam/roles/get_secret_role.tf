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

resource "aws_iam_policy" "get_secret_policy" {
  name        = "get-secret"
  description = "Policy for get secret"

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
        {
            "Effect": "Allow",
            "Action": [
              "dynamodb:Scan",
              "dynamodb:GetItem"
            ],  
            "Resource": "arn:aws:dynamodb:${var.region}:${var.account_id}:table/secrets-table"
        }
  ]
}
EOF
}

resource "aws_iam_role_policy_attachment" "get_secret_policy" {
  role       = aws_iam_role.get_secret_role.name
  policy_arn = aws_iam_policy.get_secret_policy.arn
}

module "get_secret_policies" {
  source = "./policies"

  role = aws_iam_role.get_secret_role
}

output "get_secret_role" {
  value = aws_iam_role.get_secret_role
}
