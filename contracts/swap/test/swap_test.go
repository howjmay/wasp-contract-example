// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package test

import (
	"testing"

	"github.com/stretchr/testify/require"

	"???ers/hauyang/Work/wasp-contract-example/swap/go/swap"
	"github.com/iotaledger/wasp/packages/wasmvm/wasmsolo"
)

func TestDeploy(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, swap.ScName, swap.OnDispatch)
	require.NoError(t, ctx.ContractExists(swap.ScName))
}
