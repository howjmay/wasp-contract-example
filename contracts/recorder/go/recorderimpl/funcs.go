package recorderimpl

import (
	"github.com/howjmay/wasp-contract-example/contracts/swap/go/swap"
	"github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib"
	"github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib/wasmtypes"
)

func funcUpdatePrice(ctx wasmlib.ScFuncContext, f *UpdatePriceContext) {
	ret := ctx.Call(swap.HScName, swap.HFuncSetPrice, nil, nil)
	retPrice := wasmtypes.Uint64FromBytes(ret.Get([]byte(swap.ResultPrice)))
	f.Events.PriceHistory(retPrice)
	f.Results.Success().SetValue(true)
}

func viewGetPrice(ctx wasmlib.ScViewContext, f *GetPriceContext) {
	ret := ctx.Call(swap.HScName, swap.HViewGetPrice, nil)
	retPrice := wasmtypes.Uint64FromBytes(ret.Get([]byte(swap.ResultPrice)))
	f.Results.Price().SetValue(retPrice)
}
