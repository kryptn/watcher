

terraform {
  required_providers {
    spacelift = {
      source  = "spacelift-io/spacelift"
      version = "1.9.2"
    }
  }
}

provider "spacelift" {
  # Configuration options
}


resource "spacelift_stack" "watcher" {
  administrative    = false
  autodeploy        = true
  branch            = "main"
  description       = "Watcher"
  name              = "Watcher"
  project_root      = "infra/core/"
  repository        = "kryptn/watcher"
  terraform_version = "1.5.7"
}
