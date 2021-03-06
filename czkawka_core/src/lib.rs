#[macro_use]
extern crate bitflags;

pub mod big_file;
pub mod duplicate;
pub mod empty_files;
pub mod empty_folder;
pub mod temporary;

pub mod common;
pub mod common_directory;
pub mod common_extensions;
pub mod common_items;
pub mod common_messages;
pub mod common_traits;
pub mod invalid_symlinks;
pub mod same_music;
pub mod similar_images;
pub mod zeroed;

pub const CZKAWKA_VERSION: &str = env!("CARGO_PKG_VERSION");
