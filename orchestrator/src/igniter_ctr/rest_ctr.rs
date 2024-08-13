use reqwest::Client;

pub struct IgniterRestClient {
    address: String,
    port: i32,
    pub inner_client: Client,
    base_url: String,
}

impl IgniterRestClient {
    pub fn new(address: String, port: i32) -> IgniterRestClient {
        let client = Client::builder().build().unwrap();
        let base_url = format!("http://{}:{}", address, port);
        IgniterRestClient {
            address,
            port,
            base_url,
            inner_client: client,
        }
    }
    pub fn get_base_url(&self) -> &String {
        &self.base_url
    }
}
