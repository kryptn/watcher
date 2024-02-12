variable "name" {
  type = string
}

variable "region" {
  type = string
}

variable "artifact_bucket_name" {
  type = string
}

variable "artifact_key" {
  type = string
}

variable "lambda_environment" {
  type    = map(string)
  default = {}
}

variable "additional_policy_arns" {
  type    = list(string)
  default = []
}

variable "architectures" {
  type    = list(string)
  default = ["x86_64"]
}