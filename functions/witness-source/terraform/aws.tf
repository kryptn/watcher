
terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "5.36.0"
    }
  }
}

terraform {
  backend "s3" {
    bucket         = "infra-remote-tf-state"
    key            = "state/watcher/functions/witness-source/terraform.tfstate"
    region         = "us-west-2"
    encrypt        = "true"
    kms_key_id     = "alias/terraform-bucket-key"
    dynamodb_table = "terraform-state"
  }
}
