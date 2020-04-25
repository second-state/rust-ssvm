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

mod host;
pub use host::*;
mod loader;
use evmc_sys as ffi;
pub use loader::EvmcLoaderErrorCode;
use std::ffi::CStr;

extern "C" {
    fn evmc_create() -> *mut ffi::evmc_instance;
}

fn error(err: EvmcLoaderErrorCode) -> Result<EvmcLoaderErrorCode, &'static str> {
    match err {
        EvmcLoaderErrorCode::EvmcLoaderSucces => Ok(EvmcLoaderErrorCode::EvmcLoaderSucces),
        EvmcLoaderErrorCode::EvmcLoaderCannotOpen => Err("evmc loader: library cannot open"),
        EvmcLoaderErrorCode::EvmcLoaderSymbolNotFound => {
            Err("evmc loader: the EVMC create function not found")
        }
        EvmcLoaderErrorCode::EvmcLoaderInvalidArgument => {
            panic!("evmc loader: filename argument is invalid")
        }
        EvmcLoaderErrorCode::EvmcLoaderInstanceCreationFailure => {
            Err("evmc loader: VM instance creation failure")
        }
        EvmcLoaderErrorCode::EvmcLoaderAbiVersionMismatch => {
            Err("evmc loader: ABI version mismatch")
        }
        _ => Err("evmc loader: unexpected error"),
    }
}

pub struct EvmcVm {
    handle: *mut ffi::evmc_instance,
    evmc_context: *mut ffi::evmc_context,
}

impl EvmcVm {
    pub fn destroy(&self) {
        unsafe { ((*self.handle).destroy.unwrap())(self.handle) }
    }

    pub fn get_name(&self) -> &str {
        unsafe {
            let c_str: &CStr = CStr::from_ptr((*self.handle).name);
            c_str.to_str().unwrap()
        }
    }

    pub fn get_version(&self) -> &str {
        unsafe {
            let c_str: &CStr = CStr::from_ptr((*self.handle).version);
            c_str.to_str().unwrap()
        }
    }

    pub fn has_capability(&self, capability: ffi::evmc_capabilities) -> bool {
        unsafe {
            std::mem::transmute::<ffi::evmc_capabilities, u32>(capability)
                == ((*self.handle).get_capabilities.unwrap())(self.handle)
        }
    }

    pub fn execute(
        &self,
        host_context: Box<dyn HostInterface>,
        rev: ffi::evmc_revision,
        kind: ffi::evmc_call_kind,
        is_static: bool,
        depth: i32,
        gas: i64,
        destination: &[u8; 20],
        sender: &[u8; 20],
        input: &[u8],
        value: &[u8; 32],
        code: &[u8],
        create2_salt: &[u8; 32],
    ) -> (&[u8], i64, StatusCode) {
        host::set_host_context(Some(host_context));
        let evmc_destination = ffi::evmc_address {
            bytes: *destination,
        };
        let evmc_sender = ffi::evmc_address { bytes: *sender };
        let evmc_value = ffi::evmc_uint256be { bytes: *value };
        let evmc_create2_salt = ffi::evmc_bytes32 {
            bytes: *create2_salt,
        };
        let mut evmc_flags: u32 = 0;
        unsafe {
            if is_static {
                evmc_flags |=
                    std::mem::transmute::<ffi::evmc_flags, u32>(ffi::evmc_flags::EVMC_STATIC);
            }
        }
        let evmc_message = Box::into_raw(Box::new({
            ffi::evmc_message {
                kind: kind,
                flags: evmc_flags,
                depth: depth,
                gas: gas,
                destination: evmc_destination,
                sender: evmc_sender,
                input_data: input.as_ptr(),
                input_size: input.len(),
                value: evmc_value,
                create2_salt: evmc_create2_salt,
            }
        }));

        unsafe {
            let result = ((*self.handle).execute.unwrap())(
                self.handle,
                self.evmc_context,
                rev,
                evmc_message,
                code.as_ptr(),
                code.len(),
            );
            host::set_host_context(None);
            return (
                std::slice::from_raw_parts(result.output_data, result.output_size),
                result.gas_left,
                result.status_code,
            );
        }
    }
}

fn get_evmc_context() -> ffi::evmc_context {
    ffi::evmc_context {
        host: Box::into_raw(Box::new(ffi::evmc_host_interface {
            account_exists: Some(host::account_exists),
            get_storage: Some(host::get_storage),
            set_storage: Some(host::set_storage),
            get_balance: Some(host::get_balance),
            get_code_size: Some(host::get_code_size),
            get_code_hash: Some(host::get_code_hash),
            copy_code: None,
            selfdestruct: Some(host::selfdestruct),
            call: Some(host::call),
            get_tx_context: Some(host::get_tx_context),
            get_block_hash: Some(host::get_block_hash),
            emit_log: Some(host::emit_log),
        })),
    }
}

pub fn load(fname: &str) -> (EvmcVm, Result<EvmcLoaderErrorCode, &'static str>) {
    let (instance, ec) = loader::evmc_load_and_create(fname);
    (
        EvmcVm {
            handle: instance,
            evmc_context: Box::into_raw(Box::new(get_evmc_context())),
        },
        error(ec),
    )
}

pub fn create() -> EvmcVm {
    unsafe {
        EvmcVm {
            handle: evmc_create(),
            evmc_context: Box::into_raw(Box::new(get_evmc_context())),
        }
    }
}
