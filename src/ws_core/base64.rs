const BASE64_TBL: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

pub fn encode(bin: &[u8]) -> String {
    assert!(!bin.is_empty());

    let mut bin_str = String::from("");

    for b in bin.iter() {
        bin_str.push_str(format!("{:08b}", b).as_str())
    }

    let mut encoded_str = String::from("");
    let mut window = 0_usize;
    for _ in 0..bin_str.len() / 6 {
        let base_idx = usize::from_str_radix(&bin_str[window..window + 6], 2).unwrap();
        encoded_str.push_str(&BASE64_TBL[base_idx].to_string());
        window += 6;
    }

    if bin_str.len() % 6 != 0 {
        handle_final_quant(&bin_str, &mut encoded_str);
    }

    encoded_str
}

fn handle_final_quant(bin_str: &str, encoded_str: &mut String) {
    let mut final_quant = String::from("");
    final_quant.push_str(&bin_str[(bin_str.len() / 6) * 6..bin_str.len()]);
    final_quant.push_str("0".repeat(6 - bin_str.len() % 6).as_str());
    let final_idx = usize::from_str_radix(&final_quant, 2).unwrap();
    encoded_str.push_str(&BASE64_TBL[final_idx].to_string());
    encoded_str.push_str("=".repeat((6 - (bin_str.len() % 6)) / 2).as_str());
}

pub fn decode(s: &str) -> String {
    let mut bin_str = String::from("");
    for c in s.chars() {
        let idx = match BASE64_TBL.iter().position(|r| c.eq(r)) {
            Some(n) => n,
            _ => {
                if '='.eq(&c) {
                    64
                } else {
                    panic!("Invalid base64 string");
                }
            }
        };

        if idx != 64 {
            bin_str.push_str(&format!("{:06b}", idx));
        }
    }

    let mut decoded = vec![];

    let mut window = 0;
    for _ in 0..bin_str.len() / 8 {
        let decoded_byte = u8::from_str_radix(&bin_str[window..window + 8], 2).unwrap();
        window += 8;
        decoded.push(decoded_byte);
    }

    String::from_utf8(decoded).unwrap()
}
