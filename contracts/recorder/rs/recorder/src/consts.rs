// Code generated by schema tool; DO NOT EDIT.

#![allow(dead_code)]

use wasmlib::*;

pub const SC_NAME        : &str = "recorder";
pub const SC_DESCRIPTION : &str = "An example recorder contract";
pub const HSC_NAME       : ScHname = ScHname(0xc8783c3c);

pub const RESULT_PRICE   : &str = "price";
pub const RESULT_SUCCESS : &str = "success";

pub const FUNC_UPDATE_PRICE : &str = "updatePrice";
pub const VIEW_GET_PRICE    : &str = "getPrice";

pub const HFUNC_UPDATE_PRICE : ScHname = ScHname(0xd6847598);
pub const HVIEW_GET_PRICE    : ScHname = ScHname(0xb645c80d);
