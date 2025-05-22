{ buffet }:
let
  callPackage = buffet.pkgs.lib.callPackageWith (buffet.pkgs // buffet.support // buffet.support.node // { inherit buffet; });
in
{
  chat = {
    message_store = callPackage ./chat/message_store {};
    user_registry = callPackage ./chat/user_registry {};
    room_manager = callPackage ./chat/room_manager {};
  };
}