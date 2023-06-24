use plugin_interface::PluginInterface;
use std::{any::Any, fs};
fn main() {
    //list all files

    let folder_path = "plugins";

    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                println!("File: {:?}", file_name);
                // or use a logging macro like `info!` from the `log` crate
                let lib = libloading::Library::new(format!(
                    "plugins/{}",
                    file_name.into_string().unwrap()
                ))
                .expect("load library");
                let new_service: libloading::Symbol<fn() -> Box<dyn PluginInterface>> =
                    unsafe { lib.get(b"new_service") }.expect("load symbol");
                let service = new_service();
                println!("main got this {}", service.start("oh hi mark".to_string()));
            }
        }
    }
}
