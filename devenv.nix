{ pkgs, lib, config, inputs, ... }:

{
  # Set environment variable
  env.GREET = "devenv";

  # Install Git, Rust, Trunk (for frontend), and necessary dependencies for Axum
  packages = [
    pkgs.git
    pkgs.rustup
    pkgs.trunk
    pkgs.llvm
    pkgs.cargo
    pkgs.lld
    pkgs.openssl
  ];

  # Enable Rust for both frontend and backend
  languages.rust.enable = true;

  # Frontend setup using Trunk
  processes.frontend = {
    exec = "trunk serve";  # Adjust as needed, trunk serves the frontend
  };

  # Backend setup with Axum (Rust web framework)
  processes.backend = {
    exec = "cargo run";  # Backend command to run the Axum app
  };

  # Custom task to run both frontend and backend
  tasks = {
    "dev:all".exec = ''
      # Run both processes in parallel
      ${pkgs.bash}/bin/bash -c "
        devenv frontend &
        devenv backend &
        wait
      "
    '';
  };

  # Scripts to greet and run Git version checks
  scripts.hello.exec = ''
    echo hello from $GREET
  '';

  
  enterShell = ''
    hello
    git --version
    rustc --version  # Check Rust version
  '';

  # Testing setup (example test for git)
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

}
