pub struct FieldsContext {
    pub float_buffer: dtoa::Buffer,
}

impl FieldsContext {
    pub fn new() -> Self {
        Self {
            float_buffer: dtoa::Buffer::new(),
        }
        
    }
}
