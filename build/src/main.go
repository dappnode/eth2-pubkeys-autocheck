package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"os"
	"strings"

	"github.com/robfig/cron/v3"
)

// Struct that will be used to parse the response from the web3signer
type Web3SignerResponseGet struct {
	Data []struct {
		ValidatingPubkey string `json:"validating_pubkey"`
		DerivationPath string `json:"derivation_path"`
		Readonly bool `json:"readonly"`
	} `json:"data"`
}

// Struct that will be used to parse the response from the client
type ClientResponseGet struct {
	Data []struct {
		Pubkey string `json:"pubkey"`
		Url string `json:"url"`
		Readonly bool `json:"readonly"`	
	} `json:"data"`
}

// Client DELETE response
type ClientResponseDelete struct {
	Data []struct {
		Status string `json:"status"`
		Message string `json:"message"`
	} `json:"data"`
}

// Client POST response
type ClientResponsePost struct {
	Data []struct {
		Status string `json:"status"`
		Message string `json:"message"`
	} `json:"data"`
}

// Client DELETE request
type ClientRequestDelete struct {
	Pubkeys []string `json:"pubkeys"`
}

// Client POST request
type ClientRequestPost struct {
	RemoteKeys []struct {
		Pubkey string `json:"pubkey"`
		Url string `json:"url"`
	} `json:"remote_keys"`
}

func main() {
	clientAddress := os.Getenv("CLIENT_ADDRESS")
	if clientAddress == "" {
		log.Fatal("Error: CLIENT_ADDRESS must be set, i.e http://validator.prysm-prater.dappnode")
		os.Exit(1)
	}

	clientPort := os.Getenv("CLIENT_PORT")
	if clientPort == "" {
		log.Fatal("Error: CLIENT_PORT must be set, i.e 9000")
		os.Exit(1)
	}

	// Get the web3signer url
	var web3signerUrl string
	network := os.Getenv("NETWORK")
	if network == "mainnet" {
		web3signerUrl = "https://web3signer.web3signer.dappnode/eth/v1/keystores"
	} else if network == "prater" {
		web3signerUrl = "https://web3signer.web3signer-prater.dappnode/eth/v1/keystores"
	}else {
		log.Fatal("Error: NETWORK must be set to 'mainnet' or 'prater'")
		os.Exit(1)
	}

	// Get the client url
	clientUrl := fmt.Sprintf("http://%s:%s/eth/v1/keystores", clientAddress, clientPort)

	fmt.Printf("Starting cron with web3signer url %s and client url %s", web3signerUrl, clientUrl)

	c := cron.New()
	c.AddFunc("@every 1m", func() {
		// Get the list of keystores from the web3signer
		web3signerResponse, err := getWeb3signerKeystores(web3signerUrl)
		if err != nil {
			log.Fatal(err)
			return
		}

		// Get the list of keystores from the client
		clientResponseGet, err := getClientKeystores(clientUrl) 
		if err != nil {
			log.Fatal(err)
			return
		}

		// Compare the public keys from the client and the web3signer
		// If the keystore is not in the client, it will be added
		// If the keystore is in the client but not in the web3signer, it will be deleted
		// If the keystore is in both, it will be ignored
		pubKeysToAdd, pubKeysToDelete := compareKeystores(web3signerResponse, clientResponseGet)

		// If there are keys to add, add them to the client
		if len(pubKeysToAdd) > 0 {
			err := postClientKeystores(clientUrl, pubKeysToAdd)
			if err != nil {
				log.Fatal(err)
				return
			}
		} else {
			fmt.Println("No keys to add")
		}

		// If there are keys to delete, delete them from the client
		if len(pubKeysToDelete) > 0 {
			err := deleteClientKeystores(clientUrl, pubKeysToDelete)
			if err != nil {
				log.Fatal(err)
				return
			}
		} else {
			fmt.Println("No keys to delete")
		}
	})

	c.Start()
	select {}
}

