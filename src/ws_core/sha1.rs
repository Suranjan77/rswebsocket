use std::fmt::Write;

pub fn hash(msg: &str) -> Vec<u8> {
    let pad_msg = pad(msg.as_bytes());

    let mut h = [
        0x67452301u32,
        0xEFCDAB89u32,
        0x98BADCFEu32,
        0x10325476u32,
        0xC3D2E1F0u32,
    ];
    pad_msg.chunks(64).for_each(|block| {
        let mut w = [0u32; 80];
        block
            .chunks(4)
            .enumerate()
            .for_each(|(t, d)| w[t] = u32::from_be_bytes(d.try_into().unwrap()));

        for t in 16..80 {
            w[t] = cls(w[t - 3] ^ w[t - 8] ^ w[t - 14] ^ w[t - 16], 1);
        }

        let mut a = h;

        (0..80).for_each(|t| {
            let tmp = cls(a[0], 5)
                .wrapping_add(f(&a[1..=3], t))
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

    let v = h.iter().fold(Vec::new(), |mut v, b| {
        v.extend_from_slice(&(*b).to_be_bytes());
        v
    });

    println!("{:?}", v);

    v
}

fn f(a: &[u32], t: usize) -> u32 {
    match t {
        0..=19 => (a[0] & a[1]) | ((!a[0]) & a[2]),
        20..=39 => a[0] ^ a[1] ^ a[2],
        40..=59 => (a[0] & a[1]) | (a[0] & a[2]) | (a[1] & a[2]),
        _ => a[0] ^ a[1] ^ a[2],
    }
}

fn k(t: usize) -> u32 {
    match t {
        0..=19 => 0x5A827999u32,
        20..=39 => 0x6ED9EBA1u32,
        40..=59 => 0x8F1BBCDCu32,
        _ => 0xCA62C1D6u32,
    }
}

fn pad(msg: &[u8]) -> Vec<u8> {
    let mut pad_vec = Vec::new();
    pad_vec.extend_from_slice(msg);
    pad_vec.push(0x80u8);

    while (pad_vec.len() + 8) % 64 != 0 {
        pad_vec.push(0u8);
    }

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
        assert_eq!(
            result,
            vec!(
                218, 57, 163, 238, 94, 107, 75, 13, 50, 85, 191, 239, 149, 96, 24, 144, 175, 216,
                7, 9
            )
        );
    }

    #[test]
    fn test_sha1_abc() {
        let result = hash("abc");
        assert_eq!(
            result,
            vec!(
                169, 153, 62, 54, 71, 6, 129, 106, 186, 62, 37, 113, 120, 80, 194, 108, 156, 208,
                216, 157
            )
        );
    }

    #[test]
    fn test_sha1_longer_message() {
        let result = hash("The quick brown fox jumps over the lazy dog");
        assert_eq!(
            result,
            vec!(
                47, 212, 225, 198, 122, 45, 40, 252, 237, 132, 158, 225, 187, 118, 231, 57, 27,
                147, 235, 18
            )
        );
    }

    #[test]
    fn test_sha1_longer_message_with_period() {
        let result = hash("The quick brown fox jumps over the lazy dog.");
        assert_eq!(
            result,
            vec!(
                64, 141, 148, 56, 66, 22, 248, 144, 255, 122, 12, 53, 40, 232, 190, 209, 224, 176,
                22, 33
            )
        );
    }

    #[test]
    fn test_sha1_repeated_characters() {
        let result = hash("aaaaaaaaaa");
        assert_eq!(
            result,
            vec!(
                52, 149, 255, 105, 211, 70, 113, 209, 225, 91, 51, 166, 60, 19, 121, 253, 237, 211,
                163, 42
            )
        );
    }

    #[test]
    fn test_sha1_alphanumeric_string() {
        let result = hash("1234567890abcdefghijklmnopqrstuvwxyz");
        assert_eq!(
            result,
            vec!(
                84, 113, 213, 228, 233, 29, 12, 13, 135, 36, 157, 88, 115, 215, 252, 181, 161, 65,
                165, 130
            )
        );
    }

    #[test]
    fn test_sha1_non_ascii_characters() {
        let result = hash("こんにちは世界");
        assert_eq!(
            result,
            vec!(
                164, 217, 221, 68, 176, 149, 26, 0, 143, 168, 72, 101, 223, 20, 213, 182, 198, 247,
                236, 219
            )
        );
    }
}
