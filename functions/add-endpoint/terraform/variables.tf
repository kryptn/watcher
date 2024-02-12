variable "account_number" {
  type = string

}

variable "region" {
  type    = string
  default = "us-west-2"
}

variable "environment" {
    type    = string
    default = "dev"
}

variable "name" {
  type    = string
  default = "add-endpoint"
}

variable "function_version" {
  type    = string
  default = "v0.1.0"
}

variable "artifact_bucket_name" {
  type    = string
  default = "watcher-artifacts-dev"
}

variable "table_name" {
    type    = string
    default = "watcher"
}