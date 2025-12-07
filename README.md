# bitcoin-ecc-secp256k1

A minimal, educational implementation of elliptic curve cryptography (ECC) and the secp256k1 curve used in Bitcoin

> This repository accompanies the article:
> **Bitcoin & Elliptic Curves for Developers**
> and demonstrates how secp256k1 works under the hood.

---

## What is this project?

This repository provides a clean and simple Rust implementation of:

* Finite fields arithmetic
* Elliptic curve points
* The secp256k1 domain parameters
* Point addition & doubling
* Scalar multiplication (`k * G`)
* Key generation (private & public keys)
* Basic ECDSA signature demo (optional depending on what you include)

The goal is **education**, not production use.
It helps developers understand why Bitcoin chose secp256k1 and how its math works.

---

## Motivation

Bitcoin relies on elliptic curve cryptography to:

* Generate secure key pairs
* Sign transactions
* Verify signatures on the blockchain

Understanding secp256k1 gives you a deeper appreciation of Bitcoin’s security model.

This project explains the math behind:

* finite fields
* modular arithmetic
* elliptic curve addition
* scalar multiplication
* digital signatures

---

## Installation

Clone the project:

```bash
git clone https://github.com/nejos97/bitcoin-ecc-secp256k1.git
cd bitcoin-ecc-secp256k1
```

Run the demo:

```bash
cargo run
```
