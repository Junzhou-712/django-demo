pub struct InterfaceInfo{
    constant_pool_index:u32,
    interface_name: String,
}

impl InterfaceInfo{
    pub fn new(index: u32, name: &str) -> Self {
        Self {
            index,
            name,
        }
    }

}