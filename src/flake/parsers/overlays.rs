use anyhow::{Context, Result};

use crate::flake::interface::{INDENT_IN, INDENT_OUT};

// NOTE: Process to follow when adding:
// 1. Neither pin nor overlay pin_exists
// 1.1. Pin gets generated with ref and name
// 1.2. Overlays get updated with new name/list matching the new pin
// 1.3. Package is added to the list for the overlay to process
// 2. Pin already pin_exists
// 2.1 Check if overlay pin_exists
// 2.2 Add package to list in overlay
// 3. Pin and overlay exist
// 3.1. Check if they match current ref
// 3.2. Check if package is named the exact name
// 3.2.1 If package is not the same, add to list
// 3.2.2 If package is the same, nothing changes

// NOTE: Pins shall be named pkgs-<hash>, where sha is the commit hash of the nixpkgs version to be
// pinned. This allows for more than one package to share the same pin, reducing the amount of pins
// if a match is found.

// FIXME: The list of packages would be ideal if it contained names such as <package>@<version>,
// this would allow for clear representation of pinned packages in the subsequent flakes/profiles

pub fn overlay_exists(name: &str) -> Result<bool> {
    Ok(true)
}

pub fn add_overlay(name: &str) -> Result<()> {
    Ok(())
}

pub fn add_package_to_overlay(name: &str, package: &str, version: &str) -> Result<()> {
    Ok(())
}

pub fn remove_overlay(name: &str) -> Result<()> {
    Ok(())
}

pub fn remove_package_form_overlay(package: &str) -> Result<()> {
    Ok(())
}

pub fn pin_exists(name: &str) -> Result<bool> {
    Ok(true)
}

pub fn add_pin(name: &str) -> Result<bool> {
    Ok(true)
}

pub fn remove_pin(name: &str) -> Result<bool> {
    Ok(true)
}
