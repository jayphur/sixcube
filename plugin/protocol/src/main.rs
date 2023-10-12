use fp_bindgen::prelude::*;

// Plugins export these
fp_import! {
    fn test_plugin_fn() -> u32;
}
// Host exports these, plugins import these.
fp_export! {
    fn test_host_fn() -> u32;
}

fn main() {
    use std::path::Path;
    use std::process::Command;
    use std::str::from_utf8;

    let workspace_manifest = Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let workspace_dir = Path::new(from_utf8(&workspace_manifest).unwrap().trim())
        .parent()
        .unwrap()
        .to_str()
        .unwrap();
    println!("workspace dir: {:?}", workspace_dir);
    let bindings_types = [
        BindingsType::RustPlugin(
            RustPluginConfig::builder()
                .name("sc-plugin-bindings")
                .version("1.0.0")
                .build(),
        ),
        BindingsType::RustWasmer2Runtime,
    ];
    for bindings_type in bindings_types {
        let output_path = format!("{workspace_dir}/plugin-bindings/{bindings_type}");
        fp_bindgen!(BindingConfig {
            bindings_type,
            path: &output_path,
        });
        println!("Generated bindings written to `{output_path}/`.");
    }
}
