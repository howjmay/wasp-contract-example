if [ ! -f "wasp-cli.json" ]; then
    wasp-cli init
fi
set -e
set -o xtrace
wasp-cli request-funds
wasp-cli chain deploy --chain=mychain
# wasp-cli chain activate
wasp-cli chain deposit base:500000000
wasp-cli balance
wasp-cli chain balance
root_path=$(git rev-parse --show-toplevel)
wasp-cli chain deploy-contract wasmtime recorder "recorder contract" $root_path/contracts/recorder/rs/recorderwasm/pkg/recorderwasm_bg.wasm
wasp-cli chain deploy-contract wasmtime swap "swap contract" $root_path/contracts/swap/rs/swapwasm/pkg/swapwasm_bg.wasm
# wasp-cli chain call-view blocklog getBlockInfo | wasp-cli decode string n uint32
# wasp-cli chain call-view swap viewPrice | wasp-cli decode string price uint64
wasp-cli chain balance
wasp-cli chain list-accounts
wasp-cli check-versions
