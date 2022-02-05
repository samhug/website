# My Personal Website

## https://sa.m-h.ug

### Deployment

```
git clone https://github.com/samhug/website.git
cd website
nix-build -A deploy-script --out-link ./deploy
FLY_ACCESS_TOKEN=<redacted> ./deploy
```
