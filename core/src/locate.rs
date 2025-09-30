use serde::{Deserialize, Serialize};

use crate::filters::Meta;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocateRequest {
    pub container_image: String,
    pub main_image: String,
    pub relative_image: String,
    #[serde(default)]
    pub meta: Meta,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RelativePosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Left,
    Right,
    Above,
    Below,
    Overlapping,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocateResult {
    pub status: String, // "Found" | "NotFound"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_region: Option<(u32, u32, u32, u32)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_region: Option<(u32, u32, u32, u32)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_position_from_main: Option<RelativePosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_image_ref: Option<String>,
}

/// Stub implementation: returns NotFound without regions.
pub fn flex_locate(_req: LocateRequest) -> LocateResult {
    LocateResult {
        status: "NotFound".to_string(),
        main_region: None,
        relative_region: None,
        relative_position_from_main: None,
        description: None,
        result_image_ref: None,
    }
}
