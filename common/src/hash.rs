use std::io::{Read, Write};

use crate::serialization::ReadWrite;

pub type Hash = [u8; 32];

impl ReadWrite for Hash {
    fn read_from<T: Read>(reader: &mut T) -> Self {
        let mut value = [0u8; 32];
        reader.read_exact(&mut value).unwrap();
        value
    }

    fn write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writer.write_all(self)
    }
}
