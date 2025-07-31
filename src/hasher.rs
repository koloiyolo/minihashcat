use md2::Md2;
use md4::Md4;
use sha2::{Digest, Sha256, Sha384, Sha512};
use sha3::{Sha3_256, Sha3_384, Sha3_512};

pub trait Hasher {
    /// Returns hash of the input
    fn hash(&self, input: &[u8]) -> Vec<u8>;
    /// Returns name of the hash algorithm
    fn name(&self) -> &'static str;

    /// Compares hash of the provided text with provided hash. Returns bool
    fn compare_hash(&self, text: &[u8], hash: &String) -> bool {
        let hashed_text = hex::encode(self.hash(text));
        hashed_text == *hash
    }
}

pub struct Md2Hasher;
/// MD2 implementation
impl Hasher for Md2Hasher {
    fn hash(&self, input: &[u8]) -> Vec<u8> {
        let mut hasher = Md2::new();
        hasher.update(input);
        hasher.finalize().to_vec()
    }

    fn name(&self) -> &'static str {
        "MD2"
    }
}

pub struct Md4Hasher;
/// MD4 implementation
impl Hasher for Md4Hasher {
    fn hash(&self, input: &[u8]) -> Vec<u8> {
        let mut hasher = Md4::new();
        hasher.update(input);
        hasher.finalize().to_vec()
    }

    fn name(&self) -> &'static str {
        "MD4"
    }
}

pub struct Sha256Hasher;
/// SHA-2 256 implementation
impl Hasher for Sha256Hasher {
    fn hash(&self, input: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(input);
        hasher.finalize().to_vec()
    }

    fn name(&self) -> &'static str {
        "SHA-2 256"
    }
}

pub struct Sha384Hasher;
/// MD4 implementation
impl Hasher for Sha384Hasher {
    fn hash(&self, input: &[u8]) -> Vec<u8> {
        let mut hasher = Sha384::new();
        hasher.update(input);
        hasher.finalize().to_vec()
    }

    fn name(&self) -> &'static str {
        "SHA-2 384"
    }
}

pub struct Sha512Hasher;
/// SHA-2 256 implementation
impl Hasher for Sha512Hasher {
    fn hash(&self, input: &[u8]) -> Vec<u8> {
        let mut hasher = Sha512::new();
        hasher.update(input);
        hasher.finalize().to_vec()
    }

    fn name(&self) -> &'static str {
        "SHA-2 512"
    }
}

pub struct Sha3_256Hasher;
/// SHA-3 256 implementation
impl Hasher for Sha3_256Hasher {
    fn hash(&self, input: &[u8]) -> Vec<u8> {
        let mut hasher = Sha3_256::new();
        hasher.update(input);
        hasher.finalize().to_vec()
    }

    fn name(&self) -> &'static str {
        "SHA-3 256"
    }
}

pub struct Sha3_384Hasher;
/// SHA-3 256 implementation
impl Hasher for Sha3_384Hasher {
    fn hash(&self, input: &[u8]) -> Vec<u8> {
        let mut hasher = Sha3_384::new();
        hasher.update(input);
        hasher.finalize().to_vec()
    }

    fn name(&self) -> &'static str {
        "SHA-3 384"
    }
}

pub struct Sha3_512Hasher;
/// SHA-3 512 implementation
impl Hasher for Sha3_512Hasher {
    fn hash(&self, input: &[u8]) -> Vec<u8> {
        let mut hasher = Sha3_512::new();
        hasher.update(input);
        hasher.finalize().to_vec()
    }

    fn name(&self) -> &'static str {
        "SHA-3 512"
    }
}

pub fn create_hasher(name: &str) -> Box<dyn Hasher> {
    match name.to_lowercase().as_str() {
        "sha2_256" | "sha256" => Box::new(Sha256Hasher),
        "sha2_384" | "sha384" => Box::new(Sha384Hasher),
        "sha2_512" | "sha512" => Box::new(Sha512Hasher),
        "sha3_256" => Box::new(Sha3_256Hasher),
        "sha3_384" => Box::new(Sha3_384Hasher),
        "sha3_512" => Box::new(Sha3_512Hasher),
        "md2" => Box::new(Md2Hasher),
        "md4" => Box::new(Md4Hasher),
        _ => Box::new(Sha256Hasher),
    }
}
