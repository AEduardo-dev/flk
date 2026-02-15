//! # Overlay and Package Pinning Types
//!
//! Data structures for managing version-pinned packages through Nix overlays.
//!
//! When a package is pinned to a specific version (e.g., `flk add ripgrep --version 15.1.0`),
//! flk creates entries in `.flk/pins.nix` that reference specific nixpkgs commits
//! containing that version.
//!
//! ## Structure in pins.nix
//!
//! ```nix
//! {
//!   sources = {
//!     pkgs-abc123 = "github:NixOS/nixpkgs/abc123...";
//!   };
//!   pinnedPackages = {
//!     pkgs-abc123 = [
//!       { pkg = "ripgrep"; name = "ripgrep@15.1.0"; }
//!     ];
//!   };
//! }
//! ```

use anyhow::{Context, Result};

/// A package pinned to a specific version via an overlay.
#[derive(Debug, Clone)]
pub struct PinnedPackage {
    /// Original package name in nixpkgs (e.g., "ripgrep")
    pub name: String,
    /// Aliased name with version (e.g., "ripgrep@15.1.0")
    pub pin_name: String,
}

/// An overlay entry containing packages from a specific nixpkgs commit.
#[derive(Debug, Clone)]
pub struct OverlayEntry {
    /// Overlay identifier (e.g., "pkgs-abc123")
    pub name: String,
    /// Packages provided by this overlay
    pub packages: Vec<PinnedPackage>,
}

/// A source entry pointing to a specific nixpkgs commit.
#[derive(Debug, Clone)]
pub struct SourceEntry {
    /// Source identifier (e.g., "pkgs-abc123")
    pub name: String,
    /// Git reference (e.g., "github:NixOS/nixpkgs/abc123...")
    pub reference: String,
}

/// Collection of overlay entries from the `pinnedPackages` section.
#[derive(Debug, Clone)]
pub struct OverlaysSection {
    /// All overlay entries
    pub entries: Vec<OverlayEntry>,
    /// Detected indentation for consistent formatting
    pub indentation: String,
}

/// Collection of source entries from the `sources` section.
#[derive(Debug, Clone)]
pub struct SourcesSection {
    /// All source entries
    pub entries: Vec<SourceEntry>,
    /// Detected indentation for consistent formatting
    pub indentation: String,
}

impl SourcesSection {
    /// Check if a source with the given name exists.
    pub fn source_exists(&self, source_name: &str) -> bool {
        self.entries.iter().any(|entry| entry.name == source_name)
    }

    /// Add a new source entry.
    ///
    /// # Errors
    ///
    /// Returns an error if a source with the same name already exists.
    pub fn add_source(&mut self, source_name: &str, source_ref: &str) -> Result<()> {
        if self.source_exists(source_name) {
            return Err(anyhow::anyhow!("Source '{}' already exists", source_name));
        }

        self.entries.push(SourceEntry {
            name: source_name.to_string(),
            reference: source_ref.to_string(),
        });

        Ok(())
    }

    /// Remove a source entry.
    ///
    /// # Errors
    ///
    /// Returns an error if the source doesn't exist.
    pub fn remove_source(&mut self, source_name: &str) -> Result<()> {
        let before = self.entries.len();
        self.entries.retain(|e| e.name != source_name);

        if self.entries.len() == before {
            return Err(anyhow::anyhow!(
                "Source '{}' not found in sources section",
                source_name
            ));
        }

        Ok(())
    }
}

impl OverlaysSection {
    /// Check if a pin entry with the given name exists.
    pub fn pin_entry_exists(&self, pin_name: &str) -> bool {
        self.entries.iter().any(|entry| entry.name == pin_name)
    }

    /// Add a new pin entry (overlay source).
    ///
    /// # Errors
    ///
    /// Returns an error if a pin entry with the same name already exists.
    pub fn add_pin_entry(&mut self, pin_name: &str) -> Result<()> {
        if self.pin_entry_exists(pin_name) {
            return Err(anyhow::anyhow!("Pin entry '{}' already exists", pin_name));
        }

        self.entries.push(OverlayEntry {
            name: pin_name.to_string(),
            packages: vec![],
        });

        Ok(())
    }

    /// Remove a pin entry.
    ///
    /// # Errors
    ///
    /// Returns an error if the pin entry doesn't exist.
    pub fn remove_pin_entry(&mut self, pin_name: &str) -> Result<()> {
        let before = self.entries.len();
        self.entries.retain(|e| e.name != pin_name);

        if self.entries.len() == before {
            return Err(anyhow::anyhow!("Pin entry '{}' not found", pin_name));
        }

        Ok(())
    }

    /// Check if a package exists within a specific pin entry.
    pub fn package_in_pin_exists(&self, pin_name: &str, package_alias: &str) -> bool {
        self.entries
            .iter()
            .find(|e| e.name == pin_name)
            .is_some_and(|e| e.packages.iter().any(|p| p.name == package_alias))
    }

    /// Add a package to an existing pin entry.
    ///
    /// # Arguments
    ///
    /// * `pin_name` - The overlay/pin entry name (e.g., "pkgs-abc123")
    /// * `package` - The original package name in nixpkgs
    /// * `package_alias` - The aliased name with version (e.g., "ripgrep@15.1.0")
    ///
    /// # Errors
    ///
    /// Returns an error if the pin entry doesn't exist or the package already exists.
    pub fn add_package_to_pin(
        &mut self,
        pin_name: &str,
        package: &str,
        package_alias: &str,
    ) -> Result<()> {
        let pin_entry = self
            .entries
            .iter_mut()
            .find(|e| e.name == pin_name)
            .context(format!("Pin entry '{}' not found", pin_name))?;

        if pin_entry
            .packages
            .iter()
            .any(|pkg| pkg.pin_name == package_alias)
        {
            return Err(anyhow::anyhow!(
                "Package '{}' already exists in pin '{}'",
                package_alias,
                pin_name
            ));
        }

        pin_entry.packages.push(PinnedPackage {
            name: package.to_string(),
            pin_name: package_alias.to_string(),
        });

        Ok(())
    }

    /// Remove a package from a pin entry.
    ///
    /// # Errors
    ///
    /// Returns an error if the pin entry or package doesn't exist.
    pub fn remove_package_from_pin(&mut self, pin_name: &str, package: &str) -> Result<()> {
        let pin_entry = self
            .entries
            .iter_mut()
            .find(|e| e.name == pin_name)
            .context(format!("Pin entry '{}' not found", pin_name))?;

        let before = pin_entry.packages.len();
        pin_entry.packages.retain(|p| p.name != package);

        if pin_entry.packages.len() == before {
            return Err(anyhow::anyhow!(
                "Package '{}' not found in pin '{}'",
                package,
                pin_name
            ));
        }

        Ok(())
    }
}
