use windres::Build;

fn main() {
    Build::new().compile("tray-icon.rc").unwrap();
}
