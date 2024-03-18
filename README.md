# Ballot with vote controlled by token allowance

This repo contains the same ballot contract as [this repo](https://github.com/icolomina/ballot-contract) but this time the contract controls the vote using a token allowance. 
The token contract is located in the *BallotToken* folder.

## Installing soroban
Before testing the contracts, you have to install [soroban](https://stellar.org/soroban). You can follow their [step-by-step guide](https://soroban.stellar.org/docs/getting-started/setup).

## Testing the BallotToken contract
Go to the BallotToken folder and execute the following commands:

```shell
cargo test
```

This will compile the code and then execute the tests. After ensuring all tests pass, you can generate the token wasm file:

```shell
soroban contract build
```

## Testing the Ballot contract
The Ballot contract needs the token wasm file to run so, as we have generated it, we can now test the ballot contract. Go to the Ballot folder and execute the tests and we did before:

```shell
cargo test
```