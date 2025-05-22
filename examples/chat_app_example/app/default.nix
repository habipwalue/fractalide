{ buffet, pkgs }:

let
  agent = buffet.nodes.rs.agent;
  mods = buffet.mods;
  edges = buffet.edges;
in

agent {
  src = ./.;
  edges = with edges; [ ];
  mods = with mods.rs; [ rustfbp ];
  osdeps = with pkgs; [];
  crates = with buffet.support.rust; [];
}