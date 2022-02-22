use std::env;

/**
 * Compares public keys from remote signer and client. Public keys in the client must match the public keys in the remote signer.
 * Returns the public keys to add and the public keys to remove
 * - If public keys are the same do nothing
 * - If there are public keys in the remote singer that are not in the client, add them to the client
 * - If there are public keys in the client that are not in the remote signer, remove them from the client
 */
pub fn compare_public_keys(public_keys_remote_vec: &Vec<String>, public_keys_client_vec: &Vec<String>) -> (Vec<String>, Vec<String>) {
    // Compare public keys
    let mut public_keys_to_add: Vec<String> = Vec::new();
    let mut public_keys_to_remove: Vec<String> = Vec::new();
    for public_key_remote in public_keys_remote_vec.iter() {
        if !public_keys_client_vec.contains(&public_key_remote) {
            public_keys_to_add.push(public_key_remote.clone());
        }
    }
    for public_key_client in public_keys_client_vec.iter() {
        if !public_keys_remote_vec.contains(&public_key_client) {
            public_keys_to_remove.push(public_key_client.clone());
        }
    }

    (public_keys_to_add, public_keys_to_remove)
}



/**
 * Check for the environment variables and panic if they are not set:
 * - RUST_ENV must equal production or development
 * - ETH2_CLIENT_API_URL
 * - WEB3SIGNER_API_URL
 */
pub fn get_env_variables() -> (String, String, String) {
    // check for RUST_ENV
    let rust_env = env::var("RUST_ENV");
    let rust_env = match rust_env {
        Ok(rust_env) => match rust_env.as_str() {
            "production" | "development" => rust_env,
            _ => panic!("RUST_ENV must be either production or development"),
        },
        Err(_) => panic!("RUST_ENV environment variable not set"),
    };

    // check for ETH2_CLIENT_API_URL
    let eth2_client_api_url = env::var("ETH2_CLIENT_API_URL");
    let eth2_client_api_url = match eth2_client_api_url {
        Ok(eth2_client_api_url) => eth2_client_api_url,
        Err(_) => panic!("ETH2_CLIENT_API_URL environment variable not set"),
    };
    
    // check for WEB3SIGNER_API_URL
    let web3signer_api_url = env::var("WEB3SIGNER_API_URL");
    let web3signer_api_url = match web3signer_api_url {
        Ok(web3signer_api_url) => web3signer_api_url,
        Err(_) => panic!("WEB3SIGNER_API_URL environment variable not set"),
    };

    (rust_env, eth2_client_api_url, web3signer_api_url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_get_public_keys_to_add() {
        let public_keys_remote_vec: Vec<String> = vec!["0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a".to_string(), "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56fjfh3jcisp9".to_string(), "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56fj4hc6s930p".to_string()];
        let public_keys_client_vec: Vec<String> = vec!["0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56fjfh3jcisp9".to_string()];

        let (public_keys_to_add, public_keys_to_remove) = compare_public_keys(&public_keys_remote_vec, &public_keys_client_vec);

        assert_eq!(public_keys_to_add, vec!["0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a".to_string(), "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56fj4hc6s930p".to_string()]);
        assert_eq!(public_keys_to_remove.len() == 0, true);
    }

    #[test]
    fn it_should_get_public_keys_to_remove() {
        let public_keys_remote_vec: Vec<String> = vec!["0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56fjfh3jcisp9".to_string()]; 
        let public_keys_client_vec: Vec<String> = vec!["0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a".to_string(), "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56fjfh3jcisp9".to_string(), "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56fj4hc6s930p".to_string()];

        let (public_keys_to_add, public_keys_to_remove) = compare_public_keys(&public_keys_remote_vec, &public_keys_client_vec);

        assert_eq!(public_keys_to_remove, vec!["0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f43611df74a".to_string(), "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56fj4hc6s930p".to_string()]);
        assert_eq!(public_keys_to_add.len() == 0, true);
    }

    #[test]
    fn it_should_get_public_keys_to_add_and_remove() {
        let public_keys_remote_vec: Vec<String> = vec!["0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f3h4ic90pol".to_string(), "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56fj4hndh746g".to_string(), "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f93jdnriopx".to_string()]; 
        let public_keys_client_vec: Vec<String> = vec!["0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f93jdnriopx".to_string(), "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f093epxk4f8".to_string(), "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f5u3hdb5op0".to_string()];

        let (public_keys_to_add, public_keys_to_remove) = compare_public_keys(&public_keys_remote_vec, &public_keys_client_vec);

        assert_eq!(public_keys_to_add, vec!["0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f3h4ic90pol".to_string(), "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56fj4hndh746g".to_string()]);
        assert_eq!(public_keys_to_remove, vec!["0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f093epxk4f8".to_string(), "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f5u3hdb5op0".to_string()]);
    }

    #[test]
    fn it_should_get_no_public_keys() {
        let public_keys_remote_vec: Vec<String> = vec!["0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f3h4ic90pol".to_string(), "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56fj4hndh746g".to_string(), "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f93jdnriopx".to_string()]; 
        let public_keys_client_vec: Vec<String> = vec!["0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f93jdnriopx".to_string(), "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56f3h4ic90pol".to_string(), "0x93247f2209abcacf57b75a51dafae777f9dd38bc7053d1af526f220a7489a6d3a2753e5f3e8b1cfe39b56fj4hndh746g".to_string()];

        let (public_keys_to_add, public_keys_to_remove) = compare_public_keys(&public_keys_remote_vec, &public_keys_client_vec);

        assert_eq!(public_keys_to_add.len() == 0, true);
        assert_eq!(public_keys_to_remove.len() == 0, true);
    }
}