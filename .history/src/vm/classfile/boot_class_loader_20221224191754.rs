use std::collections::HashMap
use oops::instance_klass::InstanceKlass

pub struct BootClassLoader {
    #[data(ignore)]
    class_loader_data: HashMap<&str, InstanceKlass>,
}

impl BootClassLoader {
    pub fn load_main_klass(class_path: String) -> InstanceKlass {
        
    }
}