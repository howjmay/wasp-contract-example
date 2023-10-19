if [ ! -f "wasp-cli.json" ]; then
    wasp-cli init
fi
set -e
set -o xtrace
wasp-cli request-funds
wasp-cli chain deploy --chain=mychain --skip-version-check
# wasp-cli chain activate --skip-version-check
wasp-cli chain deposit base:500000000 --skip-version-check
wasp-cli balance --skip-version-check
wasp-cli chain balance --skip-version-check
root_path=$(git rev-parse --show-toplevel)
wasp-cli chain deploy-contract wasmtime recorder "recorder contract" $root_path/contracts/recorder/rs/recorderwasm/pkg/recorderwasm_bg.wasm --skip-version-check
wasp-cli chain deploy-contract wasmtime swap "swap contract" $root_path/contracts/swap/rs/swapwasm/pkg/swapwasm_bg.wasm --skip-version-check
# wasp-cli chain call-view blocklog getBlockInfo | wasp-cli decode string n uint32 --skip-version-check
# wasp-cli chain call-view swap viewPrice | wasp-cli decode string price uint64 --skip-version-check
wasp-cli chain balance --skip-version-check
wasp-cli chain list-accounts --skip-version-check
wasp-cli check-versions --skip-version-check
