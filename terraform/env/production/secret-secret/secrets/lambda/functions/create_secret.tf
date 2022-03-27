locals {
  root_dir    = "./../../../../../"
  lambda_name = "secrets_create-secret"
}

# CREATES ECR AND DEPLOYS IMAGE
resource "aws_ecr_repository" "lambda_repository" {
  name = "${local.lambda_name}-production"
}

resource "null_resource" "lambda_ecr_image_builder" {
  triggers = {
    docker_file     = filesha256("${local.root_dir}/Dockerfile")
    cargo_file      = filesha256("${local.root_dir}/Cargo.toml")
    cargo_lock_file = filesha256("${local.root_dir}/Cargo.lock")
    src_dir         = sha256(join("", [for f in fileset("${local.root_dir}/src/secret-secret/functions/create-secret", "**") : filesha256("${local.root_dir}/src/secret-secret/functions/create-secret/${f}")]))
  }

  provisioner "local-exec" {
    working_dir = local.root_dir
    interpreter = ["/bin/bash", "-c"]
    command     = <<-EOT
      aws ecr get-login-password --region ${var.region} | docker login --username AWS --password-stdin ${var.account_id}.dkr.ecr.${var.region}.amazonaws.com
      docker image build -t ${aws_ecr_repository.lambda_repository.repository_url}:latest --build-arg binary=create-secret --build-arg log_level=${var.log_level} .
      docker push ${aws_ecr_repository.lambda_repository.repository_url}:latest
    EOT
  }
}

data "aws_ecr_image" "lambda_image" {
  depends_on = [
    null_resource.lambda_ecr_image_builder
  ]

  repository_name = "secrets_create-secret-production"
  image_tag       = "latest"
}

resource "aws_lambda_function" "create_secret" {
  function_name = local.lambda_name
  package_type  = "Image"
  role          = var.iam_module.create_secret_role.arn
  image_uri     = "${aws_ecr_repository.lambda_repository.repository_url}@${data.aws_ecr_image.lambda_image.id}"
}

resource "aws_cloudwatch_log_group" "lambda_log_group" {
  name              = "/aws/lambda/${local.lambda_name}"
  retention_in_days = var.log_retention_in_days
}

output "create_secret_service" {
  value = aws_lambda_function.create_secret
}
