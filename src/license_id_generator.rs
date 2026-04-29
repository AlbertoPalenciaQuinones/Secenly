use openssl::hash::{hash, MessageDigest};

pub struct LicenseIdGenerator;

impl LicenseIdGenerator {
    pub fn generate(product_id: &str) -> String {
        let length = product_id.len();

        let mut hashes: Vec<Vec<u8>> = Vec::new();

        for i in 0..4 {
            let start = i * length / 4;
            let end = (i + 1) * length / 4;

            let fragment = &product_id[start..end];

            let hash = Self::hash(fragment);
            hashes.push(hash);
        }

        let mut mixed = hashes[0].clone();

        for h in hashes.iter().skip(1) {
            mixed = Self::xor_buffers(&mixed, h);
        }

        hex::encode(mixed)
    }

    fn hash(fragment: &str) -> Vec<u8> {
        let digest = hash(MessageDigest::sha512(), fragment.as_bytes())
            .expect("Error hashing");

        digest.to_vec()
    }

    fn xor_buffers(a: &[u8], b: &[u8]) -> Vec<u8> {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| x ^ y)
            .collect()
    }
}