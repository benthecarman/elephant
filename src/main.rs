mod app;
mod app_wallet;

use app::App;
use app_wallet::AppWallet;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
