{ agent, edges, mods, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ user_event option string ];
  mods = with mods.rs; [ rustfbp ];
  osdeps = with pkgs; [];
}