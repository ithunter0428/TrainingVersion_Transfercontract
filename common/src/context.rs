use std::io::{Read, Write};

use reflection::Reflection;

use crate::address::Address;
use crate::hash::Hash;
use crate::serialization::{ReadInt, ReadWrite, WriteInt};

#[repr(C)]
#[derive(Eq, PartialEq, Debug, Reflection)]
pub struct ContractContext {
    pub owner: Address,
    pub contract_address: Address,
    pub sender: Address,
    pub block_time: i64,
    pub block_production_time: i64,
    pub current_transaction: Hash,
    pub original_transaction: Hash,
}

impl ReadWrite for ContractContext {
    fn read_from<T: Read>(reader: &mut T) -> Self {
        ContractContext {
            owner: Address::read_from(reader),
            contract_address: Address::read_from(reader),
            sender: Address::read_from(reader),
            block_time: reader.read_i64_be(),
            block_production_time: reader.read_i64_be(),
            current_transaction: Hash::read_from(reader),
            original_transaction: Hash::read_from(reader),
        }
    }

    fn write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        self.owner.write_to(writer)?;
        self.contract_address.write_to(writer)?;
        self.sender.write_to(writer)?;
        writer.write_i64_be(self.block_time)?;
        writer.write_i64_be(self.block_production_time)?;
        self.current_transaction.write_to(writer)?;
        self.original_transaction.write_to(writer)
    }
}
