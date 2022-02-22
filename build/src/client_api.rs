use serde::{ Deserialize, Serialize};

// Eth2 client API:
// - Docs: https://ethereum.github.io/keymanager-APIs/#/
// - Instance must be created with the base URL of the eth2 client API (i.e http://localhost:9000 or http://validator.lighthouse-prater.dappnode:9000)
// - There is added to the base url the path: /eth/v1/remotekeys

#[derive(Debug)]
pub struct Eth2ClientRequest {
    pub client: reqwest::Client,
    pub base_url: String,
    pub url: String
}

impl Eth2ClientRequest {
    pub fn new(base_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.clone(),
            url: base_url + "/eth/v1/remotekeys"
        }
    }

    /**
     * GET public keys: https://ethereum.github.io/keymanager-APIs/#/Remote%20Key%20Manager/ListRemoteKeys
     */
    pub async fn get_public_keys(&self) -> Result<Eth2ClientGetResponse, String> {
        let response = self.client.get(&self.url).send().await.expect("Failed to get public keys from eth2 client");
        Ok(response.json::<Eth2ClientGetResponse>().await.expect("Failed to parse public keys from eth2 client"))
    }

    pub fn parse_public_keys(&self, public_keys: &Eth2ClientGetResponse) -> Vec<String> {
        let mut validators: Vec<String> = Vec::new();
        for validator in public_keys.data.iter() {
            validators.push(validator.pubkey.clone());
        }
        validators
    }

    /**
     * POST public keys: https://ethereum.github.io/keymanager-APIs/#/Remote%20Key%20Manager/ImportRemoteKeys
     */
    pub async fn post_public_keys(&self, post_request: &Eth2ClientPostRequest) -> Result<Eth2ClientPostResponse, String> {
        let response = self.client.post(&self.url).header("content-type", "application/json").json(&post_request).send().await.expect("Failed to post public keys to eth2 client");
        Ok(response.json::<Eth2ClientPostResponse>().await.expect("Failed to parse post response from eth2 client"))
    }

    pub fn create_post_request(&self, public_keys: &Vec<String>) -> Eth2ClientPostRequest {
        let mut post_request: Eth2ClientPostRequest  = Eth2ClientPostRequest {
            remote_keys: Vec::new()
        };
        for public_key in public_keys.iter() {
            post_request.remote_keys.push(Eth2ClientPostRequestData {
                pubkey: public_key.clone(),
                url: self.base_url.clone()
            });
        }
        post_request
    }

    /**
     * DELETE public keys: https://ethereum.github.io/keymanager-APIs/#/Remote%20Key%20Manager/DeleteRemoteKeys
     */
    pub fn create_delete_request(&self, public_keys: &Vec<String>) -> Eth2ClientDeleteRequest {
        let mut delete_request: Eth2ClientDeleteRequest  = Eth2ClientDeleteRequest {
            pubkeys: Vec::new()
        };
        for public_key in public_keys.iter() {
            delete_request.pubkeys.push(public_key.clone());
        }
        delete_request
    }

    pub async fn delete_public_keys(&self, delete_request: &Eth2ClientDeleteRequest) -> Result<Eth2ClientDeleteResponse, String> {
        let response = self.client.delete(&self.url).header("content-type", "application/json").json(&delete_request).send().await.expect("Failed to delete public keys from eth2 client");
        Ok(response.json::<Eth2ClientDeleteResponse>().await.expect("Failed to parse delete response from eth2 client"))
    }

}

#[derive(Debug, Deserialize, Serialize)]
pub struct Eth2ClientGetResponse {
    data: Vec<Eth2ClientGetResponseData>,
}

#[derive( Debug, Deserialize, Serialize)]
pub struct Eth2ClientGetResponseData {
    pubkey: String,
    url: String,
    readonly: bool
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Eth2ClientPostRequest {
    pub remote_keys: Vec<Eth2ClientPostRequestData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Eth2ClientPostRequestData {
    pub pubkey: String,
    pub url: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Eth2ClientPostResponse {
    data: Vec<Eth2ClientPostResponseData>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Eth2ClientPostResponseData {
    status: String,
    message: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Eth2ClientDeleteRequest {
    pubkeys: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Eth2ClientDeleteResponse {
    data: Vec<Eth2ClientDeleteResponseData>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Eth2ClientDeleteResponseData {
    status: String,
    message: String
}


#[cfg(test)]
mod tests {
    use httpmock::prelude::*;
    use serde_json::{json};
    use super::*;

    #[tokio::test]
    async fn it_works_with_httpmock() {
        let server: MockServer = MockServer::start();
        server.mock(|when, then| {
            when.path("/eth/v1/remotekeys")
                .method(GET);
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({"data": [{"pubkey": "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a","url": "https://remote.signer","readonly": true}]}));
        });

        server.mock(|when, then| {
            when.path("/eth/v1/remotekeys")
                .header("content-type", "application/json")
                .json_body(json!({"remote_keys": [{"pubkey": "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a","url": "https://remote.signer"}]}))
                .method(POST);
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({"data": [{"status": "imported","message": "string"}]}));
      });

        server.mock(|when, then| {
            when.path("/eth/v1/remotekeys")
                .header("content-type", "application/json")
                .json_body(json!({"pubkeys": ["0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a"]}))
                .method(DELETE);
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({"data": [{"status": "deleted","message": "string"}]}));
        });

        let url = format!("http://{}", server.address());
        let eth2_client = Eth2ClientRequest::new(url);

        // GET 

        let public_keys = eth2_client.get_public_keys().await.unwrap();
        let validators = eth2_client.parse_public_keys(&public_keys);
        assert_eq!(validators[0], "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a");

        // POST

        let post_request: Eth2ClientPostRequest = Eth2ClientPostRequest {
            remote_keys: vec![Eth2ClientPostRequestData {
                pubkey: "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a".to_string(),
                url: "https://remote.signer".to_string()
            }]
        };

        let post_public_keys = eth2_client.post_public_keys(&post_request).await.unwrap();
        assert_eq!(post_public_keys.data[0].status, "imported");

        // DELETE

        let delete_request: Eth2ClientDeleteRequest = Eth2ClientDeleteRequest {
            pubkeys: vec!["0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a".to_string()]
        };

        let delete_public_keys = eth2_client.delete_public_keys(&delete_request).await.unwrap();
        assert_eq!(delete_public_keys.data[0].status, "deleted");
    }
}