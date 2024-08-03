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
