{ pkgs ? import ./nix { }
}:

let
  inherit (pkgs)
    buildEnv
    dockerTools
    flyctl
    formats
    gzip
    lib
    runCommand
    skopeo
    ;

  fly-cfg =
    let
      internalPort = 8000;
    in
    {
      app = "web163c91d04b1448cabef86f9f";

      build.image = "registry.fly.io/${fly-cfg.app}:latest";

      deploy.strategy = "immediate";

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
        }
      ];
      # statics = [
      #   {
      #     guest_path = "/app/public";
      #     url_prefix = "/public";
      #   }
      # ];
      env = {
        RUST_LOG = "debug";
        RUST_BACKTRACE = "1";
        ROCKET_ADDRESS = "0.0.0.0";
        ROCKET_PORT = "${toString internalPort}";
      };
    };

  mkFlyTOML = cfg: (formats.toml { }).generate "fly.toml" cfg;
  fly-toml = mkFlyTOML fly-cfg;

  app = pkgsLinux.rustPlatform.buildRustPackage rec {
    name = "app";
    src = ./app;
    cargoSha256 = "sha256-8fQ7Tovh0YI/cDJWOjn8DZuo2vX23v1IyL3cnhNYIiw=";
  };

  pkgsLinux = import "${pkgs.path}" { system = "x86_64-linux"; };

  initScript = pkgs.writeScript "init" ''
    #! ${pkgsLinux.execline}/bin/execlineb -P

    export PATH "${with pkgsLinux; lib.makeBinPath [
      app
      execline
    ]}"

    ${app}/bin/app
  '';

  imageCfg = {
    name = "${fly-cfg.app}";
    tag = "latest";
    contents = [
      (buildEnv {
        name = "root";
        paths = [
          (runCommand "root" { } ''
            mkdir $out
            cd $out

            mkdir -p bin www
            ln -s ${initScript} bin/init
            cp -r ${./static} www/public
          '')
        ];
      })
    ];
    config = {
      Cmd = [ "/bin/init" ];
    };
  };

  deploy-script = pkgs.writeShellScript "deploy" ''
    set -e

    export PATH="${lib.makeBinPath [ flyctl gzip skopeo ]}:$PATH"

    if [ -z "$FLY_ACCESS_TOKEN" ]; then
      echo "FLY_ACCESS_TOKEN not found in environment"
      exit 1
    fi

    skopeo copy \
      --insecure-policy \
      --dest-username x \
      --dest-password "$FLY_ACCESS_TOKEN" \
      docker-archive:<(${dockerTools.streamLayeredImage imageCfg} | gzip --fast) \
      docker://registry.fly.io/${fly-cfg.app}:latest

    flyctl deploy --config ${fly-toml}
  '';

in
{
  inherit
    app
    deploy-script
    fly-toml
    ;
}
