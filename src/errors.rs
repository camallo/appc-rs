use nix;
use serde_json;
use std::{io, string, path};

error_chain! {
    foreign_links {
        Json(serde_json::Error);
        Io(io::Error);
        Path(path::StripPrefixError);
        Utf8Parse(string::FromUtf8Error);
        Linux(nix::Error);
    }
}
