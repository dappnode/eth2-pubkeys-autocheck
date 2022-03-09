## Auto check remote keys in Eth2 clients

:warning: This can only be used on Eth2 clients with [remote keymanager API](https://ethereum.github.io/keymanager-APIs/) implemented. Track eth2 clients progress at https://github.com/dappnode/DAppNode/issues/389

Eth2 clients does not have a way to auto-update public keys changes made in the remote signer, either imports and deletes. For this reason, eth2 clients should have implemented a way to update their public keys to the public keys loaded in the web3signer. To achieve this there are several approaches.

### Functionality

Check, compare and update remote keys between remote signer and eth2 client following the API standards:

- [Remote signer API](https://consensys.github.io/web3signer/web3signer-eth2.html#tag/Keymanager)
- [Eth2 client API keymanager](https://ethereum.github.io/keymanager-APIs/#/Remote%20Key%20Manager)

1. Check: fetches public keys from remote signer API and Eth2 client API
2. Compare: compare both public keys
3. Update: update public keys in the Eth2 client to be up to date with the remote signer, by importing and deleting.

### How to use it

This repository creates a binary to be used either in the Eth2 client validator container or as a separated service in the same `compose`. It is intended to be executed every period of time, usually using tools such as `cronjob`.

It must be executed with the following environment variables `CLIENT_ADDRESS`, `CLIENT_PORT` and `NETWORK`

i.e

```
CLIENT_ADDRESS=http://validator.prysm-prater.dappnode CLIENT_PORT=9000 NETWORK=prater ./auto-check-remote-keys
```

### Development environment

Run the compose development file with web3signer and a client to test the binary

```
docker-compose -f docker-compose.development.yml --env-file=./build/.env up -d
```

### References

- [Web3signer API docs](https://consensys.github.io/web3signer/)
- [Remote signer EPI](https://eips.ethereum.org/EIPS/eip-3030)
- [Client remote keymanager API](https://ethereum.github.io/keymanager-APIs/#/Remote%20Key%20Manager)
