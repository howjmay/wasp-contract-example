use std::sync::Arc;
use wasmclient::isc::keypair;
use wasmclient::{WasmClientContext, WasmClientService};

const MYCHAIN: &str = "atoi1pq3v5c0mvgtzrjaegx3n6lf2dv9qn23rvtcj8vatn6m94laezrz8z5rtwnr";
const MYSEED: &str = "0xa580555e5b84a4b72bbca829b4085a4725941f3b3702525f36862762d76c21f3";

fn main() {
    let ctx = setup_client();
    let f = recorder::ScFuncs::update_price(&ctx);
    f.func.call();
    check_error(&ctx);
}

fn setup_client() -> WasmClientContext {
    let svc = Arc::new(WasmClientService::new("http://localhost:19090", MYCHAIN));
    let mut ctx = WasmClientContext::new(svc.clone(), "recorder");
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