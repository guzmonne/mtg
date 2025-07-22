pub use anstream::eprintln as aeprintln;
pub use anstream::println as aprintln;
pub use color_eyre::eyre::{eyre, Result};
pub use std::format as f;

/// Create a new prettytable with clean borders.
/// This is useful for displaying structured data in a readable format.
pub fn new_table() -> prettytable::Table {
    let mut table = prettytable::Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_CLEAN);

    table
}
