// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasmlib::*;

use crate::*;

pub fn func_update_price(ctx: &ScFuncContext, f: &UpdatePriceContext) {
    let ret = ctx.call(swap::HSC_NAME, swap::HFUNC_CALL_FOR_PRICE, None, None);
    let ret_price = wasmtypes::uint64_from_bytes(&ret.get(swap::RESULT_PRICE.as_bytes()));
    f.events.price_history(ret_price);
    f.results.success().set_value(true);
}
