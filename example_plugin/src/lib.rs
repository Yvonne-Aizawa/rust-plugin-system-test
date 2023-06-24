use plugin_interface::PluginInterface;
use rand;
#[no_mangle]
pub fn new_service() -> Box<dyn PluginInterface> {
    Box::new(PluginSayHello::new())
}

pub struct PluginSayHello {
    id: String,
}

impl PluginSayHello {
    fn new() -> PluginSayHello {
        let id = format!("{:08x}", rand::random::<u32>());
        println!("[{}] Created instance!", id);
        PluginSayHello { id }
    }
}

impl PluginInterface for PluginSayHello {
    fn start(&self, string: String) -> String {
        let id = format!("{:08x}", rand::random::<u32>());
        println!(
            "[{}] got this from main:  {} and returning this to main {}",
            self.id, string, id
        );
        return id;
    }
}

impl Drop for PluginSayHello {
    fn drop(&mut self) {
        println!("[{}] Destroyed instance!", self.id);
    }
}
