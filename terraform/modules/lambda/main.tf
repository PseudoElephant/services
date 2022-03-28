locals {
  root_dir = "./../../../../../"
}


resource "null_resource" "lambda_ecr_image_builder" {
  triggers = {
    docker_file     = filesha256("${local.root_dir}/Dockerfile")
    cargo_file      = filesha256("${local.root_dir}/Cargo.toml")
    cargo_lock_file = filesha256("${local.root_dir}/Cargo.lock")
    src_dir         = sha256(join("", [for f in fileset("${local.root_dir}/src/${var.project_name}/functions/${var.function_dir}", "**") : filesha256("${local.root_dir}/src/${var.project_name}/functions/${var.function_dir}/${f}")]))
  }

  provisioner "local-exec" {
    working_dir = local.root_dir
    interpreter = ["/bin/bash", "-c"]
    command     = <<-EOT
      aws ecr get-login-password --region ${var.region} | docker login --username AWS --password-stdin ${var.account_id}.dkr.ecr.${var.region}.amazonaws.com
      docker image build -t ${var.repository_url}:${var.function_name} --build-arg binary=${var.function_dir} --build-arg log_level=${var.log_level} .
      docker push ${var.repository_url}:${var.function_name}
    EOT
  }
}

data "aws_ecr_image" "lambda_image" {
  depends_on = [
    null_resource.lambda_ecr_image_builder
  ]

  repository_name = var.repository_name
  image_tag       = var.function_name
}

# Lambda function configuration
resource "aws_lambda_function" "lambda_function" {
  function_name = var.function_name
  package_type  = "Image"
  role          = var.role.arn
  image_uri     = "${var.repository_url}@${data.aws_ecr_image.lambda_image.id}"
}

resource "aws_cloudwatch_log_group" "lambda_log_group" {
  name              = "/aws/lambda/${var.function_name}"
  retention_in_days = var.log_retention_in_days
}

