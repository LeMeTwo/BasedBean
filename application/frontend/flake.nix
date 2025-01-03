{
  description = "Front-end";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-24.05";
  };
  outputs = { self, nixpkgs }:
  
  let 
   pkgs = import nixpkgs { system = "x86_64-linux"; config.allowUnfree = true; };
  in
  { 
    devShells."x86_64-linux".default = pkgs.mkShell {
      venvDir = ".venv";
      packages = with pkgs; [
          # typescript
          nodejs_22
      ];
        
    };
  };
}
