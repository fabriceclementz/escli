pub mod aliases;
pub mod indices;
pub mod mappings;
pub mod reindex;
pub use colored::Colorize;

mod aliases_add;
mod aliases_list;
mod aliases_remove;
mod aliases_update;
mod indices_close;
mod indices_create;
mod indices_delete;
mod indices_list;
mod indices_open;
mod indices_settings;
mod indices_settings_get;
mod mappings_get;
