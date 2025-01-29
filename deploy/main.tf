# Configure the Azure provider
terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 4.16.0"
    }
  }
}

provider "azurerm" {
  features {}
}

resource "azurerm_resource_group" "simsdeploy" {
  name     = "SIMSDeploy"
  location = "polandcentral"
}

resource "azurerm_dashboard_grafana" "iotgrafana" {
  name                              = "iot-grafana-deploy"
  resource_group_name               = azurerm_resource_group.simsdeploy.name
  location                          = "West Europe"
  grafana_major_version             = 10
  api_key_enabled                   = false
  deterministic_outbound_ip_enabled = false
  public_network_access_enabled     = true

  identity {
    type = "SystemAssigned"
  }
}

resource "azurerm_cosmosdb_postgresql_cluster" "iotpractice" {
  name                            = "iot-practice-1-deploy"
  resource_group_name             = azurerm_resource_group.simsdeploy.name
  location                        = "Poland Central"
  administrator_login_password    = "Password1@"
  coordinator_storage_quota_in_mb = 131072
  coordinator_vcore_count         = 2
  node_count                      = 0
}

resource "azurerm_iothub" "iotpracticehub" {
  name                         = "iot-practicehub-deploy"
  resource_group_name          = azurerm_resource_group.simsdeploy.name
  location                     = "West Europe"

  sku {
    name     = "S1"
    capacity = "1"
  }
}

resource "azurerm_stream_analytics_job" "streamjob" {
  name                                     = "streamjob-deploy"
  resource_group_name                      = azurerm_resource_group.simsdeploy.name
  location                                 = "Poland Central"
  compatibility_level                      = "1.2"
  data_locale                              = "en-GB"
  events_late_arrival_max_delay_in_seconds = 60
  events_out_of_order_max_delay_in_seconds = 50
  events_out_of_order_policy               = "Adjust"
  output_error_policy                      = "Drop"
  streaming_units                          = 3

  transformation_query = <<QUERY
  SELECT
    deviceId AS device_id,
    temperature,
    humidity,
    weight,
    System.Timestamp AS timestamp
  INTO
    [iot-practice-1]
  FROM
    [iot-practicehub]
QUERY

}