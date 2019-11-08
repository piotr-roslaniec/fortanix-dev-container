# Fortanix-dev-container

This project provides Fortanix EDP image to be used with [Remote Development](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.vscode-remote-extensionpack) Visual Studio Code extension.

The Docker image is based on [sebva/docker-sgx](https://github.com/sebva/docker-sgx). Original `README.md` is as a reference in `.devcontainer/README.md`

## Installation

Install [SGX driver (v2.6)](https://github.com/intel/linux-sgx/tree/sgx_2.6) on host OS

## Run example

Once inside container, run:
```bash
cargo run --target x86_64-fortanix-unknown-sgx
```