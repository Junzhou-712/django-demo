use oops::code_attribute_info::CodeAttributeInfo;

pub struct FieldInfo {
    access_flag: u32,
    name_index: u32,
    description_index: u32,
    attribute_count: u32,
    attributes: Vec<CodeAttributeInfo>
}