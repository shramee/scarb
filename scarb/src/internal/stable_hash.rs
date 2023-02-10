#[allow(deprecated)]
use std::hash::SipHasher;
use std::hash::{Hash, Hasher};

use data_encoding::BASE32HEX_NOPAD;

/// Implementation of a hasher that produces the same values across Scarb releases.
///
/// The hasher should be fast and have a low chance of collisions (but is not sufficient for
/// cryptographic purposes).
#[allow(deprecated)]
pub struct StableHasher(SipHasher);

impl StableHasher {
    pub fn new() -> Self {
        #[allow(deprecated)]
        Self(SipHasher::new())
    }
}

impl Hasher for StableHasher {
    fn finish(&self) -> u64 {
        self.0.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }
}

pub fn short_hash(hashable: &impl Hash) -> String {
    let hash = {
        let mut hasher = StableHasher::new();
        hashable.hash(&mut hasher);
        hasher.finish()
    };

    BASE32HEX_NOPAD.encode(&hash.to_le_bytes())
}

#[cfg(test)]
mod tests {
    use super::short_hash;

    #[test]
    fn short_hash_is_stable() {
        assert_eq!(short_hash(&"abcd"), "LA8VKK9KUOE2M");
        assert_eq!(short_hash(&123), "8B89NJO1D02MG");
    }
}