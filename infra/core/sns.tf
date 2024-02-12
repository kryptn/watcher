
resource "aws_sns_topic" "observation_scheduled_topic" {
  name = "observation-scheduled-topic"
}

resource "aws_sns_topic" "endpoint_observed_topic" {
  name = "observation-scheduled-topic"
}

output "observation_scheduled_topic_arn" {
  value = aws_sns_topic.observation_scheduled_topic.arn
}

output "endpoint_observed_topic_arn" {
  value = aws_sns_topic.endpoint_observed_topic.arn
}