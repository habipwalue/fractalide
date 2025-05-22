{ agent, edges, mods, pkgs }:

agent {
  src = ./.;
  edges = with edges; [ chat_message option string ];
  mods = with mods.rs; [ rustfbp ];
  osdeps = with pkgs; [];
}