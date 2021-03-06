use std::{
    fmt::{self, Display, Formatter},
    path::PathBuf,
    str::FromStr,
};

use structopt::StructOpt;

/// Generates Nix files for Rust projects which uses naersk.
#[allow(clippy::struct_excessive_bools)]
#[derive(StructOpt, Debug, Default, Clone)]
#[structopt(name = "rust-nix-templater")]
pub struct Options {
    /// Enable GitHub Actions file generation.
    #[structopt(long)]
    pub github_ci: bool,
    /// Enable GitLab CI file generation.
    #[structopt(long)]
    pub gitlab_ci: bool,
    /// Disable app / builds flake output generation.
    #[structopt(long)]
    pub disable_build: bool,

    /// Output directory where generated files will be put in. [example: -o example]
    #[structopt(short, long, default_value = ".")]
    pub out_dir: PathBuf,

    /// License of the package. Must be a valid Cargo.toml license. [example: mit]
    #[structopt(short = "l", long = "license")]
    pub package_license: Option<String>,
    /// Name of the package. Must be passed when also creating a Cargo project. [example: icy_matrix]
    #[structopt(short = "n", long = "name")]
    pub package_name: Option<String>,
    /// Systems that the package is supported on. [example: -s x86_64-linux x86_64-darwin] [default: nixpkgs default systems]
    #[structopt(short = "s", long = "systems")]
    pub package_systems: Option<Vec<String>>,

    /// A short, single line description of the package.
    #[structopt(short = "d", long = "description")]
    pub package_description: Option<String>,
    /// A longer description of the package.
    #[structopt(short = "D", long = "long-description")]
    pub package_long_description: Option<String>,
    /// Homepage of the package. [example: -h "https://gitlab.com/example/example"]
    #[structopt(short = "h", long = "homepage")]
    pub package_homepage: Option<String>,

    /// Whether to disable app output for flake.
    #[structopt(short = "A", long = "no-app")]
    pub disable_app: bool,
    /// Whether to copy libraries to package output.
    #[structopt(short = "L", long = "library")]
    pub package_lib: bool,

    /// Icon to use in the generated desktop file. [example: --icon assets/icon.ico]
    #[structopt(long = "icon")]
    pub package_icon: Option<String>,
    /// Comment to put in the generated desktop file. [default: package description]
    #[structopt(long = "xdg-comment")]
    pub package_xdg_comment: Option<String>,
    /// Desktop name to put in the generated desktop file. Defaults to package name. [example: --xdg-desktop-name "Icy Matrix"]
    #[structopt(long = "xdg-desktop-name")]
    pub package_xdg_desktop_name: Option<String>,
    /// Generic name to put in the generated desktop file. [example: --xdg-generic-name "Matrix Client"]
    #[structopt(long = "xdg-generic-name")]
    pub package_xdg_generic_name: Option<String>,
    /// Categories to put in the generated desktop file. [example: --xdg-categories "Network;InstantMessaging;"]
    #[structopt(long = "xdg-categories")]
    pub package_xdg_categories: Option<String>,

    /// Rust toolchain channel to use. [example: -t nightly]
    #[structopt(short = "t", long = "toolchain", default_value = "stable")]
    pub rust_toolchain_channel: RustToolchainChannel,

    /// Cachix cache name. [example: --cachix-name rust-nix-templater]
    #[structopt(long)]
    pub cachix_name: Option<String>,
    /// Cachix cache public key. [example: --cachix-public-key "rust-nix-templater.cachix.org-1:Tmy1V0KK+nxzg0XFePL/++t4JRKAw5tvr+FNfHz7mIY=""]
    #[structopt(long)]
    pub cachix_public_key: Option<String>,
}

#[derive(Debug)]
pub enum RustToolchainChannelParseError {
    Invalid,
}

impl Display for RustToolchainChannelParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Invalid => write!(f, "Invalid channel name specified. Valid channels are 'stable', 'beta' and 'nightly'."),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustToolchainChannel {
    Stable,
    Beta,
    Nightly,
}

impl Default for RustToolchainChannel {
    fn default() -> Self {
        Self::Stable
    }
}

impl FromStr for RustToolchainChannel {
    type Err = RustToolchainChannelParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "stable" => Ok(Self::Stable),
            "beta" => Ok(Self::Beta),
            "nightly" => Ok(Self::Nightly),
            _ => Err(Self::Err::Invalid),
        }
    }
}

impl Display for RustToolchainChannel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Stable => write!(f, "stable"),
            Self::Beta => write!(f, "beta"),
            Self::Nightly => write!(f, "nightly"),
        }
    }
}
