#[macro_use]
extern crate pbc_contract_codegen;
extern crate pbc_contract_common;
extern crate reflection;
#[macro_use]
extern crate reflection_derive;

use std::collections::BTreeMap;
//use std::convert::TryInto;
use std::fmt::write;
use std::io::{Read, Write};
use std::ptr::null;

use reflection::Reflection;

use pbc_contract_common::address::Address;
use pbc_contract_common::context::ContractContext;
use pbc_contract_common::serialization::{ReadInt, ReadWrite, WriteInt};

#[state]
pub struct TokenContractState {
    symbol: [u8; 16],
    total_supply: u64,
    balances: BTreeMap<Address, u64>,
}

impl TokenContractState {
    fn update_balance(&mut self, address: Address, delta: i64) {
        // Your code here
        let wallet_balance: u64 = match self.balances.get(&address) {
            Some(balance) => *balance,
            None => 0,
        };

        self.balances.insert(address, wallet_balance + delta as u64);

        self.total_supply += delta as u64;

    }

    fn get_balance(&self, address: Address) -> u64 {
        // Your code here
        let wallet_balance = match self.balances.get(&address) {
            Some(balance) => *balance,
            None => 0,
        };

        wallet_balance
    }
}

impl ReadWrite for TokenContractState {
    fn read_from<T: Read>(reader: &mut T) -> Self {
        // Your code here
        let mut _symbol = [0u8; 16];
        reader.read_exact(&mut _symbol).unwrap();
        let mut _balances = BTreeMap::new();
        _balances = BTreeMap::read_from(reader);
        Self {
            symbol: _symbol,
            total_supply: reader.read_u64_be(),
            balances: _balances,
        }
    }

    fn write_to<T: Write + WriteInt>(&self, writer: &mut T) -> std::io::Result<()> {
       // Your code here
       // self.symbol.write_to(writer)?;
       let _symbol = self.symbol.clone();
       writer.write(&_symbol)?;
       self.total_supply.write_to(writer)?;
       self.balances.write_to(writer)
    }
}

#[init]
pub fn initialize(
    ctx: ContractContext,
    base_state: Option<TokenContractState>,
) -> TokenContractState {
   // Your code here
    let mut _symbol = [0u8; 16];
    let mut _total_supply:u64 = 0;
    let mut _balances = BTreeMap::new();
    match base_state {
        None => {},
        Some(TokenContractState { symbol, total_supply, balances }) => {
            _symbol = symbol;
            _total_supply = total_supply;
            _balances = balances;
        },
    }
    TokenContractState { symbol: _symbol, total_supply: _total_supply, balances: _balances }
}

#[action]
pub fn mint(
    context: ContractContext,
    state: TokenContractState,
    amount: u64,
) -> TokenContractState {
   // Your code here
    let mut _state = state.clone();
    if context.owner != context.sender {
        return _state
    }
    _state.update_balance(context.sender, amount as i64);
    TokenContractState { symbol: _state.symbol, total_supply: _state.total_supply + amount, balances: _state.balances }
}

#[action]
pub fn transfer(
    context: ContractContext,
    state: TokenContractState,
    dest: Address,
    amount: u64,
) -> TokenContractState {
   // Your code here
   let sender = context.sender;
   let mut _state = state.clone();
   if context.owner != sender {
       return _state
   }
   let sender_balance = _state.get_balance(sender);

   // Throw an error if the sender does not have enough balance.
   if sender_balance < amount {
        return _state
   }

   // Modify sender balance.
   _state.update_balance(sender, -(amount as i64));

   // Modify dest balance.
   _state.update_balance(dest, amount as i64);

   TokenContractState {symbol: _state.symbol, total_supply: _state.total_supply, balances: _state.balances}
}
