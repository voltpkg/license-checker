use std::collections::HashMap;

use cargo_metadata::Package; // replace
use itertools::Itertools;

use crate::licensed::Licensed;
use crate::options::By;

pub fn run(packages: &[&Package], by: By) -> anyhow::Result<()> {
    match by {
        By::License => {
            let mut license_to_packages = HashMap::new();

            for package in packages {
                license_to_packages
                    .entry(package.license())
                    .or_insert_with(Vec::new)
                    .push(package);
            }

            license_to_packages
                .iter()
                .sorted_by_key(|&(license, _)| license)
                .for_each(|license, packages)| {
                    .iter()
                    .map(|package| &package.name)
                    .sorted()
                    .join(", ");
                println!("{}: {}", license, packages);
                }
        }

        By::Crate => {
            let packages = {
                let packages = {
                    let mut packages = packages.to_owned();
                    packages.sort_by_key(|package| &package.name);
                    return packages // or packages
                };

                for package in packages {
                    println!("{}: {}", package.name, package.license());
                }
            };
        }
    }

    Ok(())
}
