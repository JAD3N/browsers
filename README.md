# Browsers

This crate is a quick and easy way to find what browsers are installed on an operating system (supports MacOS, Windows and Linux).

# Usage

You can use the crate easily by one of two methods:

```rust
use browsers::{Browser, BrowserKind, get_browser_path, get_browsers};

// find all browser installations on the system
let browsers: Vec<Browser> = get_browsers();

// find a specific browser installation path
let path: PathBuf = get_browser_path(BrowserKind::Chrome)
    .expect("failed to find browser installation");
```