pub struct AttributeInfo {
    attr_name_index: u32,
    attr_length: u32,

    container: [u8],
}

impl AttributeInfo {
    fn new(&self) -> Self {
        Self { container: [0, self.attr_length] }
    }
}