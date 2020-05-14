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

use crate::host;
use crate::loader::{evmc_load_and_create, EvmcLoaderErrorCode};
use crate::types::*;
use evmc_sys as ffi;
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
    pub fn get_abi_version(&self) -> i32 {
        unsafe {
            let version: i32 = (*self.handle).abi_version;
            version
        }
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

    pub fn destroy(&self) {
        unsafe { ((*self.handle).destroy.unwrap())(self.handle) }
    }

    pub fn execute(
        &self,
        host_context: Box<dyn host::HostInterface>,
        rev: Revision,
        kind: CallKind,
        is_static: bool,
        depth: i32,
        gas: i64,
        destination: &Address,
        sender: &Address,
        input: &Bytes,
        value: &Bytes32,
        code: &Bytes,
        create2_salt: &Bytes32,
    ) -> (&Bytes, i64, StatusCode) {
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

    pub fn has_capability(&self, capability: Capabilities) -> bool {
        unsafe {
            std::mem::transmute::<Capabilities, u32>(capability)
                == ((*self.handle).get_capabilities.unwrap())(self.handle)
        }
    }
}

pub fn load(fname: &str) -> (EvmcVm, Result<EvmcLoaderErrorCode, &'static str>) {
    let (instance, ec) = evmc_load_and_create(fname);
    (
        EvmcVm {
            handle: instance,
            evmc_context: Box::into_raw(Box::new(host::get_evmc_context())),
        },
        error(ec),
    )
}

pub fn create() -> EvmcVm {
    unsafe {
        EvmcVm {
            handle: evmc_create(),
            evmc_context: Box::into_raw(Box::new(host::get_evmc_context())),
        }
    }
}
