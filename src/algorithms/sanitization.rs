use once_cell::sync::Lazy;
use regex::Regex;

static URL_STRIP_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"https?://\S+").unwrap());

/// Sanitize JSON by removing MongoDB operator keys (keys starting with $)
pub fn sanitize_for_mongo(value: &mut serde_json::Value) {
    if let serde_json::Value::Object(obj) = value {
        obj.retain(|k, _| !k.starts_with('$'));
        for v in obj.values_mut() {
            sanitize_for_mongo(v);
        }
    }
}

/// Non-mutating version — returns a new Value instead of mutating in place.
pub fn sanitize_for_mongo_owned(value: serde_json::Value) -> serde_json::Value {
    let mut v = value;
    sanitize_for_mongo(&mut v);
    v
}

/// Escape HTML characters for safe display (XSS prevention)
pub fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Strip URLs from text (for chat filtering)
pub fn strip_urls(input: &str) -> String {
    URL_STRIP_REGEX.replace_all(input, "[removed]").to_string()
}

/// Cap string to maximum length, preserving UTF-8 character boundaries
pub fn cap_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        s.chars().take(max_len).collect()
    }
}

/// Sanitize chat text specifically for overlay rendering.
///
/// Order matters: strip URLs BEFORE escaping HTML to prevent entity reconstruction attacks.
pub fn sanitize_for_overlay(text: &str, max_len: usize) -> String {
    let without_links = strip_urls(text);
    let escaped = escape_html(&without_links);
    let trimmed = escaped.trim();
    cap_string(trimmed, max_len)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // -- escape_html -----------------------------------------------------------

    #[test]
    fn escape_html_replaces_ampersand_first() {
        // & must be replaced first to avoid double-escaping the entities below.
        assert_eq!(escape_html("&"), "&amp;");
        assert_eq!(escape_html("a & b"), "a &amp; b");
    }

    #[test]
    fn escape_html_replaces_angle_brackets() {
        assert_eq!(escape_html("<script>"), "&lt;script&gt;");
        assert_eq!(escape_html("</p>"), "&lt;/p&gt;");
    }

    #[test]
    fn escape_html_replaces_double_quote() {
        assert_eq!(escape_html(r#"""#), "&quot;");
        assert_eq!(escape_html(r#"a "b" c"#), "a &quot;b&quot; c");
    }

    #[test]
    fn escape_html_replaces_single_quote() {
        assert_eq!(escape_html("'"), "&#39;");
        assert_eq!(escape_html("don't"), "don&#39;t");
    }

    #[test]
    fn escape_html_escapes_combined_xss_payload() {
        // The classic "<script>alert('xss')</script>" pattern.
        assert_eq!(
            escape_html("<script>alert('xss')</script>"),
            "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;"
        );
    }

    #[test]
    fn escape_html_is_noop_on_safe_text() {
        assert_eq!(escape_html("hello world 123"), "hello world 123");
    }

    // -- sanitize_for_mongo ----------------------------------------------------

    #[test]
    fn sanitize_for_mongo_strips_top_level_dollar_keys() {
        let mut v = json!({ "$where": "bad", "name": "ok" });
        sanitize_for_mongo(&mut v);
        assert_eq!(v, json!({ "name": "ok" }));
    }

    #[test]
    fn sanitize_for_mongo_recurses_into_nested_objects() {
        let mut v = json!({
          "outer": { "$ne": null, "inner": { "$gt": 1, "x": 2 } }
        });
        sanitize_for_mongo(&mut v);
        assert_eq!(v, json!({ "outer": { "inner": { "x": 2 } } }));
    }

    #[test]
    fn sanitize_for_mongo_leaves_non_object_values_alone() {
        let mut v = json!([1, "two", true, null]);
        sanitize_for_mongo(&mut v);
        assert_eq!(v, json!([1, "two", true, null]));

        let mut s = json!("plain string with $dollar in it");
        sanitize_for_mongo(&mut s);
        assert_eq!(s, json!("plain string with $dollar in it"));
    }

    // -- strip_urls ------------------------------------------------------------

    #[test]
    fn strip_urls_removes_http_and_https() {
        assert_eq!(
            strip_urls("visit http://example.com today"),
            "visit [removed] today"
        );
        assert_eq!(strip_urls("see https://rust-lang.org"), "see [removed]");
    }

    #[test]
    fn strip_urls_preserves_text_without_urls() {
        assert_eq!(strip_urls("plain text no url"), "plain text no url");
    }

    // -- cap_string ------------------------------------------------------------

    #[test]
    fn cap_string_returns_input_under_max_len() {
        assert_eq!(cap_string("hello", 10), "hello");
        assert_eq!(cap_string("hello", 5), "hello");
    }

    #[test]
    fn cap_string_truncates_at_max_len_preserving_utf8_boundaries() {
        // "café" has a 4-byte UTF-8 'é'; truncation must not split it.
        assert_eq!(cap_string("café", 3), "caf");
        assert_eq!(cap_string("a😀b", 1), "a");
    }

    #[test]
    fn cap_string_handles_max_len_zero() {
        assert_eq!(cap_string("anything", 0), "");
    }

    // -- sanitize_for_overlay -------------------------------------------------

    #[test]
    fn sanitize_for_overlay_strips_url_then_escapes_html_then_trims() {
        // The URL regex `https?://\S+` is greedy on non-whitespace, so a URL
        // directly followed by markup (e.g. `</b>`) is consumed together.
        // This documents the current behaviour so a future regex fix is a
        // visible change. The important guarantee is that the URL itself is
        // removed and any remaining markup is HTML-escaped.
        let input = "see <b>https://x.y</b> now";
        let expected = "see &lt;b&gt;[removed] now";
        assert_eq!(sanitize_for_overlay(input, 1024), expected);
    }

    #[test]
    fn sanitize_for_overlay_strips_url_with_punctuation_only() {
        // URL with terminal punctuation that would otherwise be included in \S+
        // is a known limitation of the current regex; pin the behaviour.
        let input = "see https://x.y.";
        let out = sanitize_for_overlay(input, 1024);
        // Currently the trailing `.` is part of the URL match.
        assert!(out.contains("[removed]"));
        assert!(out.contains("see "));
    }

    #[test]
    fn sanitize_for_overlay_trims_then_caps() {
        // Leading/trailing whitespace is removed before the cap is applied.
        let out = sanitize_for_overlay("   hello world   ", 5);
        assert_eq!(out, "hello");
    }

    #[test]
    fn escape_html_entity_in_realistic_xss() {
        assert_eq!(
            escape_html("&lt;img src=x onerror=alert(1)&gt;"),
            "&amp;lt;img src=x onerror=alert(1)&amp;gt;"
        );
    }

    #[test]
    fn strip_urls_multiple_urls_in_text() {
        let input = "visit http://foo.com and https://bar.com ok";
        assert_eq!(strip_urls(input), "visit [removed] and [removed] ok");
    }

    #[test]
    fn strip_urls_no_url() {
        assert_eq!(strip_urls("plain text no url"), "plain text no url");
    }

    #[test]
    fn strip_urls_empty_string() {
        assert_eq!(strip_urls(""), "");
    }

    #[test]
    fn strip_urls_ftp_url() {
        assert_eq!(
            strip_urls("ftp://files.example.com"),
            "ftp://files.example.com"
        );
    }

    #[test]
    fn sanitize_for_overlay_xss_payload_with_url() {
        let input = "check https://evil.com <script>alert(1)</script>";
        let out = sanitize_for_overlay(input, 1024);
        assert!(out.contains("[removed]"));
        assert!(out.contains("&lt;script&gt;"));
        assert!(!out.contains("<script>"));
    }

    #[test]
    fn sanitize_for_overlay_empty_input() {
        assert_eq!(sanitize_for_overlay("", 100), "");
    }

    #[test]
    fn sanitize_for_overlay_preserves_text_structure() {
        let input = "Hello &amp; goodbye <b>bold</b>";
        let out = sanitize_for_overlay(input, 1024);
        assert_eq!(out, "Hello &amp;amp; goodbye &lt;b&gt;bold&lt;/b&gt;");
    }
}
