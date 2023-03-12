use std::sync::Arc;
use wasmclient::isc::keypair;
use wasmclient::{WasmClientContext, WasmClientService};
// These should be the same as the configurations in wasp-cli.json
const MYCHAIN: &str = "tst1pq3z7j5kvjmc8xff9mrywpwv6tf5z8p8juvwsp85sqc9af5h2emdcn9c6p9";
const MYSEED: &str = "0xa580555e5b84a4b72bbca829b4085a4725941f3b3702525f36862762d76c21f3";

fn main() {
    let mut ctx = setup_client();

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

#[track_caller]
fn check_error(ctx: &WasmClientContext) {
    if let Err(e) = ctx.err() {
        println!("err: {}", e);
        assert!(false);
    }
}
