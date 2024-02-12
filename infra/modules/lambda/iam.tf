
data "aws_iam_policy_document" "exec" {
  version = "2012-10-17"
  statement {
    actions = ["sts:AssumeRole"]
    effect  = "Allow"
    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
  }
}

resource "aws_iam_role" "exec" {
  name               = "${var.name}-exec"
  assume_role_policy = data.aws_iam_policy_document.exec.json
}

data "aws_iam_policy_document" "ssm_read_policy" {

  statement {
    actions = [
      "ssm:GetParameter",
      "ssm:GetParameters",
      "ssm:GetParametersByPath",
    ]
    effect    = "Allow"
    resources = ["*"]
  }
}

resource "aws_iam_policy" "ssm_policy" {
  name   = "${var.name}-ssm-read"
  policy = data.aws_iam_policy_document.ssm_read_policy.json
}

resource "aws_iam_role_policy_attachment" "ssm_policy" {
  role       = aws_iam_role.exec.name
  policy_arn = aws_iam_policy.ssm_policy.arn
}

data "aws_iam_policy" "basic_policy" {
  arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

resource "aws_iam_role_policy_attachment" "basic_policy" {
  role       = aws_iam_role.exec.name
  policy_arn = data.aws_iam_policy.basic_policy.arn
}

resource "aws_iam_role_policy_attachment" "additional_policy" {
  count      = length(var.additional_policy_arns)
  role       = aws_iam_role.exec.name
  policy_arn = var.additional_policy_arns[count.index]
}
