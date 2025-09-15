{
  config,
  pkgs,
  lib,
  ...
}:

let
  inherit (lib)
    types
    mkEnableOption
    mkPackageOption
    mkOption
    ;
  inherit (lib) mkIf;

  cfg = config.services.bluegent;
  settingsFormat = pkgs.formats.toml { };

  description = "Bluegent, a BlueZ authentication agent made for headless devices";
in
{
  options.services.bluegent = {
    enable = mkEnableOption description;

    package = mkPackageOption pkgs "bluegent" { };

    settings = mkOption {
      type = types.submodule {
        freeformType = settingsFormat.type;

        options = {
          pin_code = mkOption {
            type = types.str;
            description = "Preset PIN code to use for legacy pairing.";
            default = "0000";
          };

          authorized_services = mkOption {
            type = with types; listOf str;
            description = "List of authorized (allowed to use by devices) services.
            These should be UUIDs with hex letters encoded in lowercase.
            At the moment, Bluegent will not tell you if you specified them incorrectly,
            it will probably just reject your service.
            As you can probably tell it's very much a work-in-progress";
            default = [ ];
          };
        };
      };
    };
  };

  config = mkIf (cfg.enable) {
    assertions = [
      {
        assertion = cfg.enable -> config.hardware.bluetooth.enable;
        message = "Bluetooth needs to be enabled in the system configuration for Bluegent to be useful";
      }
    ];

    environment.etc."bluegent.conf".source = settingsFormat.generate "bluegent.conf" cfg.settings;

    systemd.services.bluegent =
      let
        dependencies = [
          "bluetooth.service"
        ];
      in
      {
        inherit description;

        bindsTo = dependencies;
        after = dependencies;

        restartTriggers = [ cfg ];

        serviceConfig = {
          Type = "exec";
          ExecStart = "${cfg.package}/bin/bluegent";
          Restart = "on-failure";
        };
      };
  };
}
