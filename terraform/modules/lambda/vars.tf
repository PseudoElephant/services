variable "function_name" {
  type = string
}

variable "project_name" {
  type = string
}

variable "function_dir" {
  type = string
}

variable "role" {

}

variable "region" {
  type = string
}

variable "account_id" {
  type = string
}

variable "log_retention_in_days" {
  type    = number
  default = 30
}

variable "log_level" {
  type    = string
  default = "info"
}

variable "repository_url" {
  type = string
}

variable "repository_name" {
  type = string
}

variable "language" {
  type    = string
  default = "rust"
}
