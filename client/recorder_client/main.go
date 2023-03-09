package main

import (
	"fmt"
	"log"

	"github.com/howjmay/wasp-contract-example/contracts/recorder/go/recorder"
	"github.com/spf13/viper"

	"github.com/iotaledger/wasp/packages/cryptolib"
	"github.com/iotaledger/wasp/packages/parameters"
	"github.com/iotaledger/wasp/packages/wasmvm/wasmclient/go/wasmclient"
	"github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib/wasmtypes"
)

func main() {
	viper.SetConfigName("wasp-cli.json")
	viper.SetConfigType("json")
	viper.AddConfigPath("../..")
	if err := viper.ReadInConfig(); err != nil {
		panic(fmt.Errorf("fatal error config file: %w", err))
	}
	var seed, chainID string
	if err := viper.UnmarshalKey("wallet.seed", &seed); err != nil {
		log.Fatal(err)
	}
	if err := viper.UnmarshalKey("chains.mychain", &chainID); err != nil {
		log.Fatal(err)
	}

	ctx := setupClient(seed, chainID)
	f := recorder.ScFuncs.GetPrice(ctx)
	f.Func.Call()
	if ctx.Err != nil {
		log.Fatal(ctx.Err)
	}
	fmt.Println("price:", f.Results.Price().Value())
}

func setupClient(seedStr, chainID string) *wasmclient.WasmClientContext {
	var params *parameters.L1Params
	err := viper.UnmarshalKey("l1.params", &params)
	if err != nil {
		log.Fatal(err)
	}
	parameters.InitL1(params)
	seed := cryptolib.NewSeedFromBytes(wasmtypes.BytesFromString(seedStr))
	wallet := cryptolib.NewKeyPairFromSeed(seed.SubSeed(0))
	svc := wasmclient.NewWasmClientService("http://localhost:9090", chainID)
	ctx := wasmclient.NewWasmClientContext(svc, recorder.ScName)
	ctx.SignRequests(wallet)
	return ctx
}
