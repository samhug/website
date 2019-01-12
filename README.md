# My Personal Website

## https://sa.m-h.ug

### Deployment

```
git clone https://github.com/samhug/website.git
cd website
cp terraform.tfvars.example terraform.tfvars

# Fill in the appropriate credentials
vim terraform.tfvars
```

```
# Make sure we have the providers installed
terraform init

# View the proposed plan
terraform plan

# Deploy
terraform apply
```
