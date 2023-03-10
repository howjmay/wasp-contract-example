// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the schema definition file instead

//nolint:dupl
package swapimpl

import (
	"github.com/howjmay/wasp-contract-example/contracts/swap/go/swap"
	"github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib"
)

var exportMap = wasmlib.ScExportMap{
	Names: []string{
		swap.FuncSetPrice,
		swap.ViewGetPrice,
	},
	Funcs: []wasmlib.ScFuncContextFunction{
		funcSetPriceThunk,
	},
	Views: []wasmlib.ScViewContextFunction{
		viewGetPriceThunk,
	},
}

func OnDispatch(index int32) {
	exportMap.Dispatch(index)
}

type SetPriceContext struct {
	Events  swap.SwapEvents
	Results swap.MutableSetPriceResults
	State   swap.MutableSwapState
}

func funcSetPriceThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("swap.funcSetPrice")
	f := &SetPriceContext{
		Results: swap.NewMutableSetPriceResults(),
		State:   swap.NewMutableSwapState(),
	}
	funcSetPrice(ctx, f)
	ctx.Results(f.Results.Proxy)
	ctx.Log("swap.funcSetPrice ok")
}

type GetPriceContext struct {
	Results swap.MutableGetPriceResults
	State   swap.ImmutableSwapState
}

func viewGetPriceThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("swap.viewGetPrice")
	f := &GetPriceContext{
		Results: swap.NewMutableGetPriceResults(),
		State:   swap.NewImmutableSwapState(),
	}
	viewGetPrice(ctx, f)
	ctx.Results(f.Results.Proxy)
	ctx.Log("swap.viewGetPrice ok")
}
