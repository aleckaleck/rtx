use color_eyre::eyre::Result;
use indoc::formatdoc;

use crate::cli::command::Command;
use crate::cli::Cli;
use crate::config::Config;
use crate::output::Output;

/// internal command to generate markdown from help
#[derive(Debug, clap::Args)]
#[clap(hide = true)]
pub struct RenderHelp {}

impl Command for RenderHelp {
    fn run(self, _config: Config, out: &mut Output) -> Result<()> {
        let mut cli = Cli::command().term_width(80).max_term_width(80);
        out.stdout.write(formatdoc!(
            r#"
# [rtx](https://github.com/jdxcode/rtx)

[![Crates.io](https://img.shields.io/crates/v/rtx-cli.svg)](https://crates.io/crates/rtx-cli)
[![License: MIT](https://img.shields.io/github/license/jdxcode/rtx)](https://github.com/jdxcode/rtx/blob/main/LICENSE)
[![CI](https://github.com/jdxcode/rtx/actions/workflows/rtx.yml/badge.svg?branch=main)](https://github.com/jdxcode/rtx/actions/workflows/rtx.yml)
[![Codecov](https://codecov.io/gh/jdxcode/rtx/branch/main/graph/badge.svg?token=XYH3Q0BOO0)](https://codecov.io/gh/jdxcode/rtx)
[![Discord](https://img.shields.io/discord/1066429325269794907)](https://discord.gg/mABnUDvP57)

_{about}_

## Quickstart (macOS)

Install rtx (other methods [here](#installation)):

```sh-session
$ brew install jdxcode/tap/rtx
$ rtx --version
rtx {version}
```

Hook rtx into to your shell (choose one, and open a new shell session for the changes to take effect):

```sh-session
$ echo 'eval "$(rtx activate -s bash)"' >> ~/.bashrc
$ echo 'eval "$(rtx activate -s zsh)"' >> ~/.zshrc
$ echo 'rtx activate -s fish | source' >> ~/.config/fish/config.fish
```

Install a runtime and set it as the default:

```sh-session
$ rtx install nodejs@18
$ rtx global nodejs@18
$ node -v
v18.10.9
```

> **Note**  
> `rtx install` is optional, `rtx global` will prompt to install the runtime if it's not
> already installed. This is configurable in [`~/.config/rtx/config.toml`](#configuration).


## About

rtx is a tool for managing programming language and tool versions. For example, use this to install
a particular version of node.js and ruby for a project. Using `rtx activate`, you can have your
shell automatically switch to the correct node and ruby versions when you `cd` into the project's
directory. Other projects on your machine can use a different set of versions.

rtx is inspired by [asdf](https://asdf-vm.com) and uses asdf's vast [plugin ecosystem](https://github.com/asdf-vm/asdf-plugins)
under the hood. However, it is _much_ faster than asdf and has a more friendly user experience.
For more on how rtx compares to asdf, [see below](#comparison-to-asdf). The goal of this project
was to create a better front-end to asdf.

It uses the same `.tool-versions` file that asdf uses. It's also compatible with idiomatic version
files like `.node-version` but you need to enable "legacy version file support" in the config.

### How it works

rtx installs as a shell extension (e.g. `rtx activate -s zsh`) that sets the `PATH`
environment variable to point your shell to the correct runtime binaries. When you `cd` into a
directory containing a `.tool-versions` file, rtx will automatically activate the correct versions.

Every time your prompt starts it will call `rtx hook-env` to fetch new environment variables. This
should be very fast and it exits early if the the directory wasn't changed or the `.tool-version`
files haven't been updated. On my machine this takes 1-2ms even if it _doesn't_ exit early.

Unlike asdf which uses shim files to dynamically locate runtimes when they're called, rtx modifies
`PATH` ahead of time so the runtimes are called directly. This is not only faster since it avoids
any overhead, but it also makes it so commands like `which node` work as expected. This also
means there isn't any need to run `asdf reshim` after installing new runtime binaries.

### Common example commands

    rtx install nodejs@20.0.0       Install a specific version number
    rtx install nodejs@20.0         Install a fuzzy version number
    rtx local nodejs@20             Use node-20.x in current project
    rtx global nodejs@20            Use node-20.x as default

    rtx install nodejs              Install the latest available version
    rtx local nodejs@latest         Use latest node in current directory
    rtx global nodejs@system        Use system node as default

    rtx x nodejs@20 -- node app.js  Run `node app.js` with the PATH pointing to node-20.x

## Installation

> **Warning**
> Regardless of the installation method, when uninstalling rtx,
> remove `RTX_DATA_DIR` folder (usually `~/.local/share/rtx`) to fully clean up.

### Standalone

Note that it isn't necessary for `rtx` to be on `PATH`. If you run the activate script in your rc
file, rtx will automatically add itself to `PATH`.

```sh-session
$ curl https://rtx.jdxcode.com/install.sh | sh
```

### Homebrew

```sh-session
$ brew install jdxcode/tap/rtx
```

### Cargo

Build from source with Cargo.

```sh-session
$ cargo install rtx-cli
```

Do it faster with [cargo-binstall](https://github.com/cargo-bins/cargo-binstall):

```sh-session
$ cargo install cargo-binstall
$ cargo binstall rtx-cli
```

### npm

rtx is available on npm as precompiled binaries. This isn't a node.js package, just distributed
via npm. It can be useful for JS projects that want to setup rtx via `package.json` or `npx`.

```sh-session
$ npm install -g @jdxcode/rtx
```

Or use npx if you just want to test it out for a single command without fully installing:

```sh-session
$ npx @jdxcode/rtx exec python@3.11 -- python some_script.py
```

### GitHub Releases

Download the latest release from [GitHub](https://github.com/jdxcode/rtx/releases).

```sh-session
$ curl https://github.com/jdxcode/rtx/releases/rtx-latest-macos-arm64.tar.xz | tar -xJv
$ mv rtx/bin/rtx /usr/local/bin
```

### ~~apt~~ (coming soon)

For installation on Ubuntu/Debian:

```sh-session
wget -qO - https://rtx.jdxcode.com/gpg-key.pub | gpg --dearmor | sudo tee /usr/share/keyrings/rtx-archive-keyring.gpg 1> /dev/null
echo "deb [signed-by=/usr/share/keyrings/rtx-archive-keyring.gpg arch=amd64] https://rtx.jdxcode.com/deb stable main" | sudo tee /etc/apt/sources.list.d/rtx.list
sudo apt update
sudo apt install -y rtx
```

> **Warning**
> If you're on arm64 you'll need to run the following:
>     echo "deb [signed-by=/usr/share/keyrings/rtx-archive-keyring.gpg arch=arm64] https://rtx.jdxcode.com/deb stable main" | sudo tee /etc/apt/sources.list.d/rtx.list

### ~~dnf~~ (coming soon)

For Fedora, CentOS, Amazon Linux, RHEL and other dnf-based distributions:

```sh-session
dnf install -y dnf-plugins-core
dnf config-manager --add-repo https://rtx.jdxcode.com/rpm/rtx.repo
dnf install -y rtx
```

### ~~yum~~ (coming soon)

```sh-session
yum install -y yum-utils
yum config-manager --add-repo https://rtx.jdxcode.com/rpm/rtx.repo
yum install -y rtx
```

### ~~apk~~ (coming soon)

For Alpine Linux:

```sh-session
apk add rtx --repository=http://dl-cdn.alpinelinux.org/alpine/edge/testing/
```

### ~~aur~~ (coming soon)

For Arch Linux:

```sh-session
git clone https://aur.archlinux.org/rtx.git
cd rtx
makepkg -si
```

## Other Shells

### Bash

```sh-session
$ echo 'eval "$(rtx activate -s bash)"' >> ~/.bashrc
```

### Fish

```sh-session
$ echo 'rtx activate -s fish | source' >> ~/.config/fish/config.fish
```

## Configuration

### `.tool-versions`

The `.tool-versions` file is used to specify the runtime versions for a project. An example of this 
is:

```
nodejs      20.0.0  # comments are allowed
ruby        3       # can be fuzzy version
shellcheck  latest  # also supports "latest"
jq          1.6
```

Create `.tool-versions` files manually, or use [`rtx local`](#rtx-local) to create them automatically.
See [the asdf docs](https://asdf-vm.com/manage/configuration.html#tool-versions) for more info on this file format.

### Global config: `~/.config/rtx/config.toml`

rtx can be configured in `~/.config/rtx/config.toml`. The following options are available (defaults shown):

```toml
# whether to prompt to install plugins and runtimes if they're not already installed
missing_runtime_behavior = 'prompt' # other options: 'ignore', 'warn', 'prompt', 'autoinstall'

# plugins can read the versions files used by other version managers (if enabled by the plugin)
# for example, .nvmrc in the case of nodejs's nvm
legacy_version_file = false         # not enabled by default

# configure `rtx install` to always keep the downloaded archive
always_keep_download = false        # deleted after install by default

# configure how frequently (in minutes) to fetch updated plugin repository changes
# this is updated whenever a new runtime is installed
plugin_autoupdate_last_check_duration = 10080 # (one week) set to 0 to disable updates

# configure how frequently (in minutes) to fetch updated shortname repository changes
# note this is not plugins themselves, it's the shortname mappings
# e.g.: nodejs -> https://github.com/asdf-vm/asdf-nodejs.git
plugin_repository_last_check_duration = 10080 # (one week) set to 0 to disable updates

# disables the short name repository (described above)
disable_plugin_short_name_repository = false

[alias.nodejs]
my_custom_node = '18'  # makes `rtx install nodejs@my_custom_node` install node-18.x
                       # this can also be specified in a plugin (see below in "Aliases")
```

### Environment variables

rtx can also be configured via environment variables. The following options are available:

#### `RTX_MISSING_RUNTIME_BEHAVIOR`

This is the same as the `missing_runtime_behavior` config option in `~/.config/rtx/config.toml`.

#### `RTX_DATA_DIR`

This is the directory where rtx stores its data. The default is `~/.local/share/rtx`.

```sh-session
$ RTX_MISSING_RUNTIME_BEHAVIOR=ignore rtx install nodejs@20
$ RTX_NODEJS_VERSION=20 rtx exec -- node --version
```

#### `RTX_CONFIG_FILE`

This is the path to the config file. The default is `~/.config/rtx/config.toml`.
(Or `$XDG_CONFIG_HOME/config.toml` if that is set)

#### `RTX_DEFAULT_TOOL_VERSIONS_FILENAME`

Set to something other than ".tool-versions" to have rtx look for configuration with alternate names.

#### `RTX_${{PLUGIN}}_VERSION`

Set the version for a runtime. For example, `RTX_NODEJS_VERSION=20` will use nodejs@20.x regardless
of what is set in `.tool-versions`.

#### `RTX_LEGACY_VERSION_FILE`

Plugins can read the versions files used by other version managers (if enabled by the plugin)
for example, .nvmrc in the case of nodejs's nvm.

## Aliases

rtx supports aliasing the versions of runtimes. One use-case for this is to define aliases for LTS
versions of runtimes. For example, you may want to specify `lts/hydrogen` as the version for nodejs@18.x.
So you can use the runtime with `nodejs lts/hydrogen` in `.tool-versions`.

User aliases can be created by adding an `alias.<PLUGIN>` section to `~/.config/rtx/config.toml`:

```toml
[alias.nodejs]
my_custom_18 = '18'
```

Plugins can also provide aliases via a `bin/list-aliases` script. Here is an example showing node.js
versions:

```bash
#!/usr/bin/env bash

echo "lts/hydrogen 18"
echo "lts/gallium 16"
echo "lts/fermium 14"
```

> **Note:**
> Because this is rtx-specific functionality not currently used by asdf it isn't likely to be in any
> plugin currently, but plugin authors can add this script without impacting asdf users.

## Plugins

rtx uses asdf's plugin ecosystem under the hood. See https://github.com/asdf-vm/asdf-plugins for a
list.

## FAQs

### I don't want to put a `.tool-versions` file into my project since git shows it as an untracked file.

You can make git ignore these files in 3 different ways:

- Adding `.tool-versions` to project's `.gitignore` file. This has the downside that you need to commit the change to the ignore file.
- Adding `.tool-versions` to project's `.git/info/exclude`. This file is local to your project so there is no need to commit it.
- Adding `.tool-versions` to global gitignore (`core.excludesFile`). This will cause git to ignore `.tool-versions` files in all projects. You can explicitly add one to a project if needed with `git add --force .tool-versions`.

### How do I create my own plugin?

Just follow the [asdf docs](https://asdf-vm.com/plugins/create.html). Everything should work the same.
If it isn't, please open an issue.

### rtx is failing or not working right

First try setting `RTX_LOG_LEVEL=debug` or `RTX_LOG_LEVEL=trace` and see if that gives you more information.
You can also set `RTX_LOG_FILE=/path/to/logfile` to write the logs to a file.

If something is happening with the activate hook, you can try disabling it and calling `eval "$(rtx hook-env)"` manually.
It can also be helpful to use `rtx env` to see what environment variables it wants to use.

Lastly, there is an `rtx doctor` command. It doesn't have much in it but I hope to add more functionality
to that to help debug issues.

### Windows support?

This is unlikely to ever happen since this leverages the vast ecosystem of asdf plugins which are built on Bash scripts.
At some point it may be worth exploring an alternate plugin format that would be Windows compatible.

## Commands

"#,
            version = env!("CARGO_PKG_VERSION"),
            about = cli.get_about().unwrap(),
        ));
        for command in cli.get_subcommands_mut() {
            match command.has_subcommands() {
                true => {
                    let name = command.get_name().to_string();
                    for subcommand in command.get_subcommands_mut() {
                        if let Some(output) = render_command(Some(&name), subcommand) {
                            out.stdout.write(output);
                        }
                    }
                }
                false => {
                    if let Some(output) = render_command(None, command) {
                        out.stdout.write(output);
                    }
                }
            }
        }

        rtxprintln!(
            out,
            r#"
## Comparison to asdf

rtx is mostly a clone of asdf, but there are notable areas where improvements have been made.

### Performance

asdf made (what I consider) a poor design decision to use shims that go between a call to a runtime
and the runtime itself. e.g.: when you call `node` it will call an asdf shim file `~/.asdf/shims/node`,
which then calls `asdf exec`, which then calls the correct version of node.

These shims have terrible performance, adding ~200ms to every call. rtx does not use shims and instead
updates `PATH` so that it doesn't have any overhead when simply calling binaries. These shims are the main reason that I wrote this.

I don't think it's possible for asdf to fix thse issues. The author of asdf did a great writeup
of [performance problems](https://stratus3d.com/blog/2022/08/11/asdf-performance/). asdf is written
in bash which certainly makes it challening to be performant, however I think the real problem is the
shim design. I don't think it's possible to fix that without a complete rewrite.

rtx does call an internal command `rtx hook-env` every time the directory has changed, but because
it's written in Rust, this is very quick—taking ~2ms on my machine.

tl;dr: asdf adds overhead (~200ms) when calling a runtime, rtx adds a tiny amount of overhead (~2ms)
when changing directories.

### Environment variables

asdf only helps manage runtime executables. However, some tools are managed via environment variables
(notably Java which switches via `JAVA_HOME`). This isn't supported very well in asdf and requires
a separate shell extension just to manage.

However asdf _plugins_ have a `bin/exec-env` script that is used for exporting environment variables
like [`JAVA_HOME`](https://github.com/halcyon/asdf-java/blob/master/bin/exec-env). rtx simply exports
the environment variables from the `bin/exec-env` script in the plugin but places them in the shell
for _all_ commands. In asdf it only exports those commands when the shim is called. This means if you
call `java` it will set `JAVA_HOME`, but not if you call some Java tool like `mvn`.

This means we're just using the existing plugin script but because rtx doesn't use shims it can be
used for more things. It would be trivial to make a plugin that exports arbitrary environment
variables like [dotenv](https://github.com/motdotla/dotenv) or [direnv](https://github.com/direnv/direnv).

### UX

Some commands are the same in asdf but others have been changed. Everything that's possible
in asdf should be possible in rtx but may use slighly different syntax. rtx has more forgiving commands,
such as using fuzzy-matching, e.g.: `rtx install nodejs@18`. While in asdf you _can_ run
`asdf install nodejs latest:18`, you can't use `latest:18` in a `.tool-versions` file or many other places.
In `rtx` you can use fuzzy-matching everywhere.

asdf requires several steps to install a new runtime if the plugin isn't installed, e.g.:

```sh-session
$ asdf plugin add nodejs
$ asdf install nodejs latest:18
$ asdf local nodejs latest:18
```

In `rtx` this can all be done in a single step to set the local runtime version. If the plugin
and/or runtime needs to be installed it will prompt:

```sh-session
$ asdf local nodejs@18
rtx: Would you like to install nodejs@18.13.0? [Y/n] Y
Trying to update node-build... ok
Downloading node-v18.13.0-darwin-arm64.tar.gz...
-> https://nodejs.org/dist/v18.13.0/node-v18.13.0-darwin-arm64.tar.gz
Installing node-v18.13.0-darwin-arm64...
Installed node-v18.13.0-darwin-arm64 to /Users/jdx/.local/share/rtx/installs/nodejs/18.13.0
$ node -v
v18.13.0
```

I've found asdf to be particularly rigid and difficult to learn. It also made strange decisions like
having `asdf list all` but `asdf latest --all` (why is one a flag and one a positional argument?).
`rtx` makes heavy use of aliases so you don't need to remember if it's `rtx plugin add nodejs` or
`rtx plugin install nodejs`. asdf also just has too many commands. It's hard to remember what the
difference is between `asdf list` and `asdf current` is. `rtx` has a single command `rtx list` which
can be passed a flag `rtx list --current` to show the current versions.

That said, there are a lot of great things about asdf. It's the best multi-runtime manager out there
and I've really been impressed with the plugin system. Most of the design decisions the authors made
were very good. I really just have 2 complaints: the shims and the fact it's written in Bash.

## Cache Behavior

rtx makes use of caching in many places in order to be efficient. The details about how long to keep
cache for should eventually all be configurable. There may be gaps in the current behavior where
things are hardcoded but I'm happy to add more settings to cover whatever config is needed.

Below I explain the behavior it uses around caching. If you're seeing behavior where things don't appear
to be updating, this is a good place to start.

### Shorthand Repository Cache

asdf maintains a [shorthand repository](https://github.com/asdf-vm/asdf-plugins) which maps plugin
short names (e.g.: `nodejs`) to full repository names (e.g.: `https://github.com/asdf-vm/asdf-nodejs`).

This is stored in `~/.local/share/rtx/repository` and updated every week by default if short names
are requested. This is similar to what asdf does, but I'm considering just baking this straight into
the codebase so it doesn't have to be fetched/maintained separately. It's not like new plugins get
added that often.

### Plugin Cache

Each plugin has a cache that's stored in `~/.local/share/rtx/plugins/<PLUGIN>/.rtxcache.msgpack.gz`. It stores
the list of versions available for that plugin (`rtx ls-remote <PLUGIN>`) and the legacy filenames (see below).

It is updated daily by default or anytime that `rtx ls-remote` is called explicitly. The file is
gzipped messagepack, if you want to view it you can run the following (requires [msgpack-cli](https://github.com/msgpack/msgpack-cli)).

```sh-session
cat ~/.local/share/rtx/plugins/nodejs/.rtxcache.msgpack.gz | gunzip | msgpack-cli decode
```

### Runtime Cache

Each runtime (language version, e.g.: `nodejs@20.0.0`), has a file called "runtimeconf" that's stored
inside the install directory, e.g.: `~/.asdf/installs/nodejs/20.0.0/.rtxconf.msgpack`. This stores the
information about the runtime that should not change after installation. Currently this is just the
bin paths the plugin defines in `bin/list-bin-paths`. By default this is just `/bin`. It's the list
of paths that rtx will add to `PATH` when the runtime is activated.

I have not seen a plugins which has _dynamic_ bin paths but let me know if you find one. If that is
the case, we may need to make this cached instead of static.

"Runtimeconf" is stored as uncompressed messagepack and can be viewed with the following:

```
cat ~/.local/share/rtx/installs/nodejs/18.13.0/.rtxconf.msgpack | msgpack-cli decode
```

### Legacy File Cache

If enabled with `legacy_version_file = true` in `~/.config/rtx/config.toml`, rtx will read the legacy
filenames such as `.node-version` for [asdf-nodejs](https://github.com/asdf-vm/asdf-nodejs).
This leverages cache in 2 places where the plugin is called:

- [`list-legacy-filenames`](https://github.com/asdf-vm/asdf-nodejs/blob/master/bin/list-legacy-filenames)
    In every plugin I've seen this simply returns a static list of filenamed like ".nvmrc .node-version".
    It's cached alongside the standard "runtime" cache which is refreshed daily by default.
- [`parse-legacy-file`](https://github.com/asdf-vm/asdf-nodejs/blob/master/bin/parse-legacy-file)
    This plugin binary is called to parse a legacy file to get the version out of it. It's relatively
    expensive so every file that gets parsed as a legacy file is cached into `~/.local/share/rtx/legacy_cache`.
    It will remain cached until the file is modified. This is a simple text file that has the path to the
    legacy file stored as a hash for the filename.

## Development

Run tests with `just`:

```sh-session
$ just test
```

Lint the codebase with:

```sh-session
$ just lint-fix
```
"#
        );

        Ok(())
    }
}

fn render_command(parent: Option<&str>, c: &mut clap::Command) -> Option<String> {
    if c.is_hide_set() {
        return None;
    }
    let name = match parent {
        Some(p) => format!("{} {}", p, c.get_name()),
        None => c.get_name().to_string(),
    };
    Some(formatdoc!(
        "
        ### `rtx {name}`

        ```
        {about}
        ```
        ",
        name = name,
        about = c.render_long_help(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::assert_cli;

    use super::*;

    #[test]
    fn test_render_help() {
        let Output { stdout, .. } = assert_cli!("render-help");
        assert!(stdout.content.contains("Quickstart"));
    }
}