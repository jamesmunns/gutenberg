use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use toml::{Value as Toml};

use errors::{Result, ResultExt};


/// Holds the data from a `theme.toml` file.
/// There are other fields than `extra` in it but Gutenberg
/// itself doesn't care about them.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    /// All user params set in [extra] in the theme.toml
    pub extra: HashMap<String, Toml>,
}

impl Theme {
    /// Parses a TOML string to our Theme struct
    pub fn parse(content: &str) -> Result<Theme> {
        let theme = match content.parse::<Toml>() {
            Ok(t) => t,
            Err(e) => bail!(e),
        };

        let mut extra = HashMap::new();
        if let Some(theme_table) = theme.as_table() {
            if let Some(ex) = theme_table.get("extra") {
                if ex.is_table() {
                    extra = ex.clone().try_into().unwrap();
                }
            }
        } else {
            bail!("Expected the `theme.toml` to be a TOML table")
        }


        Ok(Theme {extra})
    }

    /// Parses a theme file from the given path
    pub fn from_file(path: &PathBuf) -> Result<Theme> {
        let mut content = String::new();
        File::open(path)
            .chain_err(|| "No `theme.toml` file found. Are you in the right directory?")?
            .read_to_string(&mut content)?;

        Theme::parse(&content)
    }
}
