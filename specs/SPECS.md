# 📘 NEAR DID Specification

This document outlines the specification and implementation details of the `did:near` method used in this project, including how it complies with the W3C DID Core standard and its integration into Web3 identity flows.

---

## 🆔 DID Method: `did:near`

The `did:near` method allows for decentralized identifiers based on NEAR blockchain accounts or directly on Ed25519 public keys encoded in base58.

### Supported Formats

1. **Named Account**
   ```
   did:near:alice.testnet
   ```

   Represents a valid NEAR account ID (testnet or mainnet).

2. **Base58 Public Key**
   ```
   did:near:7YzVXb8... (44-50 base58 characters)
   ```

   Represents a raw Ed25519 public key used as a DID.

---

## 📄 DID Document Structure

Example:

```json
{
  "@context": "https://w3id.org/did/v1",
  "id": "did:near:alice.testnet",
  "verificationMethod": [
    {
      "id": "did:near:alice.testnet#owner",
      "type": "Ed25519VerificationKey2018",
      "controller": "did:near:alice.testnet",
      "publicKeyBase58": "3gfD2z7q..."
    }
  ],
  "authentication": [
    "did:near:alice.testnet#owner"
  ],
  "assertionMethod": [
    "did:near:alice.testnet#owner"
  ]
}
```

---

## 🔗 Compliance with W3C DID Core

This implementation complies with [W3C DID Core 1.0](https://www.w3.org/TR/did-core/) through:

- Standard context (`@context`)
- Unique `id` representing the DID
- Proper key management via `verificationMethod`
- Support for `authentication` and `assertionMethod`
- `publicKeyBase58` with Ed25519 encoding (`Ed25519VerificationKey2018`)

---

## 🔐 DID Resolution

Resolution is handled via a smart contract on NEAR:

- Contract ID: `neardidregistry.testnet`
- Function: `identity_owner(did:near:...)`
- If the DID is a named account, it is resolved using NEAR RPC (`account.state()`)
- If the DID is base58, it's resolved from the registry contract

---

## 🧾 Key Characteristics

- ✅ Self-sovereign: Users own and control their keys
- ✅ Compatible with NEAR wallets & accounts
- ✅ Compatible with Web3 credential frameworks (e.g., `did-jwt-vc`)
- ✅ Supports on-chain proof of issuance via ProofType contract

---

## 🌐 Interoperability

- Fully interoperable with any system that supports DIDs and VCs
- Can be used as issuer or subject in any Verifiable Credential
- Can be resolved in web, mobile, or backend via standard resolver logic

---

## 📦 Example Libraries

- [`did-near-resolver`](https://github.com/DTI-web3/did-near-resolver)
- [`@kaytrust/prooftypes`](https://www.npmjs.com/package/@kaytrust/prooftypes)
- `near-api-js`

---

## 📚 License

MIT — Use freely with attribution.
