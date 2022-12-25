use std::collections::HashMap

pub struct BootClassLoader {
    #[data(ignore)]
    class_loader_data:<String, InstanceKlass>
}

impl BootClassLoader {
    pub fn load_main_klass(class_path: String) -> InstanceKlass {

    }
}