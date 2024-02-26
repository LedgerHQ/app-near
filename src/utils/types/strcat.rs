use borsh::io::{Read, Result};
use core::str::from_utf8;

pub fn concatenate<'a>(
    strings: &[&str],
    output: &'a mut [u8],
) -> core::result::Result<&'a str, ()> {
    let mut offset = 0;

    for s in strings {
        let s_len = s.len();
        if offset + s_len > output.len() {
            return Err(());
        }

        output[offset..offset + s_len].copy_from_slice(s.as_bytes());
        offset += s_len;
    }

    Ok(from_utf8(&output[..offset]).unwrap())
}
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
