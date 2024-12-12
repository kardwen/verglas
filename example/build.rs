fn main() {
    println!("cargo::rerun-if-changed=assets/icons/*");

    let source_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons");
    let font_file_dest = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/example-icons.ttf");

    verglas::make_font(source_dir, font_file_dest).expect("building icon font failed")
}
