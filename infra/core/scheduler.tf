
resource "aws_scheduler_schedule_group" "observation_schedule_group" {
  name = "observation-schedule-group"
}


data "aws_iam_policy_document" "eventbridge_exec_policy" {
  version = "2012-10-17"

  statement {
    actions = ["sts:AssumeRole"]
    effect  = "Allow"
    principals {
      type        = "Service"
      identifiers = ["scheduler.amazonaws.com"]
    }
    # condition {
    #   test     = "StringEquals"
    #   variable = "aws:SourceAccount"
    #   values   = ["${var.account_number}"]
    # }
    # condition {
    #   test     = "StringEquals"
    #   variable = "aws:SourceArn"
    #   values   = ["arn:aws:scheduler:${var.region}:${var.account_number}:schedule-group/${aws_scheduler_schedule_group.observation_schedule_group.name}"]
    # }
  }
}


resource "aws_iam_role" "eventbridge_exec_role" {
  name               = "eventbridge-exec-role"
  assume_role_policy = data.aws_iam_policy_document.eventbridge_exec_policy.json
}


// sns policy
data "aws_iam_policy_document" "eventbridge_sns_policy" {
  statement {
    actions = [
      "sns:Publish",
    ]
    effect    = "Allow"
    resources = ["*"]
  }
}

resource "aws_iam_policy" "eventbridge_sns_policy" {
  name   = "eventbridge-sns-publish"
  policy = data.aws_iam_policy_document.eventbridge_sns_policy.json
}

resource "aws_iam_role_policy_attachment" "eventbridge_sns_policy" {
  role       = aws_iam_role.eventbridge_exec_role.name
  policy_arn = aws_iam_policy.eventbridge_sns_policy.arn
}


// lambda policy
data "aws_iam_policy_document" "eventbridge_invoke_policy" {
  statement {
    actions = [
      "lambda:InvokeFunction",
    ]
    effect    = "Allow"
    resources = ["*"]
  }
}

resource "aws_iam_policy" "eventbridge_invoke_policy" {
  name   = "eventbridge-invoke-function"
  policy = data.aws_iam_policy_document.eventbridge_invoke_policy.json
}

resource "aws_iam_role_policy_attachment" "eventbridge_invoke_policy" {
  role       = aws_iam_role.eventbridge_exec_role.name
  policy_arn = aws_iam_policy.eventbridge_invoke_policy.arn
}




output "eventbridge_exec_role_arn" {
  value = aws_iam_role.eventbridge_exec_role.arn
}
