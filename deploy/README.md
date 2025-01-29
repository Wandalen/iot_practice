# Azure Deployment with Terraform

## Create a Service Principal

Run the following command to create a Service Principal with the `Contributor` role:

```bash
az ad sp create-for-rbac --role="Contributor" --scopes="/subscriptions/bcf62ce6-f30a-4cb1-9485-85a712cc619d"
```

The output will provide the following information:
- `appId` (ARM_CLIENT_ID)
- `password` (ARM_CLIENT_SECRET)
- `tenant` (ARM_TENANT_ID)
- `subscription` (ARM_SUBSCRIPTION_ID)

## Set Environment Variables

Export the credentials from the output:

```bash
export ARM_CLIENT_ID="<APPID_VALUE>"
export ARM_CLIENT_SECRET="<PASSWORD_VALUE>"
export ARM_SUBSCRIPTION_ID="<SUBSCRIPTION_ID>"
export ARM_TENANT_ID="<TENANT_VALUE>"
```

## Install Terraform

Follow the [Terraform installation guide](https://learn.hashicorp.com/tutorials/terraform/install-cli) if you haven't installed Terraform yet.

## Deployment

```bash
terraform init
```

Check the plan:

```bash
terraform plan
```

Deploy the resources:

```bash
terraform apply
```

Confirm with `yes` when prompted.
