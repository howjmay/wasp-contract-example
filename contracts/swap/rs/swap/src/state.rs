// Code generated by schema tool; DO NOT EDIT.

#![allow(dead_code)]
#![allow(unused_imports)]

use wasmlib::*;

use crate::*;

#[derive(Clone)]
pub struct ImmutableSwapState {
    pub(crate) proxy: Proxy,
}

impl ImmutableSwapState {
    pub fn new() -> ImmutableSwapState {
        ImmutableSwapState {
            proxy: state_proxy(),
        }
    }

    pub fn latest_price(&self) -> ScImmutableUint64 {
        ScImmutableUint64::new(self.proxy.root(STATE_LATEST_PRICE))
    }
}

#[derive(Clone)]
pub struct MutableSwapState {
    pub(crate) proxy: Proxy,
}

impl MutableSwapState {
    pub fn new() -> MutableSwapState {
        MutableSwapState {
            proxy: state_proxy(),
        }
    }
    pub fn as_immutable(&self) -> ImmutableSwapState {
        ImmutableSwapState { proxy: self.proxy.root("") }
    }

    pub fn latest_price(&self) -> ScMutableUint64 {
        ScMutableUint64::new(self.proxy.root(STATE_LATEST_PRICE))
    }
}
