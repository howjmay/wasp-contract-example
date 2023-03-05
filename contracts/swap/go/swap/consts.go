// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
// >>>> DO NOT CHANGE THIS FILE! <<<<
// Change the schema definition file instead

package swap

import "github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib/wasmtypes"

const (
	ScName        = "swap"
	ScDescription = "An example fake swap contract"
	HScName       = wasmtypes.ScHname(0xa64f0011)
)

const (
	ResultPrice = "price"
)

const (
	StateLatestPrice = "latestPrice"
)

const (
	FuncSetPrice = "setPrice"
	ViewGetPrice = "getPrice"
)

const (
	HFuncSetPrice = wasmtypes.ScHname(0x4dc65151)
	HViewGetPrice = wasmtypes.ScHname(0xb645c80d)
)
