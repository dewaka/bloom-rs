use fnv::FnvHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Bloom {
    filter: Vec<u64>,
}

impl Bloom {
    pub fn new(size: usize) -> Self {
        Bloom {
            filter: vec![0; size],
        }
    }

    pub fn insert<T: Hash>(&mut self, elem: T) {
        let hash = Self::compute_hash(elem);
        self.insert_into(hash);
    }

    fn insert_into(&mut self, hash: u64) {
        let n = self.filter.len() as u64;
        let index = hash % (n << 3);
        let byte_index = (index >> 3) as usize;
        let bit_index = index & 7;
        self.filter[byte_index] |= 1 << bit_index;
    }

    #[inline]
    fn compute_hash<T: Hash>(t: T) -> u64 {
        let mut fnv = FnvHasher::default();
        t.hash(&mut fnv);
        fnv.finish()
    }

    pub fn contains_maybe<T: Hash>(&self, item: T) -> bool {
        let hash = Self::compute_hash(item);
        let n = self.filter.len() as u64;
        let index = hash % (n << 3);
        let byte_index = (index >> 3) as usize;
        let bit_index = index & 7;

        return (self.filter[byte_index] & (1 << bit_index)) != 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::Bloom;

    #[test]
    fn test_bloom_filter() {
        let mut bloom = Bloom::new(10);

        bloom.insert("Hello");
        assert!(bloom.contains_maybe("Hello"));

        assert!(!bloom.contains_maybe("Hi"));
    }

    #[test]
    fn test_multiple_inserts() {
        let items = [
            "the", "quick", "brown", "fox", "jumped", "over", "the", "lazy", "dog",
        ];
        let mut bloom = Bloom::new(10);

        for item in items.iter() {
            bloom.insert(item);
        }

        for item in items.iter() {
            assert!(bloom.contains_maybe(item));
        }

        for item in items.iter() {
            assert!(!bloom.contains_maybe(&format!("no {}", item)));
        }
    }
}
