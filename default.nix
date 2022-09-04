{ pkgs ? import ./nix { }
}:

let
  inherit (pkgs)
    lib
    callPackage
    flyctl
    mkShell
    skopeo
    sources # provided by overlay
    writeShellScript
    writeShellScriptBin
    ;

  fenix = import sources.fenix { inherit pkgs; };

  rustToolchain = with fenix;
    combine [
      stable.rustc
      stable.cargo
    ];

  app =
    let
      naersk = callPackage sources.naersk {
        cargo = rustToolchain;
        rustc = rustToolchain;
      };
    in
    naersk.buildPackage {
      src = lib.cleanSource ./app;
    };

  inherit (callPackage ./fly.nix {
    # rustLog = "debug";
  })
    flyConfig
    flyToml
    ;

  dockerImage = callPackage ./docker-image.nix { inherit flyConfig app; };

  deployScript = writeShellScript "deploy" ''
    set -euo pipefail

    token=$(${flyctl}/bin/flyctl auth token 2>/dev/null || true)
    if [ -z "$token" ]; then
      echo 'Error: Missing fly.io authentication token'
      echo 'Consider running `flyctl auth login` or setting one of $FLY_ACCESS_TOKEN or $FLY_API_TOKEN'
      exit 1
    fi

    # docker image upload target
    image=registry.fly.io/${flyConfig.app}:${dockerImage.imageConfig.tag}

    # upload the docker image
    ${skopeo}/bin/skopeo copy \
      --insecure-policy \
      --dest-username x \
      --dest-password "$token" \
      docker-archive:<(${dockerImage.archiveWriter}) \
      docker://$image

    # trigger deployment of the uploaded docker image
    ${flyctl}/bin/flyctl deploy \
      --config ${flyToml} \
      --image $image
  '';

  devShell = mkShell {
    name = "website";
    buildInputs = [
      flyctl
      rustToolchain
      fenix.rust-analyzer

      # an app specific flyctl command that has this app's fly.toml hardcoded
      # (writeShellScriptBin "flyctl" "exec ${flyctl}/bin/flyctl --config ${flyToml} $@")
    ];
  };
in
{
  inherit
    devShell
    app
    dockerImage
    deployScript
    flyToml
    ;
}
