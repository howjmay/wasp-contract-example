// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the schema definition file instead

//nolint:dupl
package recorderimpl

import (
	"github.com/howjmay/wasp-contract-example/contracts/recorder/go/recorder"
	"github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib"
)

var exportMap = wasmlib.ScExportMap{
	Names: []string{
		recorder.FuncUpdatePrice,
		recorder.ViewGetPrice,
	},
	Funcs: []wasmlib.ScFuncContextFunction{
		funcUpdatePriceThunk,
	},
	Views: []wasmlib.ScViewContextFunction{
		viewGetPriceThunk,
	},
}

func OnDispatch(index int32) {
	exportMap.Dispatch(index)
}

type UpdatePriceContext struct {
	Events  recorder.RecorderEvents
	Results recorder.MutableUpdatePriceResults
}

func funcUpdatePriceThunk(ctx wasmlib.ScFuncContext) {
	ctx.Log("recorder.funcUpdatePrice")
	f := &UpdatePriceContext{
		Results: recorder.NewMutableUpdatePriceResults(),
	}
	funcUpdatePrice(ctx, f)
	ctx.Results(f.Results.Proxy)
	ctx.Log("recorder.funcUpdatePrice ok")
}

type GetPriceContext struct {
	Results recorder.MutableGetPriceResults
}

func viewGetPriceThunk(ctx wasmlib.ScViewContext) {
	ctx.Log("recorder.viewGetPrice")
	f := &GetPriceContext{
		Results: recorder.NewMutableGetPriceResults(),
	}
	viewGetPrice(ctx, f)
	ctx.Results(f.Results.Proxy)
	ctx.Log("recorder.viewGetPrice ok")
}
