resource "aws_iam_role_policy_attachment" "basic_lambda_policy" {
  role       = var.role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

