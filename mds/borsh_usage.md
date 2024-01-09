- [ ] extract everything about `alloc` crate into separate feature
  - [ ] change `borsh/src/nostd_io.rs` or make 2 variants, depending on `alloc` feature
  - [ ] make `BorshDeserialize` trait method `vec_from_reader` depending on `alloc` feature 
  - [ ] each usage of `io::Error::new` implies depending on `alloc` feature, there's around 50 occurencies of it in `borsh` 
  - [ ] no change required to `BorshSerialize`
  - [ ] make `unstable__schema` feature imply `alloc` featue
```rust
// from
enum Repr {
    Simple(ErrorKind),
    Custom(Custom),
}

impl Error {
    pub fn new<T: Into<String>>(kind: ErrorKind, error: T) -> Error {
        Self::_new(kind, error.into())
    }

    fn _new(kind: ErrorKind, error: String) -> Error {
        Error {
            repr: Repr::Custom(Custom { kind, error }),
        }
    }
}
```

```rust
// to
enum Repr {
    Simple(ErrorKind),
}
impl Error {
  
}
```

- [ ] one cannot `extern crate alloc` without defining allocator
  - [ ] after
    - [ ] adding stub allocator (allocator doing only panics)
    - [ ] patching `cargo ledger build` with  `"-Z",  "build-std=core,alloc,proc_macro"`
  - [ ] compiling fails,  [similar issue](https://gitlab.com/jD91mZM2/no-std-compat/-/issues/6)
    - [ ] probably `nanos`, `nanosplus`, `nanox` targets' json config has to be defined more correctly
- [ ] `pub fn deserialize_reader_in_place<R: Read>(&mut self, reader: &mut R) -> Result<()>` is more efficient for larger types
  - [ ] than `fn deserialize_reader<R: Read>(rd: &mut R) -> Result<Self>`
  - [ ] changing to `deserialize_reader_in_place` allowed to increase used buffers
