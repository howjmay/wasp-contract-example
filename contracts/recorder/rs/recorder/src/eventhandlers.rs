// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the schema definition file instead

use std::collections::HashMap;
use wasmlib::*;

use crate::*;

pub struct RecorderEventHandlers {
    id: String,
    recorder_handlers: HashMap<&'static str, fn(evt: &RecorderEventHandlers, msg: &Vec<String>)>,

    price_history: Box<dyn Fn(&EventPriceHistory)>,
}

impl IEventHandlers for RecorderEventHandlers {
    fn call_handler(&self, topic: &str, params: &Vec<String>) {
        if let Some(handler) = self.recorder_handlers.get(topic) {
            handler(self, params);
        }
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}

unsafe impl Send for RecorderEventHandlers {}
unsafe impl Sync for RecorderEventHandlers {}

impl RecorderEventHandlers {
    pub fn new(id: &str) -> RecorderEventHandlers {
        let mut handlers: HashMap<&str, fn(evt: &RecorderEventHandlers, msg: &Vec<String>)> = HashMap::new();
        handlers.insert("recorder.priceHistory", |e, m| { (e.price_history)(&EventPriceHistory::new(m)); });
        return RecorderEventHandlers {
            id: id.to_string(),
            recorder_handlers: handlers,
            price_history: Box::new(|_e| {}),
        };
    }

    pub fn on_recorder_price_history<F>(&mut self, handler: F)
        where F: Fn(&EventPriceHistory) + 'static {
        self.price_history = Box::new(handler);
    }
}

pub struct EventPriceHistory {
    pub timestamp: u64,
    pub price: u64,
}

impl EventPriceHistory {
    pub fn new(msg: &Vec<String>) -> EventPriceHistory {
        let mut evt = EventDecoder::new(msg);
        EventPriceHistory {
            timestamp: evt.timestamp(),
            price: uint64_from_string(&evt.decode()),
        }
    }
}