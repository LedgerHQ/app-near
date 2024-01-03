use ledger_device_sdk::{
    buttons::ButtonEvent,
    io::{Comm, Event, StatusWords},
};

use crate::{io, Instruction};

pub struct SingleTxStream<'a> {
    pub comm: &'a mut Comm,
    chunk_counter: usize,
    is_last_chunk: bool,
}

impl<'a> SingleTxStream<'a> {
    pub fn new(comm: &'a mut Comm, is_last_chunk: bool) -> Self {
        let total_counter = 0;
        Self {
            comm,
            chunk_counter: total_counter,
            is_last_chunk,
        }
    }
}

pub struct HashingStream<R> {
    reader: R,
    hash_state: u32,
}

impl<R> HashingStream<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            hash_state: 0,
            
        }
        
    }
}

impl<R: io::Read> io::Read for HashingStream<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() > 0 {
            let n = self.reader.read(buf)?;

            if n > 0 {
                self.hash_state += buf[n - 1] as u32;
            }
            return Ok(n);
        }
        Ok(0)
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
        let is_last_chunk = loop {
            match self.comm.next_event() {
                Event::Button(button) => match button {
                    ButtonEvent::BothButtonsRelease => {
                        return Err(io::Error::from(io::ErrorKind::Interrupted))
                    }
                    _ => {
                        // ignore all other button presses
                    }
                },
                Event::Command(Instruction::GetVersion)
                | Event::Command(Instruction::GetPubkey) => {
                    return Err(io::Error::from(io::ErrorKind::InvalidData))
                }
                Event::Command(Instruction::SignTx { is_last_chunk }) => break is_last_chunk,
                _ => (),
            };
        };

        self.is_last_chunk = is_last_chunk;
        self.chunk_counter = 0;
        let mut data = self
            .comm
            .get_data()
            .map_err(|_err| io::Error::from(io::ErrorKind::BrokenPipe))?;
        if data.is_empty() {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }
        let n = data.read(buf)?;
        self.chunk_counter += n;
        return Ok(n);
    }
}
