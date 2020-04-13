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

use rust_ssvm::*;

struct HostContext;
impl HostInterface for HostContext {
    fn account_exists(&self, _addr: &[u8; 20]) -> bool {
        println!("callback account_exists");
        return true;
    }
    fn get_storage(&self, _addr: &Address, _key: &Bytes32) -> Bytes32 {
        println!("callback get_storage");
        return [0u8; BYTES32_LENGTH];
    }

    fn set_storage(&self, _addr: &Address, _key: &Bytes32, _value: &Bytes32) -> StorageStatus {
        println!("callback set_storage");
        return StorageStatus::EVMC_STORAGE_UNCHANGED;
    }
    fn get_balance(&self, _addr: &Address) -> Bytes32 {
        println!("callback get_balance");
        return [0u8; BYTES32_LENGTH];
    }
    fn get_code_size(&self, _addr: &Address) -> usize {
        println!("callback get_code_size");
        return 0;
    }
    fn get_code_hash(&self, _addr: &Address) -> Bytes32 {
        println!("callback get_code_hash");
        return [0u8; BYTES32_LENGTH];
    }
    fn selfdestruct(&self, _addr: &Address, _beneficiary: &Address) {
        println!("callback selfdestruct");
    }
    fn get_tx_context(&self) -> (Bytes32, Address, Address, i64, i64, i64, Bytes32) {
        println!("callback get_tx_context");
        return (
            [0u8; BYTES32_LENGTH],
            [0u8; ADDRESS_LENGTH],
            [0u8; ADDRESS_LENGTH],
            0,
            0,
            0,
            [0u8; BYTES32_LENGTH],
        );
    }
    fn get_block_hash(&self, _number: i64) -> Bytes32 {
        println!("callback get_block_hash");
        return [0u8; BYTES32_LENGTH];
    }
    fn emit_log(&self, _addr: &Address, _topics: &Vec<Bytes32>, _data: &Bytes) {
        println!("callback emit_log");
    }
    fn call(
        &self,
        _kind: CallKind,
        _destination: &Address,
        _sender: &Address,
        _value: &Bytes32,
        _input: &[u8],
        _gas: i64,
        _depth: i32,
        _is_static: bool,
    ) -> (Vec<u8>, i64, Address, StatusCode) {
        println!("callback call");
        return (
            vec![0u8],
            0,
            [0u8; ADDRESS_LENGTH],
            StatusCode::EVMC_SUCCESS,
        );
    }
}

fn main() {
    // let lib_path = "Choose specific .so path";
    // let (_vm, _result) = load(lib_path);
    let _vm = create();

    println!("{:?}", (_vm.get_name(), _vm.get_version()));

    let host_context = HostContext;
    _vm.execute(
        Box::new(host_context),
        Revision::EVMC_BYZANTIUM,
        CallKind::EVMC_CALL,
        false,
        123,
        456,
        &[32u8; 20],
        &[128u8; 20],
        &[0u8; 0],
        &[0u8; 32],
        &[b'\0', b'a', b's', b'm'],
        &[0u8; 32],
    );
    _vm.destroy();
}
