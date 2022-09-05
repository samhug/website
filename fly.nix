{ lib
, formats
  ## ===
, appName ? "web163c91d04b1448cabef86f9f"
, internalPort ? 8000
, rustLog ? null
}:

let
  flyConfig = {
    app = appName;
    # deploy.strategy = "immediate";
    services = [
      {
        internal_port = internalPort;
        protocol = "tcp";
        ports = [
          {
            handlers = [ "http" ];
            port = "80";
          }
          {
            handlers = [ "tls" "http" ];
            port = "443";
          }
        ];
        http_checks = [
          {
            grace_period = "1s";
            interval = "15s";
            timeout = "2s";
            path = "/_health";
            method = "get";
            protocol = "http";
          }
        ];
      }
    ];
    env = {
      APP_LISTEN_ADDR = "0.0.0.0:${toString internalPort}";
      APP_HOST_REDIRECT = "sa.h-h.ug";
      APP_STATIC_FILES_DIR = "/www/static";
    }
    // lib.optionalAttrs (rustLog != null) { RUST_LOG = rustLog; }
    ;
  };

  flyToml = (formats.toml { }).generate "fly.toml" flyConfig;
in
{
  inherit
    flyConfig
    flyToml
    ;
}
