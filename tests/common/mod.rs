use magiceden_rs::Client;

#[allow(dead_code)]
pub fn setup_client() -> Client {
    Client::new().with_api_key("")
}
