use rust_api_picpay::infrastructure::{
    database, 
    router
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = match database::setup().await {
        Ok(connection) => connection,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:1337").await {
        Ok(tcp) => tcp,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    let app = router::setup(pool);

    println!("✅ - Server running on 0.0.0.0:1337");

    axum::serve(listener, app).await.unwrap();
}