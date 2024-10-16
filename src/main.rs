use std::{env, net::SocketAddr, path::Path};

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use bdk::{
    bitcoin::Network,
    blockchain::{self, ElectrumBlockchain},
    database::SqliteDatabase,
    descriptor,
    electrum_client::Client,
    wallet::AddressIndex,
    SyncOptions, Wallet,
};

#[derive(serde::Serialize)]
struct AddressResponse {
    address: String,
    index: u32,
}

struct AppError(anyhow::Error);

fn setup() -> String {
    println!("Hi, initializing your foodiebites wallet!");
    dotenv::from_filename(".env").ok();
    dotenv::dotenv().ok();

    let descriptor = env::var("WALLET_DESCRIPTOR");
    match descriptor {
        Ok(descriptor) => descriptor,
        Err(_) => "Error".to_string(),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let descriptor = setup();
    let db_path: &Path = Path::new("foodiecat.db");
    let wallet = Wallet::new(
        &descriptor,
        None,
        Network::Testnet,
        SqliteDatabase::new(db_path),
    )?;

    let app = Router::new().route("/", get(handler));
    println!("Listening on {}", "3000");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    // let blockchain =
    //     ElectrumBlockchain::from(Client::new("ssl://electrum.blockstream.info:60002")?);

    // wallet.sync(&blockchain, SyncOptions::default())?;

    // let balance = wallet.get_balance()?;

    // dbg!(&balance);

    // let address = wallet.get_address(AddressIndex::New)?;

    // dbg!(&address);

    Ok(())
}

async fn handler() -> Result<impl IntoResponse, AppError> {
    let response = AddressResponse {
        address: "test".to_string(),
        index: 0,
    };
    Ok(Json(response))
}

// impl<E> From<E> for AppError
// where
//     E: Into<anyhow: Error>,
// {
//     fn from(err: E) -> Self {
//         Self(err.into())
//     }
// }

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", self.0),
        )
            .into_response()
    }
}
