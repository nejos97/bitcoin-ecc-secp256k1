use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use rand::{rng, RngCore};
use sha2::{Sha256, Digest};

fn main() {
    // Initialize the secp256k1 context
    let secp = Secp256k1::new();
    let mut rng = rng();

    // Generate a random private key
    let mut secret_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_bytes);
    let private_key = SecretKey::from_byte_array(secret_bytes)
        .expect("Invalid secret key");

    // Derive the public key
    let public_key = PublicKey::from_secret_key(&secp, &private_key);

    // Prepare and sign a message
    let message = b"Hello, world!";
    let hash = Sha256::digest(message);
    let mut hash_arr = [0u8; 32];
    hash_arr.copy_from_slice(&hash);
    let msg = Message::from_digest(hash_arr);

    let signature = secp.sign_ecdsa(msg, &private_key);
    println!("Signature: {:?}", signature);

    // Verify the signature
    let valid = secp.verify_ecdsa(msg, &signature, &public_key);
    println!("Signature valid: {}", valid.is_ok());
}