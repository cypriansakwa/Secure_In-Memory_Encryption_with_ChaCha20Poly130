# 🔐 Secure In-Memory Encryption with ChaCha20Poly1305

This Rust example demonstrates **authenticated encryption** using the [`ChaCha20Poly1305`](https://docs.rs/chacha20poly1305) AEAD (Authenticated Encryption with Associated Data) cipher. It securely encrypts and decrypts data entirely **in-memory**, ensuring cryptographic keys and nonces are never written to disk.

---

## 📦 Dependencies

Add these to your `Cargo.toml`:

```toml
[dependencies]
chacha20poly1305 = { version = "0.10", features = ["rand_core"] }
rand = "0.8"
hex = "0.4"
## 🔧 How It Works

### Generate Key and Nonce  
The `ChaCha20Poly1305` key (256 bits) and nonce (96 bits) are generated using a secure random number generator (`OsRng`).

### Encrypt the Data  
The plaintext is encrypted using the generated key and nonce. The resulting ciphertext includes a `Poly1305` authentication tag to ensure data integrity.

### Decrypt the Data  
The ciphertext is decrypted using the same key and nonce. The authentication tag is verified to confirm data integrity and authenticity before recovering the original message.
## 🧪 Example Output
When you run the code, you should see output like:
``` text
Ciphertext (hex): 5e3fa0...9c3d91
Decrypted: Confidential data goes here.
```
## 🔐 Security Notes

- ✅ The key and nonce **never leave memory**.
- ✅ The nonce is **unique per encryption**, as required by AEAD ciphers.
- ❌ This example **does not persist data or handle key rotation** — it's intended for secure, short-term, in-memory encryption.

### For real-world use:

- 🔑 Manage keys using a **secure key management system (KMS)**.
- 🚫 **Never reuse a nonce** with the same key.
- 🧹 Securely **zeroize sensitive data** if needed.

