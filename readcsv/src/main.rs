use anyhow::{anyhow, Result};
use base64::{engine, Engine};
use clap::Parser;
use csv::{ReaderBuilder, WriterBuilder};
use globutils::wallet;
use ledger::data_model::{TxoSID, Utxo, ASSET_TYPE_FRA};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use zei::serialization::ZeiFromToBytes;
use zei::xfr::asset_record::open_blind_asset_record;
use zei::xfr::structs::OwnerMemo;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    file: String,
    #[arg(long)]
    rpc: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    pub id: u64,
    pub ticker: String,
    pub user: String,
    pub amount: u64,
    pub price: f64,
    pub state: u8,
    pub to_user: Option<String>,
    pub center_mnemonic: String,
    pub create_time: i64,
    pub update_time: i64,
    pub center_user: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Balance {
    pub order_id: u64,
    pub state: u8,
    pub mnemonic: String,
    pub address: String,
    pub pub_key: String,
    pub balance: u64,
    pub to_user: Option<String>,
}

fn get_owned_utxos(url: &str, pub_key: &str) -> Result<HashMap<TxoSID, (Utxo, Option<OwnerMemo>)>> {
    let url = format!("{}/owned_utxos/{}", url, pub_key);
    debug!("RPC: {}", url);
    attohttpc::get(url)
        .send()
        .and_then(|resp| resp.bytes())
        .map_err(|e| anyhow! {"{:?}", e})
        .and_then(|b| {
            serde_json::from_slice::<HashMap<TxoSID, (Utxo, Option<OwnerMemo>)>>(&b)
                .map_err(|e| anyhow!("{:?}", e))
        })
}

fn get_fra_balance(url: &str, mnemonic: &str) -> u64 {
    let key_pair = wallet::restore_keypair_from_mnemonic_default(mnemonic).unwrap();
    let pub_key = wallet::public_key_to_base64(key_pair.get_pk_ref());
    let mut input_amount = 0;
    let utxos = get_owned_utxos(url, &pub_key).unwrap();
    for (_, (utxo, owner_memo)) in utxos.into_iter() {
        let oar = open_blind_asset_record(&utxo.0.record, &owner_memo, &key_pair).unwrap();
        if oar.asset_type != ASSET_TYPE_FRA || oar.amount == 0 {
            continue;
        }
        input_amount += oar.amount;
    }

    input_amount
}

fn main() {
    env_logger::init();
    info!("Starting up");

    let args = Args::parse();

    let file = File::open(&args.file).expect("open");
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

    let output_file = File::create(format!("output_{}", &args.file)).expect("create");
    let writer = BufWriter::new(output_file);
    let mut csv_writer = WriterBuilder::new().has_headers(false).from_writer(writer);
    // 写入表头
    csv_writer
        .write_record(&[
            "order_id", "state", "mnemonic", "address", "pub_key", "balance", "to_user",
        ])
        .unwrap();

    for line in csv_reader.deserialize() {
        let record: Record = line.unwrap();

        let balance = get_fra_balance(&args.rpc, &record.center_mnemonic);
        let key_pair =
            wallet::restore_keypair_from_mnemonic_default(&record.center_mnemonic).unwrap();
        let xfr_pk = key_pair.get_pk_ref();
        let address = wallet::public_key_to_bech32(xfr_pk);
        let pub_key = engine::general_purpose::URL_SAFE.encode(xfr_pk.zei_to_bytes());

        csv_writer
            .serialize(Balance {
                order_id: record.id,
                state: record.state,
                mnemonic: record.center_mnemonic,
                address,
                pub_key,
                balance,
                to_user: record.to_user,
            })
            .unwrap();
        csv_writer.flush().unwrap();
        println!("Write record: {}", record.id)
    }

    println!("Complete.")
}
