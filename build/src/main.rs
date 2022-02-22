use reqwest::Error;
// IMPORT MODULES:
mod client_api;
mod remote_api;
mod utils;

// TODO
// - Finish dockerfile and compose
// - finish readme
// - setup gha
// - create  github repository

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Starting cronjob");

    // Get environment variables
    let (rust_env, eth2_client_api_url, web3signer_api_url) = utils::get_env_variables();

    if rust_env == "production" {
        println!("Running in production mode");

        // Fetch public keys from remote signer
        let remote_client = remote_api::RemoteSignerRequest::new(web3signer_api_url);
        let public_keys_remote = remote_client.get_public_keys().await.unwrap();
        let public_keys_remote_vec = remote_client.parse_public_keys(&public_keys_remote);
        
        // Fetch public keys from eth2 client
        let eth2_client = client_api::Eth2ClientRequest::new(eth2_client_api_url);
        let public_keys_client = eth2_client.get_public_keys().await.unwrap();
        let public_keys_client_vec = eth2_client.parse_public_keys(&public_keys_client);

        // Compare public keys
        let (public_keys_to_add, public_keys_to_remove) = utils::compare_public_keys(&public_keys_remote_vec, &public_keys_client_vec);

        // Add public keys to eth2 client
        if public_keys_to_add.len() > 0 {
            println!("Public keys to add: {:?}", public_keys_to_add);
            let post_request = eth2_client.create_post_request(&public_keys_to_add);
            eth2_client.post_public_keys(&post_request).await.unwrap();
        } else {
            println!("No public keys to add");
        }

        // Remove public keys from eth2 client
        if public_keys_to_remove.len() > 0 {
            println!("Public keys to remove: {:?}", public_keys_to_remove);
            let delete_request = eth2_client.create_delete_request(&public_keys_to_remove);
            eth2_client.delete_public_keys(&delete_request).await.unwrap();
        } else {
            println!("No public keys to remove");
        }
    } else {
        println!("Running in development mode");
    }
    
    println!("Finished cronjon");
    Ok(())
}
