// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package test

import (
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/howjmay/wasp-contract-example/contracts/recorder/go/recorder"
	"github.com/iotaledger/wasp/packages/wasmvm/wasmsolo"
)

func TestDeploy(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, recorder.ScName, recorder.OnDispatch)
	require.NoError(t, ctx.ContractExists(recorder.ScName))
}
