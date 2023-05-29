pub mod base;

pub use base::Base;

pub(crate) mod modules;
pub(crate) mod settings;

pub(crate) use modules::Module;
pub(crate) use settings::Settings;

pub(crate) const SETTINGS: &str = "settings";
