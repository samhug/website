
variable "heroku_email" {}
variable "heroku_api_key" {}
variable "cloudflare_email" {}
variable "cloudflare_token" {}

# Configure the Heroku provider
provider "heroku" {
    email = "${var.heroku_email}"
    api_key = "${var.heroku_api_key}"
}

# Configure the CloudFlare provider
provider "cloudflare" {
    email = "${var.cloudflare_email}"
    token = "${var.cloudflare_token}"
}

resource "random_id" "app" {
  keepers = {
    # Generate a new id each time we switch to a new API key
    heroku_api_key = "${var.heroku_api_key}"
  }

  byte_length = 12
}

# Create a new Heroku app
resource "heroku_app" "default" {
    name = "web${random_id.app.hex}"
    region = "us"

    provisioner "local-exec" {
        command = "git push --force --set-upstream https://git:${var.heroku_api_key}@git.heroku.com/${heroku_app.default.name}.git master:master"
    }
}

resource "heroku_domain" "muelh-ug" {
    app = "${heroku_app.default.name}"
    hostname = "muelh.ug"
}
resource "heroku_domain" "sa-muelh-ug" {
    app = "${heroku_app.default.name}"
    hostname = "sa.muelh.ug"
}
resource "heroku_domain" "m-h-ug" {
  app = "${heroku_app.default.name}"
  hostname = "m-h.ug"
}
resource "heroku_domain" "sa-m-h-ug" {
  app = "${heroku_app.default.name}"
  hostname = "sa.m-h.ug"
}

# Update CloudFlare DNS records
resource "cloudflare_record" "m-h-ug" {
  domain = "m-h.ug"
  name = "m-h.ug"
  value = "${heroku_domain.m-h-ug.hostname}.herokudns.com"
  type = "CNAME"
  proxied = true
}
resource "cloudflare_record" "sa-m-h-ug" {
  domain = "m-h.ug"
  name = "sa"
  value = "${heroku_domain.sa-m-h-ug.hostname}.herokudns.com"
  type = "CNAME"
  proxied = true
}
resource "cloudflare_record" "muelh-ug" {
    domain = "muelh.ug"
    name = "muelh.ug"
    value = "${heroku_domain.muelh-ug.hostname}.herokudns.com"
    type = "CNAME"
    proxied = true
}
resource "cloudflare_record" "sa-muelh-ug" {
  domain = "muelh.ug"
  name = "sa"
  value = "${heroku_domain.sa-muelh-ug.hostname}.herokudns.com"
  type = "CNAME"
  proxied = true
}

output "heroku_appname" {
    value = "${heroku_app.default.name}"
}

output "web_url" {
    value = "${heroku_app.default.web_url}"
}