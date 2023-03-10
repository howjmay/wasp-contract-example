// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the schema definition file instead

use std::collections::HashMap;
use wasmlib::*;

use crate::*;

pub struct SwapEventHandlers {
    id: String,
    swap_handlers: HashMap<&'static str, fn(evt: &SwapEventHandlers, msg: &Vec<String>)>,

    price_log: Box<dyn Fn(&EventPriceLog)>,
}

impl IEventHandlers for SwapEventHandlers {
    fn call_handler(&self, topic: &str, params: &Vec<String>) {
        if let Some(handler) = self.swap_handlers.get(topic) {
            handler(self, params);
        }
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}

unsafe impl Send for SwapEventHandlers {}
unsafe impl Sync for SwapEventHandlers {}

impl SwapEventHandlers {
    pub fn new(id: &str) -> SwapEventHandlers {
        let mut handlers: HashMap<&str, fn(evt: &SwapEventHandlers, msg: &Vec<String>)> = HashMap::new();
        handlers.insert("swap.priceLog", |e, m| { (e.price_log)(&EventPriceLog::new(m)); });
        return SwapEventHandlers {
            id: id.to_string(),
            swap_handlers: handlers,
            price_log: Box::new(|_e| {}),
        };
    }

    pub fn on_swap_price_log<F>(&mut self, handler: F)
        where F: Fn(&EventPriceLog) + 'static {
        self.price_log = Box::new(handler);
    }
}

pub struct EventPriceLog {
    pub timestamp: u64,
    pub block_num: u64,
    pub price: u64,
}

impl EventPriceLog {
    pub fn new(msg: &Vec<String>) -> EventPriceLog {
        let mut evt = EventDecoder::new(msg);
        EventPriceLog {
            timestamp: evt.timestamp(),
            block_num: uint64_from_string(&evt.decode()),
            price: uint64_from_string(&evt.decode()),
        }
    }
}