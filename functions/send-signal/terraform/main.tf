data "aws_ssm_parameter" "version" {
  name = "/${var.environment}/function/${local.function_name}/version"
}

locals {
  function_name = "${var.name}-${var.environment}"

  key = "lambda/${var.name}-${nonsensitive(data.aws_ssm_parameter.version.value)}/bootstrap.zip"
  # real_key = "s3://${var.artifact_bucket_name}${local.key}"

  dynamodb_policy_arn = "arn:aws:iam::${var.account_number}:policy/dynamodb-stream-read-write"
  sqs_policy_arn      = "arn:aws:iam::${var.account_number}:policy/sqs-watcher-events-read-write"

  queue_arn = "arn:aws:sqs:us-west-2:${var.account_number}:${var.sqs_queue_name}"
}

module "lambda" {
  source = "../../../infra/modules/lambda"
  name   = local.function_name

  artifact_bucket_name = var.artifact_bucket_name
  artifact_key         = local.key

  architectures = ["arm64"]

  additional_policy_arns = [
    local.dynamodb_policy_arn,
  ]

  region = var.region
}


resource "aws_iam_role_policy_attachment" "dynamodb" {
  policy_arn = local.dynamodb_policy_arn
  role       = module.lambda.exec_role_name
}

resource "aws_iam_role_policy_attachment" "sqs" {
  policy_arn = local.sqs_policy_arn
  role       = module.lambda.exec_role_name
}


resource "aws_lambda_event_source_mapping" "send_signal" {
  event_source_arn = local.queue_arn
  function_name    = local.function_name
  batch_size       = 1

  filter_criteria {
    filter {
      pattern = jsonencode({
        body = {
          event_type : ["sink_signal_created"]
        }
      })
    }
  }

  depends_on = [module.lambda]
}


output "lambda_arn" {
  value = module.lambda.lambda_arn
}

output "lambda_invoke_arn" {
  value = module.lambda.lambda_invoke_arn
}

output "lambda_name" {
  value = module.lambda.lambda_name
}
