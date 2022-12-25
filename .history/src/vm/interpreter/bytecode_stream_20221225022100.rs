extern crate byteorder;

use crate::vm::oops::method_info::MethodInfo;
use crate::vm::oops::code_attribute_info::CodeAttributeInfo;


use self::byteorder::{BigEndian, ByteOrder};

pub struct BytecodeStream {
    belong_method: MethodInfo,
    belong_code: CodeAttributeInfo,

    codes: [u8],
}

impl BytecodeStream {
    fn read_u8(&self) -> (u8, &[u8]) {
        let (a,b) = self.codes.split_at(1);
        (a[0], b);
    }

    fn read_u16(&self) -> (u16, &[u8]) {
        let (a,b) = self.codes.split_at(2);
        (BigEndian::read_u16(a),b);
    }

    fn read_u16s(&self) -> (u16, &[u8]) {
        let (a,b) = self.read_u16();


    }
}