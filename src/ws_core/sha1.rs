use std::fmt::Write;

pub fn hash(msg: &str) -> String {
    let pad_msg = pad(msg.as_bytes());

    let mut h = [0x67452301u32, 0xEFCDAB89u32, 0x98BADCFEu32, 0x10325476u32, 0xC3D2E1F0u32];
    pad_msg.chunks(64).for_each(|block| {
        let mut w = [0u32; 80];
        block.chunks(4).enumerate()
            .for_each(|(t, d)| w[t] = u32::from_be_bytes(d.try_into().unwrap()));

        for t in 16..80 {
            w[t] = cls(w[t - 3] ^ w[t - 8] ^ w[t - 14] ^ w[t - 16], 1);
        }

        let mut a = h;

        (0..80).for_each(|t| {
            let tmp = cls(a[0], 5).wrapping_add(f(&a[1..=3], t))
                .wrapping_add(a[4])
                .wrapping_add(w[t])
                .wrapping_add(k(t));
            a[4] = a[3];
            a[3] = a[2];
            a[2] = cls(a[1], 30);
            a[1] = a[0];
            a[0] = tmp;
        });

        h[0] = h[0].wrapping_add(a[0]);
        h[1] = h[1].wrapping_add(a[1]);
        h[2] = h[2].wrapping_add(a[2]);
        h[3] = h[3].wrapping_add(a[3]);
        h[4] = h[4].wrapping_add(a[4]);
    });

    h.iter().fold(String::new(), |mut out, b| {
        let _ = write!(out, "{b:08x}");
        out
    })
}

fn f(a: &[u32], t: usize) -> u32 {
    match t {
        0..=19 => (a[0] & a[1]) | ((!a[0]) & a[2]),
        20..=39 => a[0] ^ a[1] ^ a[2],
        40..=59 => (a[0] & a[1]) | (a[0] & a[2]) | (a[1] & a[2]),
        _ => a[0] ^ a[1] ^ a[2]
    }
}

fn k(t: usize) -> u32 {
    match t {
        0..=19 => 0x5A827999u32,
        20..=39 => 0x6ED9EBA1u32,
        40..=59 => 0x8F1BBCDCu32,
        _ => 0xCA62C1D6u32
    }
}

fn pad(msg: &[u8]) -> Vec<u8> {
    let mut pad_vec = Vec::new();
    pad_vec.extend_from_slice(msg);
    pad_vec.push(0x80u8);

    let zero_count = 56 - (pad_vec.len() - (pad_vec.len() / 64) * 64);
    pad_vec.extend_from_slice(&[0u8].repeat(zero_count));

    pad_vec.extend_from_slice(&(msg.len() as u64 * 8).to_be_bytes());

    pad_vec
}

fn cls(x: u32, n: u8) -> u32 {
    (x << n) | (x >> (32 - n))
}

#[cfg(test)]
mod tests {
    use crate::ws_core::sha1::hash;

    #[test]
    fn test_sha1_empty_string() {
        let result = hash("");
        assert_eq!(result, "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    }

    #[test]
    fn test_sha1_abc() {
        let result = hash("abc");
        assert_eq!(result, "a9993e364706816aba3e25717850c26c9cd0d89d");
    }

    #[test]
    fn test_sha1_longer_message() {
        let result = hash("The quick brown fox jumps over the lazy dog");
        assert_eq!(result, "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12");
    }

    #[test]
    fn test_sha1_longer_message_with_period() {
        let result = hash("The quick brown fox jumps over the lazy dog.");
        assert_eq!(result, "408d94384216f890ff7a0c3528e8bed1e0b01621");
    }

    #[test]
    fn test_sha1_repeated_characters() {
        let result = hash("aaaaaaaaaa");
        assert_eq!(result, "3495ff69d34671d1e15b33a63c1379fdedd3a32a");
    }

    #[test]
    fn test_sha1_alphanumeric_string() {
        let result = hash("1234567890abcdefghijklmnopqrstuvwxyz");
        assert_eq!(result, "5471d5e4e91d0c0d87249d5873d7fcb5a141a582");
    }

    #[test]
    fn test_sha1_non_ascii_characters() {
        let result = hash("こんにちは世界");
        assert_eq!(result, "a4d9dd44b0951a008fa84865df14d5b6c6f7ecdb");
    }
}