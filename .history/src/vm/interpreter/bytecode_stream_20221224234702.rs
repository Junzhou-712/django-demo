use crate::vm::oops::method_info::MethodInfo;
use crate::vm::oops::code_attribute_info::CodeAttributeInfo;

pub struct BytecodeStream {
    belong_method: MethodInfo,
    belong_code: CodeAttributeInfo,

    codes: [u8],
}

impl BytecodeStream {
    fn read_u8(&mut self) -> (u8, &) {
        if self.index < 0 || self.index >= self.length {
            panic! ("invalid bytecode index");
        }
        let a = self.codes[self.index];
        self.index += 1;
        a;
    }

    fn read_u16(&mut self) -> u16 {

    }
}