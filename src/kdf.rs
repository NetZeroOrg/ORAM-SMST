use sha2::Sha256;

/// This is the key derivation function for the protocol
/// the `w = kdf(master_salt , id)` `s = kdf(w , salt_s)` `b = kdf(b, salt_b)`
pub fn kdf(salt: Option<&[u8]>, id: Option<&[u8]>, ikm: &[u8]) -> [u8; 32] {
    if salt.is_none() && id.is_none() {
        panic!("salt and byte both not provided");
    }
    let hk = hkdf::Hkdf::<Sha256>::new(salt, ikm);
    let mut okm = [0u8; 32];
    hk.expand(id.unwrap_or_default(), &mut okm)
        .expect("42 is a valid length for Sha256 to output");
    okm
}
