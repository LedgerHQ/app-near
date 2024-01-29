pub struct Base58Buf<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> Base58Buf<N> {
    pub fn new() -> Self {
        Self {
            buf: [0u8; N],
            len: 0,
        }
    }

    pub fn encode(&mut self, target: &[u8]) -> Result<(), bs58::encode::Error> {
        self.len = 0;

        // expecting `bs58` to always produce correct strings
        // https://docs.rs/bs58/0.5.0/src/bs58/encode.rs.html#201
        let len = bs58::encode(target).onto(&mut self.buf[..])?;

        self.len = len;
        Ok(())
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[..self.len]).unwrap()
    }
}
