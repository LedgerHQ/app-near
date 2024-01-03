use crate::{io::{Error, ErrorKind, Read, Result}, borsh::BorshDeserialize};
pub struct CappedString<const N: usize> {
    buffer: [u8; N],
    used: usize,
    truncated: bool,
}

impl<const N: usize> CappedString<N> {
    // #[allow(unused)]
    // const fn assert_size() {
    //     if N <= ELIPSIS_SIZE {
    //         panic!("smaller than ellipsis size");
    //     }
    // }

    pub fn new(truncated: bool) -> Self {
        CappedString {
            buffer: [0u8; N],
            used: 0,
            truncated,
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.buffer[..self.used]) }

    }

    #[allow(unused)]
    pub fn truncated(&self) -> bool {
        self.truncated
    } 
}

fn read_leftover<R: Read>(leftover: usize, reader: &mut R) -> Result<()> {
    
    let mut leftover_buff = [0u8; 20];

    let iters = leftover / leftover_buff.len();
    let remainder = leftover % leftover_buff.len();

    for _i in 0..iters {
        reader.read_exact(&mut leftover_buff)?;
        
    }
    reader.read_exact(&mut leftover_buff[0..remainder])?;
    Ok(())
}

impl<const N: usize> BorshDeserialize for CappedString<N> {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let bytes_count: u32 = u32::deserialize_reader(reader)?;
        let truncated = bytes_count > (N as u32);

        let mut result = Self::new(truncated);

        if !truncated {
            
            reader.read_exact(&mut result.buffer[0..(bytes_count as usize)])?;
            result.used = bytes_count as usize;

            // the whole string is expected to be correct
            core::str::from_utf8(&result.buffer[0..(bytes_count as usize)]).map_err(|_err| {
                Error::from(ErrorKind::InvalidData)
                
            })?;

            
        } else {
            let leftover = (bytes_count as usize) - result.buffer.len();
            reader.read_exact(&mut result.buffer)?;

            match core::str::from_utf8(&result.buffer) {
                Ok(_result) => {
                    result.used = result.buffer.len();
                    
                }, 
                Err(err) => {
                    if err.error_len().is_some() {
                        return Err(Error::from(ErrorKind::InvalidData));
                    }
                    let valid_utf8_up_to = err.valid_up_to();
                    result.used = valid_utf8_up_to;
                    
                }
                
            }

            if leftover > 0 {
                read_leftover(leftover, reader)?;
            }


            
        }

        Ok(result)

        
    }
    
}
