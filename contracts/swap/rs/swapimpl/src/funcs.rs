// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasmlib::*;

use crate::*;

pub fn func_set_price(ctx: &ScFuncContext, f: &SetPriceContext) {
    let get_block_info_f = wasmlib::coreblocklog::ScFuncs::get_block_info(ctx);
    get_block_info_f.func.call();
    let block_index = get_block_info_f.results.block_index().value();
    f.results.price().set_value((block_index * 10) as u64);
}

pub fn view_get_price(ctx: &ScViewContext, f: &GetPriceContext) {
    let get_block_info_f = wasmlib::coreblocklog::ScFuncs::get_block_info(ctx);
    get_block_info_f.func.call();
    let block_index = get_block_info_f.results.block_index().value();
    f.results.price().set_value((block_index * 10) as u64);
}
