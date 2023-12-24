<h1 align='center'>repo</h1>
<p align="center">
  <b>Switch between repositories with ease</b><br/>
  <sub>Access all repositories from anywhere</a></sub>
</p>
<p align='center'>
  <a href="https://github.com/mohitsinghs/repo/actions/workflows/release.yml">
    <img alt="Release" src="https://img.shields.io/github/actions/workflow/status/mohitsinghs/repo/release.yml?style=flat-square" />
  </a>
  <a href="https://github.com/mohitsinghs/repo/blob/main/LICENSE">
    <img alt="LICENSE" src="https://img.shields.io/github/license/mohitsinghs/repo?style=flat-square" />
  </a>
</p>
<br />

## Features

- **Fast** - takes under **60ms** to scan list my all repositories.
- **Fuzzy matching** - can match paths partially.
- **Shell integration** - Works with zsh and bash.
- **Output formats** - supports plain and nested json output.
- **Respects gitignore** - skips nested repositories that are ignored by git.
- **Configurable** - you can add multiple roots and control scan depth for each.

## Demo

![repo](https://user-images.githubusercontent.com/4941333/210469514-3e15eb3e-ae9e-44a4-8b97-f01195eb1ea4.gif)

## Installing

Download [latest release](https://github.com/mohitsinghs/repo/releases/latest) for your system and put it in the path.

## Shell integration

You can also run `repo sh` to see the setup instructions for your current shell if supported.

### For zsh

Put following in you `~/.zshrc`. The default binding is `z` but you can change it.

```bash
eval $(repo sh zsh)
```

Or if you prefer `x` or some other letter, use that

```bash
eval $(repo sh zsh -b x)
```

### For bash

Put following in you `~/.bashrc` or `~/.bash_profile`. The default binding is `z` but you can change it.

```bash
eval $(repo sh bash)
```

Or if you prefer `x` or some other letter, use that

```bash
eval $(repo sh bash -b x)
```

## Usage

You can jump around your repos with -

```bash
% z fo/ba
% /foo/bar
% z p/os/vsc
% ~/Projects/oss/vscode-repo
```

Where `z` can be any letter you bind. Hitting the tab will show you possible locations.

## Editor Integration

- **Neovim** - [repo.nvim](https://github.com/mohitsinghs/repo.nvim) is a neovim plugin that uses [telescope.nvim](https://github.com/nvim-telescope/telescope.nvim).
- **VSCode** - [vscode-repo](https://github.com/mohitsinghs/vscode-repo) can be installed from the [marketplace](https://marketplace.visualstudio.com/items?itemName=mohitsingh.repo) or from the `vsix` files under releases.

## Configuration

You can customize the behavior of repo by creating a configuration file named `repo.yml`.

> [!TIP]
> If no repo.yml file exists, the tool will search your home directory by default.

### 1. Initializing Config

To initiate a blank configuration file, execute the following command:

```sh
repo init
```

This creates the `repo.yml` file in your system's default configuration directory.

### 2. Specifying Root Paths

> [!WARNING]
> Ensure paths are specified using the correct path syntax for your operating system.

Define the directories you wish to include in the search. Each path can optionally specify a maximum traversal depth:

```yaml
roots:
  # Search all Git repositories within this directory:
  - path: /home/mohit/Projects

  # Restrict the search to a depth of 2 levels within this directory:
  - path: /home/mohit/Work
    depth: 2
```
