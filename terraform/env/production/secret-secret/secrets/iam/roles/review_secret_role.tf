resource "aws_iam_role" "review_secret_role" {
  name = "review-secret-role"

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

resource "aws_iam_policy" "review_secret_policy" {
  name        = "review-secret"
  description = "Policy for review secret"

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
           {
            "Effect": "Allow",
            "Action": [
                "sqs:ReceiveMessage",
                "sqs:DeleteMessage",
                "sqs:GetQueueAttributes"
            ],
            "Resource": "arn:aws:sqs:${var.region}:${var.account_id}:review-secret-queue"
        },
        {
            "Effect": "Allow",
            "Action": [
              "dynamodb:PutItem",
              "dynamodb:BatchWriteItem"
            ],
            "Resource": "arn:aws:dynamodb:${var.region}:${var.account_id}:table/secrets-table"
        }
  ]
}
EOF
}

resource "aws_iam_role_policy_attachment" "review_secret_policy" {
  role       = aws_iam_role.review_secret_role.name
  policy_arn = aws_iam_policy.review_secret_policy.arn
}


module "review_secret_policies" {
  source = "./policies"

  role = aws_iam_role.review_secret_role
}

output "review_secret_role" {
  value = aws_iam_role.review_secret_role
}
