// Code generated by schema tool; DO NOT EDIT.

package recorder

import "github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib"

type UpdatePriceCall struct {
	Func    *wasmlib.ScFunc
	Results ImmutableUpdatePriceResults
}

type GetPriceCall struct {
	Func    *wasmlib.ScView
	Results ImmutableGetPriceResults
}

type Funcs struct{}

var ScFuncs Funcs

func (sc Funcs) UpdatePrice(ctx wasmlib.ScFuncClientContext) *UpdatePriceCall {
	f := &UpdatePriceCall{Func: wasmlib.NewScFunc(ctx, HScName, HFuncUpdatePrice)}
	wasmlib.NewCallResultsProxy(&f.Func.ScView, &f.Results.Proxy)
	return f
}

func (sc Funcs) GetPrice(ctx wasmlib.ScViewClientContext) *GetPriceCall {
	f := &GetPriceCall{Func: wasmlib.NewScView(ctx, HScName, HViewGetPrice)}
	wasmlib.NewCallResultsProxy(f.Func, &f.Results.Proxy)
	return f
}
