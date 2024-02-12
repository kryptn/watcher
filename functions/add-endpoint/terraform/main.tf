locals {
  function_name = "${var.name}-${var.environment}"

  key = "lambda/${var.name}-${var.function_version}/bootstrap.zip"
  # real_key = "s3://${var.artifact_bucket_name}${local.key}"

  dynamodb_policy_arn = "arn:aws:iam::${var.account_number}:policy/dynamodb-stream-read-write"
}


module "lambda" {
  source = "../../../infra/modules/lambda"
  name   = local.function_name

  artifact_bucket_name = var.artifact_bucket_name
  artifact_key         = local.key

  additional_policy_arns = [
    local.dynamodb_policy_arn,
  ]

  region = var.region
}


