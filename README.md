# Libra

[![CI](https://github.com/lasuillard/Libra/actions/workflows/ci.yml/badge.svg)](https://github.com/lasuillard/Libra/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/lasuillard/Libra/graph/badge.svg?token=jqlkrMOhLd)](https://codecov.io/gh/lasuillard/Libra)

File explorer to manage your files in structured, organized and automated way.

## 🧰 Tech Stack

- **Language** Rust Nightly + Node 20.x

- **Framework** [Tauri](https://tauri.app/) + [SvelteKit](https://kit.svelte.dev/)

- **CI·CD** GitHub Actions

## ⚙️ Getting Started

This section describes how to set your local development environments up.

### **(A)** Developing Inside Container

Requirement:

- [Docker](https://www.docker.com/)

  To configure other dependent services like database, we use Docker (mainly [Docker Compose](https://docs.docker.com/compose/)).

- [Visual Studio Code](https://code.visualstudio.com/)

  VS Code Development Container provides rich features such as git and GnuPG configuration forwarding. But they sometimes require you to install some tools based on your device. Please check [this](https://code.visualstudio.com/docs/remote/containers#_sharing-git-credentials-with-your-container).

As container itself configured to include all required tools, there's no extra tools required.

1. Install VS Code extension [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers).

1. Then, clone this repository and open in VS Code, select **Remote-Containers: Reopen in Container...** at command palette (<kbd>ctrl</kbd> + <kbd>shift</kbd> + <kbd>P</kbd> or <kbd>cmd</kbd> + <kbd>shift</kbd> + <kbd>P</kbd>).

1. Done.

If you are using IDEs other than VS Code, you can use your favorite IDE's remote environments feature. Or, you can set up your environment by yourself locally, as below (B).

### **(B)** Developing Locally

This project dev environment depends on the Docker (it is highly recommended), and no non-Docker local environment is set up. Check out `Makefile` and `Dockerfile` then install system dependencies and dev-tools.

### ⌨️ Basic Commands

Commands repeatedly used are defined in [Makefile](./Makefile). Just type `make` without arguments will show you possible commands.
