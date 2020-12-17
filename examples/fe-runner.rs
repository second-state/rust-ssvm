// Copyright (C) 2020 Second State.
// This file is part of Rust-SSVM.

// Rust-SSVM is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// Rust-SSVM is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use clap::{crate_authors, crate_version, App, Arg};
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::Read;

use rust_ssvm::create;
use rust_ssvm::host::HostContext as HostInterface;
use rust_ssvm::types::*;

struct HostContext {
    storage: BTreeMap<Bytes32, Bytes32>,
}

impl HostContext {
    fn new() -> HostContext {
        HostContext {
            storage: BTreeMap::new(),
        }
    }
}

impl HostInterface for HostContext {
    fn account_exists(&mut self, _addr: &Address) -> bool {
        println!("Host: account_exists");
        return true;
    }
    fn get_storage(&mut self, _addr: &Address, key: &Bytes32) -> Bytes32 {
        println!("Host: get_storage");
        let value = self.storage.get(key);
        let ret: Bytes32;
        match value {
            Some(value) => ret = value.to_owned(),
            None => ret = [0u8; BYTES32_LENGTH],
        }
        println!("{:?} -> {:?}", hex::encode(key), hex::encode(ret));
        return ret;
    }
    fn set_storage(&mut self, _addr: &Address, key: &Bytes32, value: &Bytes32) -> StorageStatus {
        println!("Host: set_storage");
        println!("{:?} -> {:?}", hex::encode(key), hex::encode(value));
        self.storage.insert(key.to_owned(), value.to_owned());
        return StorageStatus::EVMC_STORAGE_MODIFIED;
    }
    fn get_balance(&mut self, _addr: &Address) -> Bytes32 {
        println!("Host: get_balance");
        return [0u8; BYTES32_LENGTH];
    }
    fn get_code_size(&mut self, _addr: &Address) -> usize {
        println!("Host: get_code_size");
        return 0;
    }
    fn get_code_hash(&mut self, _addr: &Address) -> Bytes32 {
        println!("Host: get_code_hash");
        return [0u8; BYTES32_LENGTH];
    }
    fn copy_code(
        &mut self,
        _addr: &Address,
        _offset: &usize,
        _buffer_data: &*mut u8,
        _buffer_size: &usize,
    ) -> usize {
        println!("Host: copy_code");
        return 0;
    }
    fn selfdestruct(&mut self, _addr: &Address, _beneficiary: &Address) {
        println!("Host: selfdestruct");
    }
    fn get_tx_context(&mut self) -> (Bytes32, Address, Address, i64, i64, i64, Bytes32, Bytes32) {
        println!("Host: get_tx_context");
        return (
            [0u8; BYTES32_LENGTH],
            [0u8; ADDRESS_LENGTH],
            [0u8; ADDRESS_LENGTH],
            0,
            0,
            0,
            [0u8; BYTES32_LENGTH],
            [0u8; BYTES32_LENGTH],
        );
    }
    fn get_block_hash(&mut self, _number: i64) -> Bytes32 {
        println!("Host: get_block_hash");
        return [0u8; BYTES32_LENGTH];
    }
    fn emit_log(&mut self, _addr: &Address, _topics: &Vec<Bytes32>, _data: &Bytes) {
        println!("Host: emit_log");
    }
    fn call(
        &mut self,
        _kind: MessageKind,
        _destination: &Address,
        _sender: &Address,
        _value: &Bytes32,
        _input: &Bytes,
        _gas: i64,
        _depth: i32,
        _is_static: bool,
        _salt: &Bytes32,
    ) -> (Vec<u8>, i64, Address, StatusCode) {
        println!("Host: call");
        return (
            vec![0u8; BYTES32_LENGTH],
            _gas,
            [0u8; ADDRESS_LENGTH],
            StatusCode::EVMC_SUCCESS,
        );
    }
}

impl Drop for HostContext {
    fn drop(&mut self) {
        println!("Dump storage:");
        for (key, value) in &self.storage {
            println!("{:?} -> {:?}", hex::encode(key), hex::encode(value));
        }
    }
}

fn read_a_file(path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    return Ok(data);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let without_prefix = args[2].trim_start_matches("0x");
    let sig = i64::from_str_radix(without_prefix, 16).unwrap();
    let sig_arr = sig.to_be_bytes().to_vec();

    let a:i32 = (args[3]).parse().unwrap();
    let a_arr = a.to_be_bytes().to_vec();
    let b:i32 = (args[4]).parse().unwrap();
    let b_arr = b.to_be_bytes().to_vec();

    
    println!("sig:{:?} a: {:?} b: {:?}", sig_arr, a_arr, b_arr);

    let _vm = create();
    
    let msg = &[0u8; 32];
    let mut callData: Vec<u8> = vec![];

    callData.extend(&sig_arr[4..8]);

   let padding = vec![0u8; 28]; 
   callData.extend(&padding);
   callData.extend(a_arr);
   callData.extend(&padding);
   callData.extend(b_arr);
   println!("{:?}", callData);

    println!("Instantiate: {:?}", (_vm.get_name(), _vm.get_version()));
    match read_a_file(file_path) {
        Ok(code) => {
            let mut host_context = HostContext::new();
            let (output, gas_left, status_code) = _vm.execute(
                &mut host_context,
                Revision::EVMC_BYZANTIUM,
                MessageKind::EVMC_CALL,
                false,
                123,
                50000000,
                &[32u8; 20],
                &[128u8; 20],
                &callData,
                msg,
                &code[..],
                &[0u8; 32],
            );
            println!("Output:  {:?}", output);
            println!("GasLeft: {:?}", gas_left);
            println!("Status:  {:?}", status_code);
        }
        Err(e) => println!("Error load wasm file: {:?}, {:?}", file_path, e),
    }
    _vm.destroy();
}
