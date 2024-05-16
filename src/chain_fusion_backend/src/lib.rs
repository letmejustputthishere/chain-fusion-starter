mod eth_logs;
mod evm_rpc;
mod evm_signer;
mod fees;
mod guard;
mod job;
mod lifecycle;
mod state;
// mod storage;
mod transactions;
mod utils;

use std::time::Duration;

use eth_logs::scrape_eth_logs;

use ic_cdk::println;
use lifecycle::InitArg;
use state::read_state;

use crate::state::{initialize_state, mutate_state};

pub const SCRAPING_LOGS_INTERVAL: Duration = Duration::from_secs(3 * 60);

fn setup_timers() {
    // as timers are synchronous, we need to spawn a new async task to get the public key
    ic_cdk_timers::set_timer(Duration::ZERO, || {
        ic_cdk::spawn(async {
            let public_key = evm_signer::get_public_key().await;
            let evm_address = evm_signer::pubkey_bytes_to_address(&public_key);
            mutate_state(|s| {
                s.ecdsa_pub_key = Some(public_key);
                s.evm_address = Some(evm_address);
            });
        })
    });
    // // Start scraping logs almost immediately after the install, then repeat with the interval.
    ic_cdk_timers::set_timer(Duration::from_secs(10), || ic_cdk::spawn(scrape_eth_logs()));
    ic_cdk_timers::set_timer_interval(SCRAPING_LOGS_INTERVAL, || ic_cdk::spawn(scrape_eth_logs()));
}

#[ic_cdk::init]
fn init(arg: InitArg) {
    println!("[init]: initialized minter with arg: {:?}", arg);
    initialize_state(state::State::try_from(arg).expect("BUG: failed to initialize minter"));
    setup_timers();
}

#[ic_cdk::query]
fn get_evm_address() -> String {
    read_state(|s| s.evm_address.clone()).expect("evm address should be initialized")
}

#[ic_cdk::update]
async fn transfer_eth(value: u128, to: String) {
    if !ic_cdk::api::is_controller(&ic_cdk::caller()) {
        ic_cdk::trap("only the controller can send transactions");
    }
    println!("transfer_eth: value={}, to={}", value, to);
    transactions::transfer_eth(value, to).await;
}

// uncomment this if you need to serve stored assets from `storage.rs` via http requests

// #[ic_cdk::query]
// fn http_request(req: HttpRequest) -> HttpResponse {
//     if let Some(asset) = get_asset(&req.path().to_string()) {
//         let mut response_builder = HttpResponseBuilder::ok();

//         for (name, value) in asset.headers {
//             response_builder = response_builder.header(name, value);
//         }

//         response_builder
//             .with_body_and_content_length(asset.body)
//             .build()
//     } else {
//         HttpResponseBuilder::not_found().build()
//     }
// }

// Enable Candid export, read more [here](https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid/)
ic_cdk::export_candid!();
