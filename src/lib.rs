use std::path::PathBuf;
use thiserror::Error;
use which::which;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to find path using which")]
    Which(#[from] which::Error),
    #[error("unknown error")]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserKind {
    Chrome,
    Chromium,
    Firefox,
    Safari,
    Brave,
    Opera,
}

pub fn get_browser_path(kind: BrowserKind) -> Result<PathBuf, Error> {
    Ok(match kind {
        BrowserKind::Chrome => which("google-chrome").or_else(|_| which("google-chrome-stable")),
        BrowserKind::Chromium => which("chromium").or_else(|_| which("chromium-browser")),
        BrowserKind::Firefox => which("firefox"),
        BrowserKind::Safari => which("safari"),
        BrowserKind::Brave => which("brave").or_else(|_| which("brave-browser")),
        BrowserKind::Opera => which("opera").or_else(|_| which("opera-browser")),
    }?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Browser {
    pub kind: BrowserKind,
    pub path: PathBuf,
}

pub fn get_browsers() -> Vec<Browser> {
    let kinds = vec![
        BrowserKind::Chrome,
        BrowserKind::Chromium,
        BrowserKind::Firefox,
        BrowserKind::Safari,
        BrowserKind::Brave,
        BrowserKind::Opera,
    ];

    kinds
        .into_iter()
        .filter_map(|kind| {
            get_browser_path(kind)
                .map(|path| Browser { kind, path })
                .ok()
        })
        .collect()
}
