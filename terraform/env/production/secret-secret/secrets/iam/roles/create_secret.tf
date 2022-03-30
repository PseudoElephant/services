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

module "create_secret_policies" {
  source = "./policies"

  role = aws_iam_role.create_secret_role
}

resource "aws_iam_policy" "create_secret_policy" {
  name        = "create-secret"
  description = "Policy for create secret"

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
           {
            "Effect": "Allow",
            "Action": [
                "sqs:SendMessage"
            ],
            "Resource": "arn:aws:sqs:${var.region}:${var.account_id}:review-secret-queue"
        }
  ]
}
EOF
}

resource "aws_iam_role_policy_attachment" "create_secret_policy" {
  role       = aws_iam_role.create_secret_role.name
  policy_arn = aws_iam_policy.create_secret_policy.arn
}


output "create_secret_role" {
  value = aws_iam_role.create_secret_role
}
