resource "aws_sqs_queue" "review_secret" {
  name                       = "review-secret-queue"
  visibility_timeout_seconds = 30
  message_retention_seconds  = 86400 # 1 DAY
  max_message_size           = 2048
  delay_seconds              = 10
  receive_wait_time_seconds  = 0

}

# COULD SETUP DLQ
output "review_secret" {
  value = aws_sqs_queue.review_secret
}
