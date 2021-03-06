use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    cdylib_link_lines::metabuild();

    let target_dir = PathBuf::from("../target");

    let pkg_config = format!(
        include_str!("s76_hidpi_widget.pc.in"),
        name = "s76_hidpi_widget",
        description = env::var("CARGO_PKG_DESCRIPTION").unwrap(),
        version = env::var("CARGO_PKG_VERSION").unwrap()
    );

    File::create(target_dir.join("s76_hidpi_widget.pc.stub"))
        .expect("failed to create pc.stub")
        .write_all(&pkg_config.as_bytes())
        .expect("failed to write pc.stub");
}