func getClientKeystores(url string) (ClientResponseGet, error) {
	// Make the GET request to the url
	resp, err := http.Get(url)
	if err != nil {
		return ClientResponseGet{}, err
	}

	// Read the response body
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return ClientResponseGet{}, err
	}

	// Parse the response body
	var clientResponse ClientResponseGet
	err = json.Unmarshal(body, &clientResponse)
	if err != nil {
		return ClientResponseGet{}, err
	}

	return clientResponse, nil
}

func getWeb3signerKeystores(url string) (Web3SignerResponseGet, error) {
	// Make the GET request to the url
	resp, err := http.Get(url)
	if err != nil {
		return Web3SignerResponseGet{}, err
	}

	// Read the response body
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return Web3SignerResponseGet{}, err
	}

	// Parse the response body
	var web3signerResponse Web3SignerResponseGet
	err = json.Unmarshal(body, &web3signerResponse)
	if err != nil {
		return Web3SignerResponseGet{}, err
	}

	return web3signerResponse, nil
}


func postClientKeystores(url string, pubKeysToAdd []string) error {
	// Create the request body
	var clientRequest ClientRequestPost
	for _, pubKey := range pubKeysToAdd {
		clientRequest.RemoteKeys = append(clientRequest.RemoteKeys, struct {
			Pubkey string `json:"pubkey"`
			Url string `json:"url"`
		}{
			Pubkey: pubKey,
			Url: "",
		})
	}

	// Marshal the request body
	reqBody, err := json.Marshal(clientRequest)
	if err != nil {
		return err
	}

	// Create the request
	req, err := http.NewRequest("POST", url, strings.NewReader(string(reqBody)))
	if err != nil {
		return err
	}

	// Set the request headers
	req.Header.Set("Content-Type", "application/json")

	// Make the request
	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return err
	}

	// Read the response body
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return err
	}

	// Parse the response body
	var clientResponse ClientResponsePost
	err = json.Unmarshal(body, &clientResponse)
	if err != nil {
		return err
	}

	// Print the response
	fmt.Println(clientResponse)
	return nil
}

func deleteClientKeystores(url string, pubKeysToDelete []string) error {
	// Create the request body
	var clientRequest ClientRequestDelete
	for _, pubKey := range pubKeysToDelete {
		clientRequest.Pubkeys = append(clientRequest.Pubkeys, pubKey)
	}

	// Marshal the request body
	reqBody, err := json.Marshal(clientRequest)
	if err != nil {
		return err
	}

	// Create the request
	req, err := http.NewRequest("DELETE", url, strings.NewReader(string(reqBody)))
	if err != nil {
		return err
	}

	// Set the request headers
	req.Header.Set("Content-Type", "application/json")

	// Make the request
	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return err
	}

	// Read the response body
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return err
	}

	// Parse the response body
	var clientResponse ClientResponsePost
	err = json.Unmarshal(body, &clientResponse)
	if err != nil {
		return err
	}

	// Print the response
	fmt.Println(clientResponse)
	return nil
}

func compareKeystores(web3signerResponse Web3SignerResponseGet, clientResponse ClientResponseGet) ([]string, []string) {
	// Create the list of keys to add
	pubKeysToAdd := []string{}

	// Create the list of keys to delete
	pubKeysToDelete := []string{}

	web3signerPubKeys := []string{}
	for _, web3signerPubKey := range web3signerResponse.Data {
		web3signerPubKeys = append(web3signerPubKeys, web3signerPubKey.ValidatingPubkey)
	}

	clientPubKeys := []string{}
	for _, clientPubKey := range clientResponse.Data {
		clientPubKeys = append(clientPubKeys, clientPubKey.Pubkey)
	}

	for _, web3signerPubKey := range web3signerPubKeys {
		if !stringInSlice(web3signerPubKey, clientPubKeys) {
			pubKeysToAdd = append(pubKeysToAdd, web3signerPubKey)
		}
	}

	for _, clientPubKey := range clientPubKeys {
		if !stringInSlice(clientPubKey, web3signerPubKeys) {
			pubKeysToDelete = append(pubKeysToDelete, clientPubKey)
		}
	}

	return pubKeysToAdd, pubKeysToDelete
}

func stringInSlice(a string, list []string) bool {
	for _, b := range list {
		if b == a {
			return true
		}
	}
	return false
}