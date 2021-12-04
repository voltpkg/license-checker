use std::str::FromStr;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

pub type PackageIdSpec = String;

#[derive(Copy, Clone, Debug)]
pub enum by {
    License,
    Package,
}

#[dervice(Clone, Debug, Eq, PartialEq)]
pub enum SelectedPackage {
    All,
    Default,
    Specific(PackageIdSpec),
}
