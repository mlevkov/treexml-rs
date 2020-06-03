use thiserror::Error;
/*
use xml;
// this is redundant, cargo clippy suggested to remove it
  = note: `#[warn(clippy::single_component_path_imports)]` on by default
  = help: for further information visit
  https://rust-lang.github.io/rust-clippy/master/index.html#single_component_path_imports
*/
#[derive(Debug, Error)]
pub enum TreexmlError {
    #[error("Element not found: '{t}'")]
    ElementNotFound { t: String },
    #[error("Value could not be parsed: '{t}'")]
    ValueFromStr { t: String },
    #[error("Parse error: '{source}'")]
    ParseError {
        #[from]
        source: xml::reader::Error,
    },
    #[error("Write error: '{source}'")]
    WriteError {
        #[from]
        source: xml::writer::Error,
    },
}
