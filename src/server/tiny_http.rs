//! Integration with [`tiny_http`](https://github.com/frewsxcv/tiny-http) with the `tiny_http`
//! feature (optional).
//!
//! Contains `impl `[`HttpRequest`](../trait.HttpRequest.html)` for tiny_http::Request` (not shown
//! here; see [`HttpRequest`'s implementors](../trait.HttpRequest.html#implementors)).

pub use tiny_http::Request as TinyHttpRequest;

use super::HttpRequest;

use std::io::Read;

impl<'r> HttpRequest for &'r mut TinyHttpRequest {
    type Body = &'r mut dyn Read;

    fn multipart_boundary(&self) -> Option<&str> {
        const BOUNDARY: &str = "boundary=";

        let content_type = self
            .headers()
            .iter()
            .find(|header| header.field.equiv("Content-Type"))?
            .value
            .as_str();

        // Extract the boundary value from the header.
        let boundary_value = content_type.find(BOUNDARY).map(|pos| {
            let after_boundary = &content_type[pos + BOUNDARY.len()..];
            match after_boundary.split_once(';') {
                Some((value, _)) => value,
                None => after_boundary,
            }
        })?;

        // Trim surrounding double quotes if present.
        let boundary = boundary_value
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .unwrap_or(boundary_value);

        Some(&boundary)
    }

    fn body(self) -> Self::Body {
        self.as_reader()
    }
}
