use ledger_device_sdk::{
    buttons::ButtonEvent,
    io::{Comm, Event},
};

use crate::{
    io::{self},
    AppSW, Instruction, SignMode,
};
use ledger_secure_sdk_sys::{
    cx_hash_final, cx_hash_t, cx_hash_update, cx_sha256_init_no_throw, cx_sha256_t, CX_OK,
};

#[cfg(feature = "speculos")]
use ledger_device_sdk::testing;

pub struct SingleTxStream<'a> {
    pub comm: &'a mut Comm,
    chunk_counter: usize,
    is_last_chunk: bool,
    sign_mode: SignMode,
}

impl<'a> SingleTxStream<'a> {
    pub fn new(comm: &'a mut Comm, is_last_chunk: bool, sign_mode: SignMode) -> Self {
        let total_counter = 0;
        Self {
            comm,
            chunk_counter: total_counter,
            is_last_chunk,
            sign_mode,
        }
    }
}

pub struct Sha256Digest(pub [u8; 32]);

pub struct HashingStream<R> {
    pub reader: R,
    sha256_ctx: cx_sha256_t,
}

impl<R> HashingStream<R> {
    pub fn new(reader: R) -> Result<Self, AppSW> {
        let mut sha256_ctx = Default::default();
        unsafe {
            if cx_sha256_init_no_throw(&mut sha256_ctx) != CX_OK {
                return Err(AppSW::TxHashFail);
            }
        }
        let res = Self { reader, sha256_ctx };
        Ok(res)
    }

    pub fn finalize(&mut self) -> Result<Sha256Digest, AppSW> {
        let mut array = [0u8; 32];
        unsafe {
            if cx_hash_final(
                &mut self.sha256_ctx.header as *mut cx_hash_t,
                array.as_mut_ptr(),
            ) != CX_OK
            {
                #[cfg(feature = "speculos")]
                testing::debug_print("`cx_hash_final` error encountered \n");
                return Err(AppSW::TxHashFinalizeFail);
            }
        }
        Ok(Sha256Digest(array))
    }
}
impl<R> HashingStream<R> {
    pub fn feed_slice(&mut self, input: &[u8]) -> io::Result<()> {
        unsafe {
            if cx_hash_update(
                &mut self.sha256_ctx.header as *mut cx_hash_t,
                input.as_ptr(),
                input.len(),
            ) != CX_OK
            {
                #[cfg(feature = "speculos")]
                testing::debug_print("`cx_hash_update` error encountered \n");
                return Err(io::Error::from(io::ErrorKind::OutOfMemory));
            }
        }
        Ok(())
    }
}

impl<R: io::Read> io::Read for HashingStream<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if !buf.is_empty() {
            let n = self.reader.read(buf)?;

            // update hash on each chunk passing through
            if n > 0 {
                let data = &buf[0..n];
                unsafe {
                    if cx_hash_update(
                        &mut self.sha256_ctx.header as *mut cx_hash_t,
                        data.as_ptr(),
                        data.len(),
                    ) != CX_OK
                    {
                        #[cfg(feature = "speculos")]
                        testing::debug_print("`cx_hash_update` error encountered \n");
                        return Err(io::Error::from(io::ErrorKind::OutOfMemory));
                    }
                }
            }
            return Ok(n);
        }
        Ok(0)
    }
}

impl<'a> SingleTxStream<'a> {
    pub fn peek_u8(&mut self) -> io::Result<Option<u8>> {
        let data = self
            .comm
            .get_data()
            .map_err(|_err| io::Error::from(io::ErrorKind::BrokenPipe))?;

        let (_read, left) = data.split_at(self.chunk_counter);

        if left.is_empty() && self.is_last_chunk {
            return Ok(None);
        }

        if !left.is_empty() {
            return Ok(Some(left[0]));
        }

        let data = self.get_next_chunk()?;
        Ok(Some(data[0]))
    }

    fn get_next_chunk(&mut self) -> io::Result<&[u8]> {
        let is_last_chunk = loop {
            match self.comm.next_event() {
                Event::Button(ButtonEvent::BothButtonsRelease) => {
                    return Err(io::Error::from(io::ErrorKind::Interrupted))
                }
                Event::Command(Instruction::GetVersion)
                | Event::Command(Instruction::GetPubkey { .. }) => {
                    return Err(io::Error::from(io::ErrorKind::InvalidData))
                }
                Event::Command(Instruction::SignTx {
                    is_last_chunk,
                    sign_mode,
                }) if sign_mode == self.sign_mode => break is_last_chunk,
                Event::Command(Instruction::SignTx { .. }) => {
                    return Err(io::Error::from(io::ErrorKind::InvalidData))
                }
                _ => (),
            };
        };

        self.is_last_chunk = is_last_chunk;
        self.chunk_counter = 0;
        let data = self
            .comm
            .get_data()
            .map_err(|_err| io::Error::from(io::ErrorKind::BrokenPipe))?;
        if data.is_empty() {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }
        Ok(data)
    }
}

impl<'a> io::Read for SingleTxStream<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let data = self
            .comm
            .get_data()
            .map_err(|_err| io::Error::from(io::ErrorKind::BrokenPipe))?;

        let (_read, mut left) = data.split_at(self.chunk_counter);

        if left.is_empty() && self.is_last_chunk {
            return Ok(0);
        }

        if !left.is_empty() {
            let n = left.read(buf)?;
            self.chunk_counter += n;
            return Ok(n);
        }

        // first inform the sender we're ready to receive next chunk
        self.comm.reply_ok();
        let mut data = self.get_next_chunk()?;
        let n = data.read(buf)?;
        self.chunk_counter += n;
        Ok(n)
    }
}
