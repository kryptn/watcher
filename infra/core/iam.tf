



data "aws_iam_policy_document" "dynamodb_policy" {
  statement {
    actions = [
      "dynamodb:DescribeStream",
      "dynamodb:GetRecords",
      "dynamodb:GetShardIterator",
      "dynamodb:ListStreams",
      "dynamodb:GetItem",
      "dynamodb:PutItem",
      "dynamodb:DeleteItem",
      "dynamodb:Query",
    ]
    effect    = "Allow"
    resources = ["*"]
  }
}

resource "aws_iam_policy" "dynamodb_policy" {
  name   = "dynamodb-stream-read-write"
  policy = data.aws_iam_policy_document.dynamodb_policy.json
}

output "dynamodb_policy_arn" {
  value = aws_iam_policy.dynamodb_policy.arn
}
