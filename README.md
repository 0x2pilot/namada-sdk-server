# Overview
Namada Sdk Server - is a simple server that provides a set of endpoints to interact with the Namada blockchain that will be useful if you are building your own explorer or chain monitoring tool. It is written in Rust and uses the Tendermint RPC to interact with the blockchain.

# Build and start server from sources
```
cargo run
```
Note that you would need to install and initalize namada chain on your local machine for some of the endpoints to work.
# Build docker image and run from container
1. build docker image
```
docker build --platform linux/amd64 -t namada-sdk-server .
```
2. remove old container if required
```
docker rm -f namada-sdk-server
```

start container
```
docker run -e TENDERMINT_ADDR=tcp://127.0.0.1:26657 -e TENDERMINT_ADDR_HTTP=http://127.0.0.1:26657 -p 8080:8080 --name namada-sdk-server -t -d namada-sdk-server
```
3. test endpoint
```
curl http://localhost:8080/balance/tnam1qrlwpwhwm9s8fktnrlh0mspnwa5j9mklustt8a5h
```
# API Endpoints Documentation

Below is the documentation for the available API endpoints.

## Epoch Information

- **Endpoint:** `/epoch`
- **Method:** GET
- **Description:** Retrieves the current epoch information.
- **Response:** JSON object containing the epoch details.

## Validators

- **Endpoint:** `/validators/{epoch}`
- **Method:** GET
- **Description:** Fetches the list of validators for a given epoch.
- **Parameters:**
  - `epoch`: The epoch number.
- **Response:** JSON array of validators.

## Validator State

- **Endpoint:** `/validator/state/{epoch}/{validator}`
- **Method:** GET
- **Description:** Gets the state of a specific validator in a given epoch.
- **Parameters:**
  - `epoch`: The epoch number.
  - `validator`: The validator's address.
- **Response:** JSON object containing the validator's state.

## Validator Stake

- **Endpoint:** `/validator/stake/{epoch}/{validator}`
- **Method:** GET
- **Description:** Retrieves the stake amount of a specific validator in a given epoch.
- **Parameters:**
  - `epoch`: The epoch number.
  - `validator`: The validator's address.
- **Response:** Text response with the stake amount.

## Validator Metadata

- **Endpoint:** `/validator/metadata/{epoch}/{validator}`
- **Method:** GET
- **Description:** Fetches metadata for a specific validator in a given epoch.
- **Parameters:**
  - `epoch`: The epoch number.
  - `validator`: The validator's address.
- **Response:** JSON object containing the validator's metadata.

## Account Balance

- **Endpoint:** `/balance/{owner}`
- **Method:** GET
- **Description:** Retrieves the balance of a specific account.
- **Parameters:**
  - `owner`: The account owner's address.
- **Response:** Text response with the balance amount.

## Transaction Response

- **Endpoint:** `/tx/{tx_hash}`
- **Method:** GET
- **Description:** Gets the response of a specific transaction by its hash.
- **Parameters:**
  - `tx_hash`: The transaction hash.
- **Response:** JSON object containing the transaction response.

## Transaction Status

- **Endpoint:** `/tx/status/{tx_hash}`
- **Method:** GET
- **Description:** Retrieves the status of a specific transaction by its hash.
- **Parameters:**
  - `tx_hash`: The transaction hash.
- **Response:** JSON object containing the transaction status.

## Transaction Events

- **Endpoint:** `/tx/events/{tx_hash}`
- **Method:** GET
- **Description:** Fetches events related to a specific transaction by its hash.
- **Parameters:**
  - `tx_hash`: The transaction hash.
- **Response:** JSON object containing the transaction events.

## Governance Parameters

- **Endpoint:** `/governance/parameters`
- **Method:** GET
- **Description:** Retrieves the governance parameters.
- **Response:** JSON object containing the governance parameters.

## Proposal by ID

- **Endpoint:** `/proposal/{proposal_id}`
- **Method:** GET
- **Description:** Fetches a specific proposal by its ID.
- **Parameters:**
  - `proposal_id`: The proposal ID.
- **Response:** JSON object containing the proposal details.

## Proposal Votes

- **Endpoint:** `/proposal/votes/{proposal_id}`
- **Method:** GET
- **Description:** Retrieves votes for a specific proposal by its ID.
- **Parameters:**
  - `proposal_id`: The proposal ID.
- **Response:** JSON array of votes.

## Total Staked Tokens

- **Endpoint:** `/total-staked/{epoch}`
- **Method:** GET
- **Description:** Gets the total amount of tokens staked in a given epoch.
- **Parameters:**
  - `epoch`: The epoch number.
- **Response:** Text response with the total staked amount.

## Latest Proposal ID

- **Endpoint:** `/latest-proposal-id`
- **Method:** GET
- **Description:** Retrieves the ID of the latest proposal.
- **Response:** Text response with the latest proposal ID.