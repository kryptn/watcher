
locals {
  watcher_keeper   = "one"
  artifacts_keeper = "one"
}

resource "random_id" "store_bucket_generation" {
  keepers = {
    # Generate a new id each time we switch to a new AMI id
    watcher_keeper = local.watcher_keeper
  }

  byte_length = 6
}

resource "aws_s3_bucket" "watcher_store" {
  bucket = "watcher-${var.environment}-${random_id.store_bucket_generation.hex}"
  tags = {
    Name        = "watcher-${var.environment}"
    Environment = var.environment
    Keeper      = random_id.store_bucket_generation.keepers.watcher_keeper
  }
}

resource "aws_s3_bucket_public_access_block" "watcher_store_block" {
  bucket = aws_s3_bucket.watcher_store.id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}


resource "random_id" "artifact_bucket_generation" {
  keepers = {
    # Generate a new id each time we switch to a new AMI id
    watcher_keeper = local.artifacts_keeper
  }

  byte_length = 6
}

resource "aws_s3_bucket" "watcher_artifacts" {
  bucket = "watcher-artifacts-${var.environment}-${random_id.artifact_bucket_generation.hex}"
  tags = {
    Name        = "watcher-artifacts-${var.environment}"
    Environment = var.environment
    Keeper      = random_id.artifact_bucket_generation.keepers.watcher_keeper
  }
}

resource "aws_s3_bucket_public_access_block" "watcher_artifacts_block" {
  bucket = aws_s3_bucket.watcher_artifacts.id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

output "watcher_store_bucket" {
  value = aws_s3_bucket.watcher_store.bucket
}

output "watcher_artifacts_bucket" {
  value = aws_s3_bucket.watcher_artifacts.bucket
}

