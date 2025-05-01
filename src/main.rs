use chacha20poly1305::aead::{Aead, AeadCore, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305}; // OrinocoStreamCipher
use rand::rngs::OsRng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- 1. Key and nonce are generated securely in-memory ---
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);

    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits

    // --- 2. Data to encrypt (can be any byte slice) ---
    let plaintext = b"Confidential data goes here.";
    
    // --- 3. Encrypt securely in memory ---
    let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref())
        .expect("encryption failure!");

    println!("Ciphertext (hex): {}", hex::encode(&ciphertext));

    // --- 4. Decrypt (same key/nonce in memory) ---
    let decrypted_plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())
        .expect("decryption failure!");

    println!("Decrypted: {}", String::from_utf8_lossy(&decrypted_plaintext));

    Ok(())
}
