use mongodb::{ options::{ ClientOptions, Credential }, Client };
use std::env;

pub async fn get_mongo_client() -> Client {
    // Load environment variables
    dotenv::dotenv().ok();

    let username = env::var("DB_USERNAME").expect("DB_USERNAME must be set");
    let password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("DB_PORT").unwrap_or_else(|_| "27017".to_string());
    let auth_source = env::var("DB_AUTH_SOURCE").unwrap_or_else(|_| "admin".to_string());

    // Build the credential options
    let credential = Credential::builder()
        .username(Some(username))
        .password(Some(password))
        .source(Some(auth_source))
        .build();

    // Build the client options
    let client_uri = format!("mongodb://{}:{}", host, port);
    let mut options = ClientOptions::parse(client_uri).await.unwrap();
    options.credential = Some(credential);

    Client::with_options(options).unwrap()
}
