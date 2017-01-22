
variable "heroku_email" {}
variable "heroku_api_key" {}
variable "cloudflare_email" {}
variable "cloudflare_token" {}

variable "cloudflare_domain" {
	default = "muelh.ug"
}
variable "cloudflare_subdomain" {
	default = "sa"
}

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

# Associate a custom domain
resource "heroku_domain" "default" {
    app = "${heroku_app.default.name}"
    hostname = "${var.cloudflare_subdomain}.${var.cloudflare_domain}"
}

# Add a record to the domain
resource "cloudflare_record" "website" {
    domain = "${var.cloudflare_domain}"
    name = "${var.cloudflare_subdomain}"
    value = "${heroku_domain.default.hostname}.herokudns.com"
    type = "CNAME"
    proxied = true
}

output "heroku_appname" {
    value = "${heroku_app.default.name}"
}

output "web_url" {
    value = "${heroku_app.default.web_url}"
}