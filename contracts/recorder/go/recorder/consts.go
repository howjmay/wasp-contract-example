// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the schema definition file instead

package recorder

import "github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib/wasmtypes"

const (
	ScName        = "recorder"
	ScDescription = "An example recorder contract"
	HScName       = wasmtypes.ScHname(0xc8783c3c)
)

const (
	ResultPrice   = "price"
	ResultSuccess = "success"
)

const (
	FuncUpdatePrice = "updatePrice"
	ViewGetPrice    = "getPrice"
)

const (
	HFuncUpdatePrice = wasmtypes.ScHname(0xd6847598)
	HViewGetPrice    = wasmtypes.ScHname(0xb645c80d)
)