pub struct InterfaceInfo{
    constant_pool_index:u32,
    interface_name: &str,
}

impl InterfaceInfo{
    pub fn new(index: u32, name: &str) -> Self {
        Self {constant_pool_index: index, interface_name: name}
    }
}