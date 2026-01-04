// ============================================================================
// DATA MODEL (renderable; no positional fields)
// ============================================================================

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
