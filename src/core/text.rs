/// Truncate a string to at most `max_bytes` bytes without splitting a UTF-8
/// character. Returns the longest prefix whose byte length is ≤ `max_bytes`.
pub fn truncate_at_boundary(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        return s;
    }
    let mut end = max_bytes;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shorter_string_unchanged() {
        assert_eq!(truncate_at_boundary("hello", 10), "hello");
        assert_eq!(truncate_at_boundary("hello", 5), "hello");
    }

    #[test]
    fn ascii_truncated_exactly() {
        assert_eq!(truncate_at_boundary("hello world", 5), "hello");
    }

    #[test]
    fn multibyte_boundary_respected() {
        // 'é' is 2 bytes — cutting at byte 1 must back off to 0
        assert_eq!(truncate_at_boundary("é", 1), "");
        // "aé" — cutting at byte 2 lands mid-'é', backs off to "a"
        assert_eq!(truncate_at_boundary("aé", 2), "a");
        assert_eq!(truncate_at_boundary("aé", 3), "aé");
    }

    #[test]
    fn emoji_not_split() {
        // '✓' is 3 bytes
        let s = "✓✓✓";
        assert_eq!(truncate_at_boundary(s, 4), "✓");
        assert_eq!(truncate_at_boundary(s, 6), "✓✓");
    }

    #[test]
    fn empty_string() {
        assert_eq!(truncate_at_boundary("", 5), "");
    }
}
