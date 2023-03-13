use serde_json::Value;
use std::sync::Arc;
use wasmclient::isc::keypair;
use wasmclient::{WasmClientContext, WasmClientService};

fn main() {
    let cfg = read_config("../../wasp-cli.json");
    let ctx = setup_client(&cfg);

    let f = recorder::ScFuncs::get_price(&ctx);
    f.func.call();
    check_error(&ctx);
    println!("price: {}", f.results.price().value());
}

fn setup_client(cfg: &Config) -> WasmClientContext {
    let svc = Arc::new(WasmClientService::new(
        "http://localhost:9090",
        &cfg.chain_id,
    ));
    let mut ctx = WasmClientContext::new(svc.clone(), recorder::SC_NAME);
    ctx.sign_requests(&keypair::KeyPair::from_sub_seed(
        &wasmlib::bytes_from_string(&cfg.seed),
        0,
    ));
    check_error(&ctx);
    return ctx;
}

struct Config {
    chain_id: String,
    seed: String,
}

fn read_config(path: &str) -> Config {
    let content = match std::fs::read_to_string(path) {
        Ok(v) => v,
        Err(e) => {
            panic!("err: {}", e);
        }
    };
    let v: Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            panic!("err: {}", e);
        }
    };
    let mut ret = Config {
        chain_id: v["chains"]["mychain"].to_string(),
        seed: v["wallet"]["seed"].to_string(),
    };
    ret.chain_id = ret.chain_id[1..ret.chain_id.len() - 1].to_string();
    ret.seed = ret.seed[1..ret.seed.len() - 1].to_string();
    return ret;
}

fn check_error(ctx: &WasmClientContext) {
    if let Err(e) = ctx.err() {
        println!("err: {}", e);
        assert!(false);
    }
}
