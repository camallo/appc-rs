extern crate appc;
extern crate serde_json;

#[test]
fn test_image_manifest_example() {
    let f = std::fs::File::open("tests/fixtures/example-image-manifest.json").expect("Missing fixture");
    let bufrd = std::io::BufReader::new(f);
    let _im: appc::spec::ImageManifest = serde_json::from_reader(bufrd).unwrap();
}
