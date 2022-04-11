resource "aws_apigatewayv2_api" "http_api" {
  name          = var.name
  description   = var.description
  protocol_type = "HTTP"
  cors_configuration {
    allow_origins = ["http://localhost:8080"]
    allow_methods = ["POST", "GET", "OPTIONS"]
    allow_headers = ["content-type"]
    max_age       = 30
  }
}

resource "aws_cloudwatch_log_group" "api_log_group" {
  name              = "/aws/api/http-api"
  retention_in_days = var.log_retention_in_days
}

resource "aws_apigatewayv2_stage" "stage" {
  api_id      = aws_apigatewayv2_api.http_api.id
  name        = var.stage_name
  auto_deploy = true

  access_log_settings {
    destination_arn = aws_cloudwatch_log_group.api_log_group.arn
    format = jsonencode(
      {
        requestId      = "$context.requestId",
        ip             = "$context.identity.sourceIp",
        requestTime    = "$context.requestTime",
        httpMethod     = "$context.httpMethod",
        routeKey       = "$context.routeKey",
        status         = "$context.status",
        protocol       = "$context.protocol",
        responseLength = "$context.responseLength",
        error          = "$context.integrationErrorMessage"
    })
  }
}


