# NearDIDRegistry

A smart contract implemented in Rust for the NEAR Protocol that functions as a decentralized identifier (DID) registry. It allows ownership management, delegation, attribute assignment, and nonce tracking for DIDs represented by base58-encoded Ed25519 public keys.

## 🔐 Cryptographic Foundation

* **Signature Algorithm:** Ed25519 (used externally for signing/verifying JWTs)
* **Key Encoding:**

  * Base58 encoding for public keys in `did:near:<base58>` format
  * Base64url encoding in JWT proofs
* **Hashing and Storage:** Data keys are tuples of strings and `Vec<u8>`, stored using `LookupMap` from `near_sdk`

## 🧠 Core Concepts

* Each identity is a `String` (base58 public key or NEAR account).
* Delegates and attributes are valid for a given duration (in seconds).
* Only the current owner of an identity can modify its state.

## 📦 Contract Storage

| Field        | Description                                      |
| ------------ | ------------------------------------------------ |
| `owners`     | Maps identity to current owner (also a `String`) |
| `delegates`  | Maps (identity, type, delegate) to expiration    |
| `attributes` | Maps (identity, name, value) to expiration       |
| `changed`    | Maps identity to block height of last change     |
| `nonce`      | Monotonic counter for off-chain use (signatures) |

---

## ⚙️ Public Methods

### `identity_owner(identity: String) -> String`

Returns the current owner of a DID. Defaults to self-owned if not registered.

### `change_owner(identity: String, new_owner: String)`

Changes the owner of a DID. Only callable by current owner.

### `add_delegate(identity: String, delegate_type: String, delegate: String, validity_secs: u64)`

Registers a delegate for a DID, valid for the given duration.

### `revoke_delegate(identity: String, delegate_type: String, delegate: String)`

Revokes a delegate by setting expiration to `0`.

### `valid_delegate(identity: String, delegate_type: String, delegate: String) -> bool`

Returns `true` if the delegate is still valid.

### `set_attribute(identity: String, name: String, value: Vec<u8>, validity_secs: u64)`

Assigns an attribute (e.g. public key, service endpoint) to a DID.

### `revoke_attribute(identity: String, name: String, value: Vec<u8>)`

Revokes an attribute by setting its expiration to `0`.

### `valid_attribute(identity: String, name: String, value: Vec<u8>) -> bool`

Returns `true` if the attribute is still valid.

### `get_nonce(identity: String) -> u64`

Returns the current nonce of the identity.

### `increment_nonce(identity: String)`

Increments the nonce by 1. Useful for signed interactions.

### `get_changed(identity: String) -> u64`

Returns the block height of the last change made to the identity.

---

## 🧪 Example cURL Calls (via RPC)

> Replace `your-contract.testnet` and `identity` accordingly.

### 🔍 View owner

```bash
curl -X POST https://rpc.testnet.near.org \
  -H 'Content-Type: application/json' \
  -d '{
    "jsonrpc": "2.0",
    "id": "dontcare",
    "method": "query",
    "params": {
      "request_type": "call_function",
      "finality": "final",
      "account_id": "your-contract.testnet",
      "method_name": "identity_owner",
      "args_base64": "eyJpZGVudGl0eSI6ICJkaWQ6bmVhcjpDaGVjaw=="  
    }
  }'
```

> `args_base64` is the base64-encoded JSON: `{ "identity": "did:near:Check" }`

### 🔐 Change owner (via Near CLI)

```bash
near call your-contract.testnet change_owner '{"identity": "did:near:Check", "new_owner": "did:near:NewKey"}' --accountId your-account.testnet
```

### ➕ Add delegate

```bash
near call your-contract.testnet add_delegate '{"identity": "did:near:Check", "delegate_type": "veriKey", "delegate": "did:near:OtherKey", "validity_secs": 3600}' --accountId your-account.testnet
```

### 🗑️ Revoke attribute

```bash
near call your-contract.testnet revoke_attribute '{"identity": "did:near:Check", "name": "did/pub/Ed25519/veriKey/base64", "value": "a2V5VmFsdWU="}' --accountId your-account.testnet
```

> `value` should be base64-encoded.

---

## ✅ Example DID Document (based on registry)

```json
{
  "@context": "https://w3id.org/did/v1",
  "id": "did:near:CF5Ri...",
  "verificationMethod": [
    {
      "id": "did:near:CF5Ri...#owner",
      "type": "Ed25519VerificationKey2018",
      "controller": "did:near:CF5Ri...",
      "publicKeyBase58": "CF5RiJYh4EVmEt8UAD..."
    }
  ],
  "authentication": ["did:near:CF5Ri...#owner"],
  "assertionMethod": ["did:near:CF5Ri...#owner"]
}
```

---

## 🧠 Notes

* Designed for interoperability with `did-jwt`, `JwtProof2020`, and NEAR-based wallets.
* Suitable for creating lightweight DID registries using only Ed25519 keys.
* Compatible with `did:near:<base58PublicKey>` and optionally `example.testnet` as identity.
