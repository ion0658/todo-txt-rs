mod de;
mod ser;

pub use de::from_str;
pub use ser::to_string;

pub mod prelude {
    pub use crate::{from_str, to_string};
}

pub const COMPLETE_MARKER: char = 'x';
pub const TOKEN_SEPARATOR: char = ' ';
pub const PRIORITY_MARKER_PRE: char = '(';
pub const PRIORITY_MARKER_POST: char = ')';
pub const PROJECT_MARKER: char = '+';
pub const CONTEXT_MARKER: char = '@';
