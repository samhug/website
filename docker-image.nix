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

  mkStaticFilesDir = path: runCommand "static-files-dir" { } ''
    mkdir -p $out/${path}
    cp -r ${./static}/* $out/${path}/
  '';

  imageConfig = {
    name = "${flyConfig.app}";
    tag = "latest";
    contents = [
      (runCommand "rootfs-base-layer" { } ''
        mkdir -p $out{/bin,/tmp,/usr,/var/lib}

        # symlink /usr/bin to /bin
        ln -s $out/bin $out/usr/bin

        # symlink our /bin/sh
        ln -s ${runtimeShell} $out/bin/sh
      '')
      (mkStaticFilesDir "/www/static")
      app
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

  # script that constructs a gzip'ed docker archive and streams it to stdout
  archiveWriter = writeShellScript "image-writer-${flyConfig.app}" ''
    set -euo pipefail
    ${dockerTools.streamLayeredImage imageConfig} | ${gzip}/bin/gzip --fast
  '';
}
