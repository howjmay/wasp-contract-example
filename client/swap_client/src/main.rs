use serde_json::Value;
use std::sync::Arc;
use wasmclient::isc::keypair;
use wasmclient::{WasmClientContext, WasmClientService};

fn main() {
    let cfg = read_config("../../wasp-cli.json");
    let mut ctx = setup_client(&cfg);

    let mut events = swap::eventhandlers::SwapEventHandlers::new("0");
    events.on_swap_price_log(|e: &swap::EventPriceLog| println!("receiving event: {:?}", e.price));
    let h = Box::new(events);
    ctx.register(h);
    check_error(&ctx);

    let f = swap::ScFuncs::get_price(&ctx);
    f.func.call();
    check_error(&ctx);
    println!("get price: {}", f.results.price().value());
    let f_set_price = swap::ScFuncs::set_price(&ctx);
    f_set_price.func.post();
    check_error(&ctx);
    println!("set price: {}", f_set_price.results.price().value());

    println!("waiting for event...");
    std::thread::sleep(std::time::Duration::from_secs(1));
}

fn setup_client(cfg: &Config) -> WasmClientContext {
    let svc = Arc::new(WasmClientService::new(
        "http://localhost:9090",
        &cfg.chain_id,
    ));
    let mut ctx = WasmClientContext::new(svc.clone(), swap::SC_NAME);
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

#[track_caller]
fn check_error(ctx: &WasmClientContext) {
    if let Err(e) = ctx.err() {
        println!("err: {}", e);
        assert!(false);
    }
}
