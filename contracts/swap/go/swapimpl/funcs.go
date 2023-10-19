// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package swapimpl

import (
	"github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib"
	"github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib/coreblocklog"
)

func funcSetPrice(ctx wasmlib.ScFuncContext, f *SetPriceContext) {
	getBlockInfoFunc := coreblocklog.ScFuncs.GetBlockInfo(ctx)
	getBlockInfoFunc.Func.Call()
	blockIndex := uint64(getBlockInfoFunc.Results.BlockIndex().Value())
	price := blockIndex * 10
	f.Events.PriceLog(blockIndex, price)
	f.Results.Price().SetValue(price)
}

func viewGetPrice(ctx wasmlib.ScViewContext, f *GetPriceContext) {
	getBlockInfoFunc := coreblocklog.ScFuncs.GetBlockInfo(ctx)
	getBlockInfoFunc.Func.Call()
	blockIndex := getBlockInfoFunc.Results.BlockIndex().Value()
	f.Results.Price().SetValue(uint64(blockIndex * 10))
}
