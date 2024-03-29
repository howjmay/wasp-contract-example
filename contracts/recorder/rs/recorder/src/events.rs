// Code generated by schema tool; DO NOT EDIT.

#![allow(dead_code)]
#![allow(unused_mut)]

use wasmlib::*;

pub struct RecorderEvents {
}

impl RecorderEvents {

	pub fn price_history(&self,
        price: u64,
    ) {
		let mut enc = EventEncoder::new("recorder.priceHistory");
		uint64_encode(&mut enc, price);
		EventEncoder::emit(&enc);
	}
}
