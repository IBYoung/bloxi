# Bloxi [![Build Status](https://travis-ci.org/lloydmeta/bloxi.svg?branch=master)](https://travis-ci.org/lloydmeta/bloxi)

A Blockchain implementation in Rust, based on the [Learn Blockchains by Building One](https://hackernoon.com/learn-blockchains-by-building-one-117428612f46)
tutorial.

Mostly done as an exercise

## How to run

`RUST_LOG=INFO PORT=8082 cargo run --release`

* `RUST_LOG` controls the log level
* `PORT` controls the port (defaults to 8088) the server runs at.

### Endpoints

* `GET /id`: Returns the randomly generated node id
* `GET /chain`: Returns the current block chain

* `POST /block`: Returns the block (e.g. `{"height": 2}`)

* `POST /transaction`: Adds a Transaction (e.g. `{"from": 112, "to": 3, "amount": 100,"index":1,"fee":123}`)
* `POST /mine`: Mines a block based on the current transactions
* `POST /node`: Adds a node (e.g. `{ "address": "http://127.0.0.1:8081" }`)
* `POST /reconcile`: "Reconciles" the local chain with all known nodes


## Explored

- Blockchains (duh)
- Actix actor model
  - Typed actors are sweeeet
  - A great way to eliminate blocking on a shared mutable resource
  - Integrates pretty well with `Future`s
- Actix web model
  - Quite easy to use with Serde
  - No coupling with Actix actors from what I've seen (good)
  
## Todo

Lots to do, since this is a pretty faithful reproduction of the tutorial.

- Optimise (aka get rid of `.clone`s and `Box`es thrown in anger)
- Gossiping data (nodes, reconciling)
- Verifying Transactions are sound (currently just dumb data keeping)
- Moar tests