#[crate_id = "xml#0.1"];
#[crate_type = "lib"];

#[feature(macro_rules, struct_variant)];

extern mod encoding;

pub mod common;
pub mod events;
pub mod pull;
