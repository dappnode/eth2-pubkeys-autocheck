use serde::{ Deserialize };

// Remote signer API:
// - Docs: https://consensys.github.io/web3signer/web3signer-eth2.html#tag/Keymanager
// - Instance must be created with the base URL of the remote signer (i.e https://web3signer.example.com)
// - There is added to the base url the path: /eth/v1/keystores

impl RemoteSignerRequest {
    pub fn new(base_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.clone(),
            url: base_url + "/eth/v1/keystores"
        }
    }

    pub fn parse_public_keys(&self, validators: &GetRemoteSignerValidators) -> Vec<String> {
        let mut validators_vec: Vec<String> = Vec::new();
        for validator in validators.data.iter() {
            validators_vec.push(validator.validating_pubkey.clone());
        }
        validators_vec
    }

    /**
     * GET public keys: https://consensys.github.io/web3signer/web3signer-eth2.html#operation/KEYMANAGER_LIST
     */
    pub async fn get_public_keys(&self) -> Result<GetRemoteSignerValidators, String> {
        let client = reqwest::Client::new();
        let response = client.get(&self.url).send().await.expect("Failed to get public keys from remote signer");
        Ok(response.json::<GetRemoteSignerValidators>().await.expect("Failed to parse public keys from remote signer"))
    }
}

pub struct RemoteSignerRequest {
    pub client: reqwest::Client,
    pub url: String,
    pub base_url: String
}

#[derive(Deserialize, Debug)]
pub struct GetRemoteSignerValidators {
    data: Vec<PublicKeysRemoteSigner>
}

#[derive(Deserialize, Debug)]
pub struct PublicKeysRemoteSigner {
    validating_pubkey: String,
    derivation_path: String,
    readonly: bool
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
            when.path("/eth/v1/keystores")
                .method(GET);
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({"data": [{"validating_pubkey": "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a","derivation_path": "m/44'/60'/0'/0/0","readonly": true}, {"validating_pubkey": "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56fjfh3jcisp9","derivation_path": "m/12381/3600/0/0/0","readonly": true}]}));
        });

        let url = format!("http://{}", server.address());
        let remote_client = RemoteSignerRequest::new(url);

        // GET 

        let public_keys = remote_client.get_public_keys().await.unwrap();
        let validators = remote_client.parse_public_keys(&public_keys);
        assert_eq!(validators[0], "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a");
        assert_eq!(validators[1], "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56fjfh3jcisp9");
    }
}