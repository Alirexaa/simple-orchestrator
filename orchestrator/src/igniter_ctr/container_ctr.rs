use crate::resource::container_resource::ContainerResource;

use super::rest_ctr::IgniterRestClient;

pub struct ContainerRestCtr {
    rest_client: IgniterRestClient,
}

impl ContainerRestCtr {
    pub fn new(client: IgniterRestClient) -> ContainerRestCtr {
        ContainerRestCtr {
            rest_client: client,
        }
    }
    pub async fn run_container(&self, conitaner: ContainerResource) {
        let request_url = format!("{}/v1/containers/run", self.rest_client.get_base_url());
        let response = self
            .rest_client
            .inner_client
            .post(request_url)
            .body(conitaner.as_json_str())
            .send()
            .await
            .unwrap();
        println!("Status: {}", response.status());
    }

    pub async fn list_containers(&self) {
        let request_url = format!("{}/v1/containers/list", self.rest_client.get_base_url());
        let response = self
            .rest_client
            .inner_client
            .get(request_url)
            .send()
            .await
            .unwrap();

        let status = response.status();
        println!("Status: {}", status);
        let body = response.text().await.unwrap();
        println!("Body: {}", body);
    }
}
