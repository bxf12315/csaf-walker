use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::path::{Path, PathBuf};

/// create a distribution base directory
pub fn distribution_base(base: impl AsRef<Path>, url: &str) -> PathBuf {
    base.as_ref()
        .join(utf8_percent_encode(url, NON_ALPHANUMERIC).to_string())
}
