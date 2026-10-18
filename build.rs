use std::path::Path;

const ASSETS_PATH: &str = "assets";

#[cfg(target_os = "windows")]
fn windows_flow() {
    const ICON_WINDOWS_NAME: &str = "bbox-64.ico";
    const MANIFEST_NAME: &str = "app.manifest";

    let mut res = winres::WindowsResource::new();
    let assets_path = Path::new(ASSETS_PATH);

    // ico
    {
        let ico_path = assets_path
            .join(ICON_WINDOWS_NAME);

        println!("cargo:rerun-if-changed={}", ico_path.display());
        res.set_icon(ico_path.to_str().unwrap());
    }

    // manifest
    {
        let manifest_path = assets_path
            .join(MANIFEST_NAME);

        println!("cargo:rerun-if-changed={}", manifest_path.display());
        res.set_manifest_file(manifest_path.to_str().unwrap());
    }

    res.compile().expect("failed to compile windows resource");
}

fn main() {
    if cfg!(target_os = "windows") {
        windows_flow();
    }
}