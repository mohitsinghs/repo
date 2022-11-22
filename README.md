<h1 align='center'>repo</h1>
<p align="center">
  <b>Switch between repositories with ease</b><br/>
  <sub>Access all repositories from anywhere</a></sub>
</p>
<p align='center'>
  <a href="https://github.com/mohitsinghs/repo/actions/workflows/release.yml">
    <img alt="Release" src="https://img.shields.io/github/workflow/status/mohitsinghs/repo/release?style=flat-square" />
  </a>
  <a href="https://github.com/mohitsinghs/repo/blob/main/LICENSE">
    <img alt="LICENSE" src="https://img.shields.io/github/license/mohitsinghs/repo?style=flat-square" />
  </a>
</p>
<br />

## Features

- **Fast enoguh** - takes **~8ms** to scan my entire drive and list all repositories.
- **Fuzzy matching** - can match paths partially.
- **Shell integration** - Works with zsh and bash.
- **Nested output** - supports tree like output for better context.
- **Respects gitignore** - skips nested repositories that are ignored by git.
- **Configurable** - you can control depth for each root for now.

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

### Neovim

There is a [telescope.nvim](https://github.com/nvim-telescope/telescope.nvim) plugin [in my dotfiles](https://github.com/mohitsinghs/dotfiles/blob/master/nvim/lua/lists/repo.lua) that uses this to jump between repositories from within neovim.

### VSCode

- I maintain an [extension](https://marketplace.visualstudio.com/items?itemName=mohitsingh.repo) for VSCode that bundles this.
- There is also `vsix` under releases for those who don't prefer VSCode Marketplace.
- Checkout [vscode-repo](https://github.com/mohitsinghs/vscode-repo) for sources of the extension.

## Configuration

Your home directory will be used for search by default.
The config lives in you config directory with name `repo.yml`.

1. Create config.

```sh
repo init
```

2. Add some root paths with or without depths.

```yaml
roots:
  # paths will work fine
  - path: /home/mohit/Projects
  # optionally with depth
  - path: /home/mohit/Work
    depth: 2
```
