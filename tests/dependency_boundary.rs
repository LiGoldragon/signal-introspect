const FRAME_REVISION: &str = "8aa0bcaeb29fe9e461a11706a469638d2fd109ac";
const SCHEMA_RUST_REVISION: &str = "664335240a40728826cfaa09e3100cd867031912";

#[test]
fn dependency_graph_has_one_frame_and_one_schema_rust_world() {
    let manifest = include_str!("../Cargo.toml");
    let lockfile = include_str!("../Cargo.lock");

    assert_eq!(lockfile.matches("name = \"signal-frame\"").count(), 1);
    assert_eq!(lockfile.matches("name = \"schema-rust\"").count(), 1);
    assert!(lockfile.contains(&format!(
        "signal-frame.git?rev={FRAME_REVISION}#{FRAME_REVISION}"
    )));
    assert!(lockfile.contains(&format!(
        "schema-rust.git?rev={SCHEMA_RUST_REVISION}#{SCHEMA_RUST_REVISION}"
    )));
    assert!(!manifest.contains("branch ="));
    assert!(!lockfile.contains("?branch="));
}
