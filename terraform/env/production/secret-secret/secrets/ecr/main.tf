resource "aws_ecr_repository" "ecr_secret_secret" {
  name                 = "pseudo-secret-secret-lambdas-production"
  image_tag_mutability = "MUTABLE"
}
