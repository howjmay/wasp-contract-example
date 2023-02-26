// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the schema definition file instead

use recorderimpl::*;
use wasmvmhost::*;

#[no_mangle]
fn on_call(index: i32) {
    WasmVmHost::connect();
    on_dispatch(index);
}

#[no_mangle]
fn on_load() {
    WasmVmHost::connect();
    on_dispatch(-1);
}
