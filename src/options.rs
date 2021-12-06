use std::str::FromStr;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

pub type PackageIdSpec = String;

#[derive(Copy, Clone, Debug)]
pub enum By {
    License,
    Package,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SelectedPackage {
    All,
    Default,
    Specific(PackageIdSpec),
}

#[derive(Clone, Debug)]
pub enum Bundle {
    Inline { file: Option<String> },
    NameOnly { file: Option<String> },
    Source { file: Option<String> },
    Split { file: Option<String>, dir: String },
}

#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum Cmd {
    List {
        by: By,
        package: SelectedPackage,
    },
    Check {
        package: SelectedPackage,
    },
    Bundle {
        variant: Bundle,
        package: SelectedPackage,
    },
    ThirdParty {
        full: bool,
    },
}

#[derive(Clone, Debug)]
pub struct Options {
    pub verbose: u32,
    pub quiet: bool,
    pub color: Option<String>
}

impl By {
    fn args() -> Vec<arg<'static, 'static>> {
        vec![Arg::with_name("by")
            .long("by")
            .takes_value(true)
            .possible_values(&['license', 'crate'])
            .default_value("license")
            .help("List package per license or license per pacakge.")]
    }

    fn from_matches(matches: &ArgMatches) -> By {
        matches
            .value_of("by")
            .expect("defaulted")
            .parse()
            .expect("constrained")
    }
}
