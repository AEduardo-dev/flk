use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct PinnedPackage {
    pub name: String,
    pub pin_name: String,
}

#[derive(Debug, Clone)]
pub struct OverlayEntry {
    pub name: String,
    pub packages: Vec<PinnedPackage>,
}

#[derive(Debug, Clone)]
pub struct SourceEntry {
    pub name: String,
    pub reference: String,
}

#[derive(Debug, Clone)]
pub struct OverlaysSection {
    pub entries: Vec<OverlayEntry>,
    pub indentation: String,
}

#[derive(Debug, Clone)]
pub struct SourcesSection {
    pub entries: Vec<SourceEntry>,
    pub indentation: String,
}

impl SourcesSection {
    pub fn source_exists(&self, source_name: &str) -> bool {
        self.entries.iter().any(|entry| entry.name == source_name)
    }

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
    pub fn pin_entry_exists(&self, pin_name: &str) -> bool {
        self.entries.iter().any(|entry| entry.name == pin_name)
    }

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

    pub fn remove_pin_entry(&mut self, pin_name: &str) -> Result<()> {
        let before = self.entries.len();
        self.entries.retain(|e| e.name != pin_name);

        if self.entries.len() == before {
            return Err(anyhow::anyhow!("Pin entry '{}' not found", pin_name));
        }

        Ok(())
    }

    pub fn package_in_pin_exists(&self, pin_name: &str, package_alias: &str) -> bool {
        self.entries
            .iter()
            .find(|e| e.name == pin_name)
            .is_some_and(|e| e.packages.iter().any(|p| p.name == package_alias))
    }

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
