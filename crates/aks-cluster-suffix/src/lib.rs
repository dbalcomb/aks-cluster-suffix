use std::hash::Hasher;

use fnv::FnvHasher;

use self::lookup::LOOKUP;

mod lookup;

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut hasher = FnvHasher::default();
    hasher.write(bytes);
    hasher.finish()
}

fn seed(mut seed: i64) -> i32 {
    seed %= std::i32::MAX as i64;

    if seed < 0 {
        seed += std::i32::MAX as i64;
    }

    if seed == 0 {
        seed = 89_482_311;
    }

    seed as i32
}

fn seedrand(mut seed: i32) -> i32 {
    const A: i32 = 48271;
    const Q: i32 = 44488;
    const R: i32 = 3399;

    let hi = seed / Q;
    let lo = seed % Q;

    seed = A * lo - R * hi;

    if seed < 0 {
        seed += std::i32::MAX;
    }

    seed
}

/// Generates the Azure Kubernetes Service (AKS) cluster suffix.
///
/// This method is equivalent to the follow `Go` code extracted from the `aks-engine` project
/// and should produce the same results.
///
/// ```go
/// import (
///     "fmt"
///     "hash/fnv"
///     "math/rand"
/// )
///
/// func suffix(input []byte) string {
///     h := fnv.New64a()
///     h.Write(input)
///     r := rand.New(rand.NewSource(int64(h.Sum64())))
///     return fmt.Sprintf("%08d", r.Uint32())[:8]
/// }
/// ```
pub fn suffix(input: &[u8]) -> String {
    let hash = fnv1a(input) as i64;

    let mut list = [0; 607];
    let mut seed = seed(hash);

    for index in 0..627 {
        seed = seedrand(seed);

        if index >= 20 {
            let mut temp = (seed as i64) << 40;
            seed = seedrand(seed);
            temp ^= (seed as i64) << 20;
            seed = seedrand(seed);
            temp ^= seed as i64;
            temp ^= LOOKUP[index - 20];
            list[index - 20] = temp;
        }
    }

    let as_u64 = list[333].wrapping_add(list[606]) as u64;
    let as_i64 = (as_u64 & (std::i64::MAX as u64)) as i64;
    let as_u32 = (as_i64 >> 31) as u32;

    as_u32.to_string().chars().take(8).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::{seed, suffix};

    #[test]
    fn test_seed() {
        assert_eq!(seed(-1), 2_147_483_646);
        assert_eq!(seed(0), 89_482_311);
        assert_eq!(seed(1), 1);
    }

    #[test]
    fn test_suffix() {
        assert_eq!(suffix(b"aks"), "35064155");
        assert_eq!(suffix(b"aks-test"), "35862059");
        assert_eq!(suffix(b"aks-testing-dns"), "25202371");
        assert_eq!(suffix(b"akstest"), "42554519");
        assert_eq!(suffix(b"k8stest"), "21324540");
        assert_eq!(suffix(b"hello-world"), "36442331");
        assert_eq!(suffix(b"abcdefghijklmnopqrstuvwxyz0123456789"), "41931308");
        assert_eq!(suffix(b"0123456789"), "33051408");
    }
}
