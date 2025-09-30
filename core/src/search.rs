use serde::{Deserialize, Serialize};

use crate::filters::Meta;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    pub parent_image: String,
    pub child_image: String,
    #[serde(default)]
    pub meta: Meta,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchRegion {
    pub top_left_x: u32,
    pub top_left_y: u32,
    pub bottom_right_x: u32,
    pub bottom_right_y: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub status: String, // "Found" | "NotFound"
    pub total_matches: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub matches: Vec<MatchRegion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_image_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<(f32, f32)>,
}

/// Stub implementation: returns NotFound with no matches.
pub fn flex_search(_req: SearchRequest) -> SearchResult {
    SearchResult {
        status: "NotFound".to_string(),
        total_matches: 0,
        matches: vec![],
        result_image_ref: None,
        precision: None,
        center: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_stub_not_found() {
        let req = SearchRequest {
            parent_image: "parent.png".into(),
            child_image: "child.png".into(),
            meta: Meta::default(),
        };
        let res = flex_search(req);
        assert_eq!(res.status, "NotFound");
        assert_eq!(res.total_matches, 0);
        assert!(res.matches.is_empty());
        assert!(res.result_image_ref.is_none());
    }
}
