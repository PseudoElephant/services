variable "name" {
  type = string
}

variable "description" {
  type = string
}

variable "stage_name" {
  type = string
}

variable "log_retention_in_days" {
  type    = number
  default = 30
}
