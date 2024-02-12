variable "account_number" {
  type = string
}

variable "environment" {
  type    = string
  default = "dev"
}

variable "region" {
  type    = string
  default = "us-west-2"
}

variable "table_name" {
  type    = string
  default = "watcher"
}
