// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the schema definition file instead

#![allow(dead_code)]

use wasmlib::*;

use crate::*;

pub struct UpdatePriceCall<'a> {
    pub func:    ScFunc<'a>,
    pub results: ImmutableUpdatePriceResults,
}

pub struct GetPriceCall<'a> {
    pub func:    ScView<'a>,
    pub results: ImmutableGetPriceResults,
}

pub struct ScFuncs {
}

impl ScFuncs {
    pub fn update_price(ctx: &impl ScFuncCallContext) -> UpdatePriceCall {
        let mut f = UpdatePriceCall {
            func:    ScFunc::new(ctx, HSC_NAME, HFUNC_UPDATE_PRICE),
            results: ImmutableUpdatePriceResults { proxy: Proxy::nil() },
        };
        ScFunc::link_results(&mut f.results.proxy, &f.func);
        f
    }

    pub fn get_price(ctx: &impl ScViewCallContext) -> GetPriceCall {
        let mut f = GetPriceCall {
            func:    ScView::new(ctx, HSC_NAME, HVIEW_GET_PRICE),
            results: ImmutableGetPriceResults { proxy: Proxy::nil() },
        };
        ScView::link_results(&mut f.results.proxy, &f.func);
        f
    }
}
