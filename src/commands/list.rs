use colored::Colorize;
use std::path::Path;

use anyhow::Result;

use crate::flake::parser;

pub fn run_list() -> Result<()> {
    let flake_path = Path::new("flake.nix");
    let mut flake_info = parser::parse_flake(flake_path.to_str().unwrap())?;

    println!("{}", "Packages list:".cyan());
    flake_info.packages.sort();

    for elm in flake_info.packages {
        println!("    {}", elm.green());
    }
    Ok(())
}
