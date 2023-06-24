pub trait PluginInterface {
    fn start(&self, teststring: String) -> String;
}
