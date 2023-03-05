// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the schema definition file instead

#![allow(dead_code)]

use wasmlib::*;

pub const SC_NAME        : &str = "swap";
pub const SC_DESCRIPTION : &str = "An example fake swap contract";
pub const HSC_NAME       : ScHname = ScHname(0xa64f0011);

pub const RESULT_PRICE : &str = "price";

pub const STATE_LATEST_PRICE : &str = "latestPrice";

pub const FUNC_SET_PRICE : &str = "setPrice";
pub const VIEW_GET_PRICE : &str = "getPrice";

pub const HFUNC_SET_PRICE : ScHname = ScHname(0x4dc65151);
pub const HVIEW_GET_PRICE : ScHname = ScHname(0xb645c80d);
