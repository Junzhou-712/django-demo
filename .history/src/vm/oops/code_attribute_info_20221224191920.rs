use crate::vm::interpreter::bytecode_stream::BytecodeStream;
use oops::attribute_info::AttributeInfo;
use std::collections::HashMap;

pub struct CodeAttributeInfo {
    attr_name_index: u32,
    attr_length: u32;

    max_stack: u32,
    max_locals: u32,

    code_length: u32,
    code: BytecodeStream,

    exn_table_length: u32,

    attributes_count: u32,

    attributes: HashMap<&str, AttributeInfo>,
}