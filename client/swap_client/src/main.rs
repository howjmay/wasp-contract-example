use std::sync::Arc;
use wasmclient::isc::keypair;
use wasmclient::{WasmClientContext, WasmClientService};

// These should be the same as the configurations in wasp-cli.json
const MYCHAIN: &str = "tst1pqw7k03vt0sm5qnalcns08x9zzftfgx4zm0lptsrnt3tnnc9lc3e5tp6mam";
const MYSEED: &str = "0xa580555e5b84a4b72bbca829b4085a4725941f3b3702525f36862762d76c21f3";

fn main() {
    let ctx = setup_client();
    let f = swap::ScFuncs::get_price(&ctx);
    f.func.call();
    check_error(&ctx);
    println!("price: {}", f.results.price().value());
    let f_set_price = swap::ScFuncs::set_price(&ctx);
    f_set_price.func.call();
    check_error(&ctx);
    println!("set price: {}", f_set_price.results.price().value());
}

fn setup_client() -> WasmClientContext {
    let svc = Arc::new(WasmClientService::new("http://localhost:9090", MYCHAIN));
    let mut ctx = WasmClientContext::new(svc.clone(), swap::SC_NAME);
    ctx.sign_requests(&keypair::KeyPair::from_sub_seed(
        &wasmlib::bytes_from_string(MYSEED),
        0,
    ));
    check_error(&ctx);
    return ctx;
}

fn check_error(ctx: &WasmClientContext) {
    if let Err(e) = ctx.err() {
        println!("err: {}", e);
        assert!(false);
    }
}
