root_path=$(git rev-parse --show-toplevel)
cd $root_path/contracts/recorder
schema -rs
wasm-pack build rs/recorderwasm
cd $root_path/contracts/swap
schema -rs -go
wasm-pack build rs/swapwasm