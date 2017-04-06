extern crate appc;
extern crate serde_json;

use std::{io, fs};

#[test]
fn test_pod_manifest_example() {
    let f = fs::File::open("tests/fixtures/example-pod-manifest.json").expect("Missing fixture");
    let bufrd = io::BufReader::new(f);
    let _manif: appc::spec::PodManifest = serde_json::from_reader(bufrd).unwrap();
}
