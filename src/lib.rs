extern crate nix;
extern crate parsswd;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

/// Errors returned by this library, compatible with error-chain consumers.
pub mod errors;
/// Schema types defined in AppC spec.
pub mod schema;
/// User and group name-to-id resolution.
pub mod usergroup;
