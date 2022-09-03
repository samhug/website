{ lib
, dockerTools
, gzip
, runCommand
, writeScript
, writeShellScript
, runtimeShell
  ## ===
, flyConfig
, app
}:

let
  init = writeScript "init" ''
    #!/bin/sh

    export PATH=/bin

    ${app}/bin/server
  '';

  wwwPublic = runCommand "www-public" { } ''
    mkdir -p $out/www/public
    cp -r ${./static}/* $out/www/public/
  '';

  imageConfig = {
    name = "${flyConfig.app}";
    tag = "latest";
    contents = [
      (runCommand "sh" { } ''
        mkdir -p $out/bin
        ln -s ${runtimeShell} $out/bin/sh
      '')
      app
      wwwPublic
    ];
    config = {
      Cmd = [ "${init}" ];
    };
  };
in
{
  inherit imageConfig;

  # docker image in the standard tarball format
  tarball = dockerTools.buildImage imageConfig;

  # script that constucts a gzip'ed docker archive and streams it to stdout
  archiveWriter = writeShellScript "image-writer-${flyConfig.app}" ''
    set -euo pipefail
    ${dockerTools.streamLayeredImage imageConfig} | ${gzip}/bin/gzip --fast
  '';
}
