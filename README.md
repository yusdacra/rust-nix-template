# rust-nix-templater

Generates Nix files for Rust projects which use [naersk](https://github.com/nmattia/naersk).

## Features

- Generate for applications or libraries
- Support for both flakes and legacy nix
- Generates release, debug and test packages
- Cachix support
- Generates development shell
- Desktop file generation
- CI file generation (currently only for GitHub Actions)

## Installation

- Flakes: `nix profile install github:yusdacra/rust-nix-templater`
- Legacy: `nix-env -i -f "https://github.com/yusdacra/rust-nix-templater/tarball/master"`

## Examples

Simple:

```shell
rust-nix-templater -l mit -n example -o .
# is equal to
rust-nix-templater --license mit --name example --out-dir .
```

This will generate files in the current directory, with license set to `mit` and package name set to `example`. It will generate both build and development environment files that have a binary package, using Rust's `stable` toolchain.

For a project that uses `rust-toolchain` file:

```shell
rust-nix-templater -T -l mit -n example -o .
# is equal to
rust-nix-templater --use-toolchain-file -l mit -n example -o .
```

This will do what the previous examples does plus use `rust-toolchain` file instead of Rust's `stable` toolchain.

For a project that uses `rust-toolchain` file, but is only a library:

```shell
rust-nix-templater -L -T -l mit -n example -o .
# is equal to
rust-nix-templater --library -T -l mit -n example -o .
```

This will do what the previous example does but it won't generate a binary package (which means it also won't generate a Flake application).

For a project that uses `beta` toolchain and is hosted on GitHub:

```shell
rust-nix-templater -c github -t beta -l mit -n example -o .
# is equal to
rust-nix-templater --ci github --toolchain beta -l mit -n example -o .
```

This will do what the first example does, but use `beta` toolchain and also generate a GitHub Actions workflow.

For more options please check `rust-nix-templater --help`.

## Usage

```
rust-nix-templater 0.1.0
Generates Nix files for Rust projects which uses naersk

USAGE:
    rust-nix-templater [FLAGS] [OPTIONS] --license <package-license> --name <package-name>

FLAGS:
        --disable-build         Disable build files generation
        --help                  Prints help information
    -M, --mk-desktop-file       Create a desktop file
    -L, --library               Create a library package instead of a binary package
    -T, --use-toolchain-file    Use the `rust-toolchain` file instead of a channel
    -V, --version               Prints version information

OPTIONS:
        --cachix-name <cachix-name>                      Cachix cache name. [example: --cachix-name rust-nix-templater]
        --cachix-public-key <cachix-public-key>
            Cachix cache public key. [example: --cachix-public-key "rust-nix-templater.cachix.org-
            1:Tmy1V0KK+nxzg0XFePL/++t4JRKAw5tvr+FNfHz7mIY=""]
    -c, --ci <ci>...                                     Which CI systems to create CI files for. [example: -c github]
    -o, --out-dir <out-dir>
            Output dir where rendered files will be put in. [example: -o .] [default: out]

    -d, --description <package-description>              A short, single line description of the package
    -e, --executable <package-executable>
            Name of the executable `cargo build` generates. Required if your package's executable name is different from
            your package's name
    -h, --homepage <package-homepage>
            Homepage of the package. [example: -h "https://gitlab.com/example/example"]

        --icon <package-icon>
            Icon to use in the generated desktop file. [example: --icon assets/icon.ico]

    -l, --license <package-license>
            License of the package. Can be any of the values listed in
            https://github.com/NixOS/nixpkgs/blob/master/lib/licenses.nix. [example: -l mit]
    -D, --long-description <package-long-description>    A longer description of the package
    -n, --name <package-name>                            Name of the package. [example: -n icy_matrix]
    -s, --systems <package-systems>...
            Systems that the package is supported on. [example: -s x86_64-linux x86_64-darwin] [default: nixpkgs default
            systems]
        --xdg-categories <package-xdg-categories>...
            Categories to put in the generated desktop file. [example: --xdg-categories Network InstantMessaging]

        --xdg-comment <package-xdg-comment>
            Comment to put in the generated desktop file. [default: package description]

        --xdg-desktop-name <package-xdg-desktop-name>
            Desktop name to put in the generated desktop file. Defaults to package name. [example: --xdg-desktop-name
            "Icy Matrix"]
        --xdg-generic-name <package-xdg-generic-name>
            Generic name to put in the generated desktop file. [example: --xdg-generic-name "Matrix Client"]

    -t, --toolchain <rust-toolchain-channel>
            Rust toolchain channel to use. [example: -t nightly] [default: stable]
```
