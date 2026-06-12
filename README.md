# gitboy - a a declarative git repo cloner for nixos

`gitboy` is a small tool for cloning git repositories into predefined folders based on a TOML configuration file.

It is idempotent (skips existing repositories).

it is NOT a multi-repo management tool, just ensure all the folders exist where they must.

## features
- declarative: define all of your public repositories in one config file
- uses `libgit2` (doesn't require a system `git`, but you probably should have it)
- very easy to install for nixos (currently the only supported distribution)

## configuration
create a tom file `config.toml` (by default at `~/.config/gitboy/config.toml`) with a similar structure to this:
```toml
[folders."~/Projects/Personal"]
repos = [
    "https://github.com/user/repo1"
    "https://github.com/user/repo2"
    "https://codeberg.org/user/repo3"
]

[folders."~/Projects/Contributing"]
repos = [
    "https://github.com/foss-project/repo1"
]

[folders."~/Testing/"]
repos = [
    "https://gitlab.com/org/project.git"
]
```

- Each `[folders."<path>"]` defines a target directory
- `repos` is the list of repos that will be cloned via `git`
- use *absolute* paths for folders

## install
to get this to run on your system you must use nix flakes (as of right now)

the flake is located at `./flake.nix`
