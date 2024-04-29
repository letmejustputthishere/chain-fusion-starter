mod eth_logs;
mod evm_rpc;
mod evm_signer;
mod fees;
mod guard;
mod job;
mod lifecycle;
mod state;
mod storage;
mod transactions;
mod utils;

use std::time::Duration;

use eth_logs::scrape_eth_logs;

use ic_canisters_http_types::{HttpRequest, HttpResponse, HttpResponseBuilder};
use ic_cdk::println;
use lifecycle::InitArg;
use state::read_state;

use crate::state::{mutate_state, State, STATE};

pub const SCRAPING_LOGS_INTERVAL: Duration = Duration::from_secs(3 * 60);

fn setup_timers() {
    // // Start scraping logs immediately after the install, then repeat with the interval.
    ic_cdk_timers::set_timer(Duration::ZERO, || ic_cdk::spawn(scrape_eth_logs()));
    ic_cdk_timers::set_timer_interval(SCRAPING_LOGS_INTERVAL, || ic_cdk::spawn(scrape_eth_logs()));
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
}

#[ic_cdk::init]
fn init(arg: InitArg) {
    println!("[init]: initialized minter with arg: {:?}", arg);
    STATE.with(|cell| {
        *cell.borrow_mut() = Some(State::try_from(arg).expect("BUG: failed to initialize minter"))
    });
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
    transactions::transfer_eth_from_canister(value, to).await;
}

#[ic_cdk::query]
fn http_request(req: HttpRequest) -> HttpResponse {
    // disable update calls for this method
    if ic_cdk::api::data_certificate().is_none() {
        ic_cdk::trap("update call rejected");
    }

    // check if the asset is in the stable memory
    if let Some(asset) = storage::get_asset(&req.path().to_string()) {
        let mut response_builder = HttpResponseBuilder::ok();

        // Apply headers
        for (key, value) in asset.headers {
            response_builder = response_builder.header(key, value);
        }

        // Set body and content length, then build the response
        response_builder
            .with_body_and_content_length(asset.body)
            .build()
    } else {
        // return 404
        HttpResponseBuilder::not_found().build()
    }
}

// Enable Candid export, read more [here](https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid/)
ic_cdk::export_candid!();
