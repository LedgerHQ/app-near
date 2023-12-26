use core::str::{from_utf8, Utf8Error};

pub use path::{PathBip32, ALLOWED_PATH_LEN};
pub use public_key::{PublicKeyBe, bip32_derive};

pub mod path;
pub mod public_key;

pub enum ConcatError {
    Utf8(Utf8Error),
    Overflow,
    
}

/// Returns concatenated strings, or an error if the concatenation buffer is too small.
pub fn concatenate<'a>(strings: &[&str], output: &'a mut [u8]) -> Result<&'a str, ConcatError> {
    let mut offset = 0;

    for s in strings {
        let s_len = s.len();
        if offset + s_len > output.len() {
            return Err(ConcatError::Overflow);
        }

        output[offset..offset + s_len].copy_from_slice(s.as_bytes());
        offset += s_len;
    }

    Ok(from_utf8(&output[..offset]).map_err(ConcatError::Utf8)?)
}

