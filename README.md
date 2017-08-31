# My Personal Website

## https://sa.m-h.ug

### Deployment

```
git clone https://github.com/samuelhug/website.git
cd website
cp terraform.tfvars.example terraform.tfvars

# Fill in the appropriate credentials
vim terraform.tfvars
```

```
# View the proposed plan
terraform plan

# Deploy
terraform apply
```
