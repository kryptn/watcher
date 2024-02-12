resource "aws_lambda_function" "fn" {
  function_name = var.name
  role          = aws_iam_role.exec.arn
  handler       = "bootstrap"

  s3_bucket = var.artifact_bucket_name
  s3_key    = var.artifact_key

  runtime = "provided.al2"

  architectures = var.architectures

  environment {
    variables = merge({
      RUST_LOG = "info"
    }, var.lambda_environment)
  }
}

resource "aws_cloudwatch_log_group" "lambda" {
  name              = "/aws/lambda/${aws_lambda_function.fn.function_name}"
  retention_in_days = 30

}
