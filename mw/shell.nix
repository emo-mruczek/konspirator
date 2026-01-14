{ pkgs ? import <nixpkgs> {} }: pkgs.mkShell {
    name = "mw";

    buildInputs = with pkgs; [
        cln
    ];

    nativeBuildInputs = with pkgs; [
        gcc
        gnumake
        flex
        bison
    ];
}
