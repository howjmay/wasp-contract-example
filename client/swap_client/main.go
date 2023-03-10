package main

import (
	"fmt"
	"log"
	"time"

	"github.com/howjmay/wasp-contract-example/contracts/swap/go/swap"
	"github.com/spf13/viper"

	"github.com/iotaledger/wasp/packages/cryptolib"
	"github.com/iotaledger/wasp/packages/parameters"
	"github.com/iotaledger/wasp/packages/wasmvm/wasmclient/go/wasmclient"
	"github.com/iotaledger/wasp/packages/wasmvm/wasmlib/go/wasmlib/wasmtypes"
)

func main() {
	log.SetFlags(log.LstdFlags | log.Lshortfile)
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

	events := &swap.SwapEventHandlers{}
	events.OnSwapPriceLog(func(e *swap.EventPriceLog) {
		fmt.Println("receiving event: ", e)
	})
	ctx.Register(events)
	if ctx.Err != nil {
		log.Fatal(ctx.Err)
	}

	fGetPrice := swap.ScFuncs.GetPrice(ctx)
	fGetPrice.Func.Call()
	if ctx.Err != nil {
		log.Fatal(ctx.Err)
	}
	fmt.Println("GetPrice result:", fGetPrice.Results.Price().Value())

	fSetPrice := swap.ScFuncs.SetPrice(ctx)
	fSetPrice.Func.Post()
	if ctx.Err != nil {
		log.Fatal(ctx.Err)
	}
	// client side cant get result from a `func` function
	fmt.Println("SetPrice result:", fSetPrice.Results.Price().Value())

	fmt.Println("wait for event...")
	time.Sleep(1000 * time.Millisecond)
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
	ctx := wasmclient.NewWasmClientContext(svc, swap.ScName)
	ctx.SignRequests(wallet)
	return ctx
}
