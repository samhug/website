# My Website - [https://sa.m-h.ug](https://sa.m-h.ug)

[![Deploy](https://github.com/samhug/website/actions/workflows/deploy.yml/badge.svg)](https://github.com/samhug/website/actions/workflows/deploy.yml)

### Manual deployment steps
- Clone repository
`git clone https://github.com/samhug/website.git; cd website`

- Authenticate to fly.io
`flyctl auth login` or `export FLY_API_TOKEN=...`

- Deploy
`eval (nix-build -A deployScript)`
