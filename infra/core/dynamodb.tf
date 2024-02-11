locals {
  table_name = "${var.table_name}-${var.environment}"
}

resource "aws_dynamodb_table" "watcher-table" {
  name           = local.table_name
  billing_mode   = "PROVISIONED"
  read_capacity  = 20
  write_capacity = 20
  hash_key       = "PK"
  range_key      = "SK"

  attribute {
    name = "PK"
    type = "S"
  }

  attribute {
    name = "SK"
    type = "S"
  }

  attribute {
    name = "node_type"
    type = "S"
  }

  attribute {
    name = "edge_type"
    type = "S"
  }

  ttl {
    attribute_name = "ttl"
    enabled        = true
  }

  global_secondary_index {
    name            = "AdjacencyList"
    hash_key        = "SK"
    range_key       = "PK"
    write_capacity  = 10
    read_capacity   = 10
    projection_type = "ALL"
  }

  global_secondary_index {
    name            = "NodeLookup"
    hash_key        = "node_type"
    range_key       = "PK"
    write_capacity  = 10
    read_capacity   = 10
    projection_type = "ALL"
  }

  global_secondary_index {
    name            = "EdgeLookup"
    hash_key        = "edge_type"
    range_key       = "PK"
    write_capacity  = 10
    read_capacity   = 10
    projection_type = "ALL"
  }

  tags = {
    Name        = local.table_name
    Environment = var.environment
  }
}


output "table_name" {
  value = aws_dynamodb_table.watcher-table.name
}
