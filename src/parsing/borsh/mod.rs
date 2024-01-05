use crate::io::{Error, ErrorKind, Read, Result};
use core::mem::size_of;

/// A data-structure that can be de-serialized from binary format by NBOR.
pub trait BorshDeserialize: Sized {
    /// Deserializes this instance from a given slice of bytes.
    /// Updates the buffer to point at the remaining bytes.
    fn deserialize(buf: &mut &[u8]) -> Result<Self> {
        Self::deserialize_reader(&mut *buf)
    }

    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self>;

    /// Deserialize this instance from a slice of bytes.
    fn try_from_slice(v: &[u8]) -> Result<Self> {
        let mut v_mut = v;
        let result = Self::deserialize(&mut v_mut)?;
        if !v_mut.is_empty() {
            return Err(Error::from(ErrorKind::InvalidData));
        }
        Ok(result)
    }

    fn try_from_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let result = Self::deserialize_reader(reader)?;
        let mut buf = [0u8; 1];
        match reader.read_exact(&mut buf) {
            Err(f) if f.kind() == ErrorKind::UnexpectedEof => Ok(result),
            _ => Err(Error::from(ErrorKind::InvalidData)),
        }
    }

    #[doc(hidden)]
    fn array_from_reader<R: Read, const N: usize>(reader: &mut R) -> Result<Option<[Self; N]>> {
        let _ = reader;
        Ok(None)
    }
}

fn unexpected_eof_to_unexpected_length_of_input(e: Error) -> Error {
    if e.kind() == ErrorKind::UnexpectedEof {
        Error::from(ErrorKind::InvalidData)
    } else {
        e
    }
}

impl BorshDeserialize for u8 {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; 1];
        reader
            .read_exact(&mut buf)
            .map_err(unexpected_eof_to_unexpected_length_of_input)?;
        Ok(buf[0])
    }

    #[doc(hidden)]
    fn array_from_reader<R: Read, const N: usize>(reader: &mut R) -> Result<Option<[Self; N]>> {
        let mut arr = [0u8; N];
        reader
            .read_exact(&mut arr)
            .map_err(unexpected_eof_to_unexpected_length_of_input)?;
        Ok(Some(arr))
    }
}

impl BorshDeserialize for u32 {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; size_of::<u32>()];
        reader
            .read_exact(&mut buf)
            .map_err(unexpected_eof_to_unexpected_length_of_input)?;
        let res = u32::from_le_bytes(buf.try_into().unwrap());
        Ok(res)
    }
}

impl BorshDeserialize for u64 {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; size_of::<u64>()];
        reader
            .read_exact(&mut buf)
            .map_err(unexpected_eof_to_unexpected_length_of_input)?;
        let res = u64::from_le_bytes(buf.try_into().unwrap());
        Ok(res)
    }
}

impl BorshDeserialize for u128 {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; size_of::<u128>()];
        reader
            .read_exact(&mut buf)
            .map_err(unexpected_eof_to_unexpected_length_of_input)?;
        let res = u128::from_le_bytes(buf.try_into().unwrap());
        Ok(res)
    }
}

impl<T, const N: usize> BorshDeserialize for [T; N]
where
    T: BorshDeserialize,
{
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        // struct ArrayDropGuard<T, const N: usize> {
        //     buffer: [MaybeUninit<T>; N],
        //     init_count: usize,
        // }
        // impl<T, const N: usize> Drop for ArrayDropGuard<T, N> {
        //     fn drop(&mut self) {
        //         let init_range = &mut self.buffer[..self.init_count];
        //         // SAFETY: Elements up to self.init_count have been initialized. Assumes this value
        //         //         is only incremented in `fill_buffer`, which writes the element before
        //         //         increasing the init_count.
        //         unsafe {
        //             core::ptr::drop_in_place(init_range as *mut _ as *mut [T]);
        //         };
        //     }
        // }
        // impl<T, const N: usize> ArrayDropGuard<T, N> {
        //     unsafe fn transmute_to_array(mut self) -> [T; N] {
        //         debug_assert_eq!(self.init_count, N);
        //         // Set init_count to 0 so that the values do not get dropped twice.
        //         self.init_count = 0;
        //         // SAFETY: This cast is required because `mem::transmute` does not work with
        //         //         const generics https://github.com/rust-lang/rust/issues/61956. This
        //         //         array is guaranteed to be initialized by this point.
        //         core::ptr::read(&self.buffer as *const _ as *const [T; N])
        //     }
        //     fn fill_buffer(&mut self, mut f: impl FnMut() -> Result<T>) -> Result<()> {
        //         // TODO: replace with `core::array::try_from_fn` when stabilized to avoid manually
        //         // dropping uninitialized values through the guard drop.
        //         for elem in self.buffer.iter_mut() {
        //             elem.write(f()?);
        //             self.init_count += 1;
        //         }
        //         Ok(())
        //     }
        // }

        if let Some(arr) = T::array_from_reader(reader)? {
            Ok(arr)
        } else {
            // let mut result = ArrayDropGuard {
            //     buffer: unsafe { MaybeUninit::uninit().assume_init() },
            //     init_count: 0,
            // };

            // result.fill_buffer(|| T::deserialize_reader(reader))?;

            // // SAFETY: The elements up to `i` have been initialized in `fill_buffer`.
            // Ok(unsafe { result.transmute_to_array() })
            unimplemented!("array for [T; N], where T::array_from_reader is not implemented");
        }
    }
}
