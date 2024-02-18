resource "aws_sqs_queue" "watcher_events_queue" {
  name                      = "watcher-events-${var.environment}"
  delay_seconds             = 90
  max_message_size          = 2048
  message_retention_seconds = 86400
  receive_wait_time_seconds = 10
  # redrive_policy = jsonencode({
  #   deadLetterTargetArn = aws_sqs_queue.terraform_queue_deadletter.arn
  #   maxReceiveCount     = 4
  # })


  # tags = {
  #   Environment = "production"
  # }
}


data "aws_iam_policy_document" "events_queue_policy" {
  statement {
    actions = [
      "sqs:SendMessage",
      "sqs:ReceiveMessage",
      "sqs:DeleteMessage",
      "sqs:GetQueueAttributes",
    ]
    effect    = "Allow"
    resources = ["${aws_sqs_queue.watcher_events_queue.arn}"]
  }
}

resource "aws_iam_policy" "events_queue_policy" {
  name   = "sqs-watcher-events-read-write"
  policy = data.aws_iam_policy_document.events_queue_policy.json
}

output "watcher_events_queue_arn" {
  value = aws_sqs_queue.watcher_events_queue.arn
}

output "events_queue_policy_arn" {
  value = aws_iam_policy.events_queue_policy.arn
}
