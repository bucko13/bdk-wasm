use bdk::bitcoin::Network;
use bdk::bitcoin::secp256k1::{Secp256k1};

use wasm_bindgen::prelude::*;

// use bdk::bitcoin::secp256k1::VerifyOnly;
// use bdk::bitcoin::secp256k1::Secp256k1;
// use std::lazy::SyncLazy;
// static SECP: SyncLazy<Secp256k1<VerifyOnly>> = SyncLazy::new(|| Secp256k1::verification_only());

use bdk::blockchain::EsploraBlockchain;
use bdk::database::MemoryDatabase;
use bdk::descriptor::policy::BuildSatisfaction;
use miniscript::{descriptor::DescriptorPublicKey, Descriptor};
use bdk::wallet::AddressIndex;
use bdk::{SyncOptions, Wallet};
use std::rc::Rc;
use bdk::wallet::signer::{SignersContainer};

use js_sys::{Promise, Error};
use wasm_bindgen_futures::future_to_promise;
use std::sync::Arc;

use bdk::descriptor::{IntoWalletDescriptor, ExtractPolicy};

#[cfg(feature = "web-sys")]
use web_sys::console;

mod utils;
use core::str::FromStr;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

//#[wasm_bindgen]
//pub fn init() {
//    console_log::init_with_level(log::Level::Debug).unwrap();
//    utils::set_panic_hook();
//
//    info!("Initialization completed");
//}

#[wasm_bindgen]
pub struct WalletWrapper {
    wallet: Rc<Wallet<MemoryDatabase>>,
    blockchain: Rc<EsploraBlockchain>,
}

#[wasm_bindgen]
impl WalletWrapper {
    #[wasm_bindgen(constructor)]
    pub async fn new(
        network: String,
        descriptor: String,
        change_descriptor: Option<String>,
        esplora: String,
        stop_gap: usize,
    ) -> Result<WalletWrapper, String> {
        let network = match network.as_str() {
            "regtest" => Network::Regtest,
            "testnet" | _ => Network::Testnet,
        };

        let blockchain = EsploraBlockchain::new(&esplora, stop_gap);
        let wallet = Wallet::new(
            descriptor.as_str(),
            change_descriptor.as_ref().map(|x| x.as_str()),
            network,
            MemoryDatabase::new(),
        )
        .map_err(|e| format!("{:?}", e))?;

        Ok(WalletWrapper {
            wallet: Rc::new(wallet),
            blockchain: Rc::new(blockchain),
        })
    }

    pub fn sync(&self) -> Promise {
        let wallet = Rc::clone(&self.wallet);
        let blockchain = Rc::clone(&self.blockchain);
        future_to_promise(async move {
            #[cfg(feature = "web-sys")]
            console::log_1(&"before sync".into());
            wallet
                .as_ref()
                .sync(blockchain.as_ref(), SyncOptions::default())
                .await
                .map_err(|e| format!("{:?}", e))?;
            #[cfg(feature = "web-sys")]
            console::log_1(&"after sync".into());
            Ok("done".into())
        })
    }

    #[wasm_bindgen]
    pub fn balance(&self) -> Result<u64, String> {
        let balance = self.wallet.get_balance().map_err(|e| format!("{:?}", e))?;
        Ok(balance)
    }

    #[wasm_bindgen]
    pub fn get_new_address(&self) -> Result<String, String> {
        let new_address = self
            .wallet
            .get_address(AddressIndex::New)
            .map_err(|e| format!("{:?}", e))?
            .address
            .to_string();
        Ok(new_address)
    }
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, bdk-wasm!");
}


#[wasm_bindgen]
pub fn extractPolicy(s: &str) -> Result<String, JsValue>  {
    // (1) Parse the descriptor from the string and find if there are any initial errors
    let desc = match Descriptor::<DescriptorPublicKey>::from_str(s) {
        Ok(desc) => desc,
        Err(err) => {
            let err = format!("Could not parse descriptor {} ", err.to_string());
            return Err(Error::new(&err.to_string()).into())
        }
    };

    let secp = Secp256k1::new();
    let wallet = Wallet::new(s, None, Network::Bitcoin, MemoryDatabase::default());

    // a closure we'll use to test the descriptor with various networks
    let get_wallet_descriptor = | network: Network |  {
        let d = desc.clone();
        d.into_wallet_descriptor(&secp, network)
    };

    // (2) try and get the wallet_descriptor but we don't know the network yet
    let (wallet_desc, keymap) = get_wallet_descriptor(Network::Bitcoin).unwrap_or_else(|e| {
            get_wallet_descriptor(Network::Testnet).unwrap_or_else(|e| {
                    get_wallet_descriptor(Network::Regtest).unwrap_or_else(|e| {
                            get_wallet_descriptor(Network::Signet).unwrap()
                    })
            })
    });

    let signers_container = Arc::new(SignersContainer::build(keymap, &wallet_desc, &secp));
    let policy = wallet_desc.extract_policy(&signers_container, BuildSatisfaction::None, &secp).unwrap().unwrap();

    Ok(serde_json::to_string_pretty(&policy).map_err(|e| e.to_string())?)
}

#[test]
fn test_stuff() {
    let testStr = "wsh(multi(2,tpubD6NzVbkrYhZ4XHndKkuB8FifXm8r5FQHwrN6oZuWCz13qb93rtgKvD4PQsqC4HP4yhV3tA2fqr2RbY5mNXfM7RxXUoeABoDtsFUq2zJq6YK/0/*,tpubD6NzVbkrYhZ4XHndKkuB8FifXm8r5FQHwrN6oZuWCz13qb93rtgKvD4PQsqC4HP4yhV3tA2fqr2RbY5mNXfM7RxXUoeABoDtsFUq2zJq6YK/1/*))";
    let mainStr = "wsh(sortedmulti(1,[9d120b19/48'/0'/0'/2']xpub6FDrnnUsgQSwRFazYbVDs9eadQaNV13f5dtQDoWrCuMNq2qgMH7GevctMAm3PeHq3KBkh9BgA8iPfaHYACHFpfueYdeAUtjjEH3vMJWEKfu/0/*,[5c9e228d/48'/0'/0'/2']xpub6EgGHjcvovyN3nK921zAGPfuB41cJXkYRdt3tLGmiMyvbgHpss4X1eRZwShbEBb1znz2e2bCkCED87QZpin3sSYKbmCzQ9Sc7LaV98ngdeX/0/*))";
    let s = testStr;
    let pol = extractPolicy(s);
    println!("{:?}", pol);
    assert!(true)
}