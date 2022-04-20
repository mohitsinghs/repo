# repo <img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/mohitsinghs/repo/release?style=flat-square" /> <img alt="GitHub" src="https://img.shields.io/github/license/mohitsinghs/repo?style=flat-square" />

Jump between repositories without pain

## Installing

Download [latest release](https://github.com/mohitsinghs/repo/releases/latest) for your system and put it in the path.

## Usage

**1. Init and populate configuration**

This step is optional, and if you don't do this, your home directory will be used for search.
The config lives in you config directory with name `repo.yml`.

Create config.

```sh
repo init
```

And add some root paths with or without depths.

```yaml
roots:
  # paths will work fine
  - path: /home/mohit/Projects
  # optionally with depth
  - path: /home/mohit/Work
    depth: 2
```

**2. Add shell completions**

Put following in you `~/.zshrc`. The default binding is `z` but you can change it.

```bash
eval $(repo sh zsh)
```

Or if you prefer `x` or some other letter

```bash
eval $(repo sh zsh -b x)
```

### Limitations

- Only zsh is supported for now.
