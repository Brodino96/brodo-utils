use blake2::{Blake2b512, Blake2s256};
use digest::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};

pub const AVAILABLE_ALGORITHMS: &[(&str, &str)] = &[
    ("MD5", "128-bit hash (legacy, not secure)"),
    ("SHA-1", "160-bit hash (legacy, not secure)"),
    ("SHA-224", "224-bit SHA-2 hash"),
    ("SHA-256", "256-bit SHA-2 hash (recommended)"),
    ("SHA-384", "384-bit SHA-2 hash"),
    ("SHA-512", "512-bit SHA-2 hash"),
    ("BLAKE2s", "256-bit BLAKE2s hash (fast)"),
    ("BLAKE2b", "512-bit BLAKE2b hash (fast)"),
    ("BLAKE3", "256-bit BLAKE3 hash (fastest)"),
    ("SHA-512/256", "256-bit truncated SHA-512"),
];

#[derive(Debug, Clone, Copy)]
pub enum HashAlgorithm {
    Md5,
    Sha1,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Sha512_256,
    Blake2s,
    Blake2b,
    Blake3,
}

impl HashAlgorithm {
    pub fn from_name(name: &str) -> Option<Self> {
        let normalized = name.to_uppercase().replace(['-', '_'], "");

        match normalized.as_str() {
            "MD5" => Some(Self::Md5),
            "SHA1" => Some(Self::Sha1),
            "SHA224" => Some(Self::Sha224),
            "SHA256" => Some(Self::Sha256),
            "SHA384" => Some(Self::Sha384),
            "SHA512" => Some(Self::Sha512),
            "SHA512256" => Some(Self::Sha512_256),
            "BLAKE2S" | "BLAKE2S256" => Some(Self::Blake2s),
            "BLAKE2B" | "BLAKE2B512" => Some(Self::Blake2b),
            "BLAKE3" => Some(Self::Blake3),
            _ => None,
        }
    }

    pub fn create_hasher(&self) -> DynHasher {
        match self {
            Self::Md5 => DynHasher::Md5(Md5::new()),
            Self::Sha1 => DynHasher::Sha1(Sha1::new()),
            Self::Sha224 => DynHasher::Sha224(Sha224::new()),
            Self::Sha256 => DynHasher::Sha256(Sha256::new()),
            Self::Sha384 => DynHasher::Sha384(Sha384::new()),
            Self::Sha512 => DynHasher::Sha512(Sha512::new()),
            Self::Sha512_256 => DynHasher::Sha512_256(sha2::Sha512_256::new()),
            Self::Blake2s => DynHasher::Blake2s(Blake2s256::new()),
            Self::Blake2b => DynHasher::Blake2b(Blake2b512::new()),
            Self::Blake3 => DynHasher::Blake3(blake3::Hasher::new()),
        }
    }
}

pub enum DynHasher {
    Md5(Md5),
    Sha1(Sha1),
    Sha224(Sha224),
    Sha256(Sha256),
    Sha384(Sha384),
    Sha512(Sha512),
    Sha512_256(sha2::Sha512_256),
    Blake2s(Blake2s256),
    Blake2b(Blake2b512),
    Blake3(blake3::Hasher),
}

impl DynHasher {
    pub fn update(&mut self, data: &[u8]) {
        match self {
            Self::Md5(h) => h.update(data),
            Self::Sha1(h) => h.update(data),
            Self::Sha224(h) => h.update(data),
            Self::Sha256(h) => h.update(data),
            Self::Sha384(h) => h.update(data),
            Self::Sha512(h) => h.update(data),
            Self::Sha512_256(h) => h.update(data),
            Self::Blake2s(h) => h.update(data),
            Self::Blake2b(h) => h.update(data),
            Self::Blake3(h) => {
                h.update(data);
            }
        }
    }

    pub fn finalize_hex(self) -> String {
        match self {
            Self::Md5(h) => format!("{:x}", h.finalize()),
            Self::Sha1(h) => format!("{:x}", h.finalize()),
            Self::Sha224(h) => format!("{:x}", h.finalize()),
            Self::Sha256(h) => format!("{:x}", h.finalize()),
            Self::Sha384(h) => format!("{:x}", h.finalize()),
            Self::Sha512(h) => format!("{:x}", h.finalize()),
            Self::Sha512_256(h) => format!("{:x}", h.finalize()),
            Self::Blake2s(h) => format!("{:x}", h.finalize()),
            Self::Blake2b(h) => format!("{:x}", h.finalize()),
            Self::Blake3(h) => h.finalize().to_hex().to_string(),
        }
    }
}