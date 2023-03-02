cd recorder
schema -rs
wasm-pack build rs/recorderwasm
cd ../swap
schema -rs
wasm-pack build /rs/swapwasm