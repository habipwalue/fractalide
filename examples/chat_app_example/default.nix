{ pkgs ? import <nixpkgs> {}
, fractalide ? import ../fractalide {}
, buffet ? fractalide.buffet
}:

let
  defaultEdges = buffet.edges;
  defaultNodes = buffet.nodes;
  
  customEdges = import ./edges { inherit buffet; };
  customNodes = import ./nodes { inherit buffet; };
  
  edges = defaultEdges // customEdges;
  nodes = defaultNodes // customNodes;
  
  # Update the buffet with our custom components
  updatedBuffet = buffet // {
    inherit edges nodes;
  };
  
  # Build the application
  chat_app = import ./app { 
    buffet = updatedBuffet;
    inherit pkgs;
  };
  
in {
  app = chat_app;
  edges = customEdges;
  nodes = customNodes;
}