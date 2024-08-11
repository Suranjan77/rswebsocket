#[cfg(test)]
mod tests {
    use crate::sha1::hash;

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
