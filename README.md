# My Personal Website

## https://sa.m-h.ug

### Manual deployment steps
- Clone repository
`git clone https://github.com/samhug/website.git; cd website`

- Authenticate to fly.io
`flyctl auth login` or `export FLY_API_TOKEN=...`

- Deploy
`eval (nix-build -A deployScript)`
