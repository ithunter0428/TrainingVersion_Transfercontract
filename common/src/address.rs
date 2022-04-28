use std::io::{Read, Write};

use reflection::Reflection;

use crate::serialization::{ReadInt, ReadWrite, WriteInt};
use serde::Serialize;

#[repr(C)]
#[derive(Eq, PartialEq, Debug, Clone, Reflection, Ord, PartialOrd, Copy, Serialize)]
pub enum Address {
    Account(Identifier),
    SystemContract(Identifier),
    PublicContract(Identifier),
    ZkContract(Identifier),
}

type Identifier = [u8; 20];

impl ReadWrite for Address {
    fn read_from<T: Read + ReadInt>(reader: &mut T) -> Self {
        let address_type = reader.read_byte();
        let mut content = [0u8; 20];
        reader.read_exact(&mut content).unwrap();

        match address_type {
            0 => Address::Account(content),
            1 => Address::SystemContract(content),
            2 => Address::PublicContract(content),
            3 => Address::ZkContract(content),
            n => {
                panic!("Unrecognized address type {}", n)
            }
        }
    }

    fn write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        match self {
            Address::Account(content) => {
                writer.write_byte(0).unwrap();
                writer.write_all(content)
            }
            Address::SystemContract(content) => {
                writer.write_byte(1).unwrap();
                writer.write_all(content)
            }
            Address::PublicContract(content) => {
                writer.write_byte(2).unwrap();
                writer.write_all(content)
            }
            Address::ZkContract(content) => {
                writer.write_byte(3).unwrap();
                writer.write_all(content)
            }
        }
    }
}
