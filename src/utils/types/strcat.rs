use borsh::io::{Read, Result};

pub fn read_leftover<R: Read>(leftover: usize, reader: &mut R) -> Result<()> {
    let mut leftover_buff = [0u8; 20];

    let iters = leftover / leftover_buff.len();
    let remainder = leftover % leftover_buff.len();

    for _i in 0..iters {
        reader.read_exact(&mut leftover_buff)?;
    }
    reader.read_exact(&mut leftover_buff[0..remainder])?;
    Ok(())
}
