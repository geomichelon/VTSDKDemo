//! FFI layer exposing C ABI for the SDK.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use serde::{Deserialize, Serialize};

// Ensure features are mutually exclusive and at least one is set.
#[cfg(all(feature = "real", feature = "mock"))]
compile_error!("features 'real' and 'mock' are mutually exclusive");

#[cfg(not(any(feature = "real", feature = "mock")))]
compile_error!("either feature 'real' or 'mock' must be enabled");

#[cfg(feature = "real")]
use core as core_crate;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FfiMeta {
    pub test_name: Option<String>,
    pub test_mode: Option<String>,
    pub project_name: Option<String>,
    pub execution_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct FfiRect {
    pub top_left_x: u32,
    pub top_left_y: u32,
    pub bottom_right_x: u32,
    pub bottom_right_y: u32,
}

fn cstr_to_str<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() { return None; }
    // SAFETY: caller must provide valid C string or null
    let s = unsafe { CStr::from_ptr(ptr) };
    s.to_str().ok()
}

fn to_c_string(json: String) -> *const c_char {
    CString::new(json).unwrap_or_else(|_| CString::new("{}").unwrap()).into_raw()
}

/// Free strings returned by this library.
#[no_mangle]
pub extern "C" fn vt_free_string(ptr: *const c_char) {
    if ptr.is_null() { return; }
    unsafe { drop(CString::from_raw(ptr as *mut c_char)); }
}

/// Compare images (URLs or file paths), returning a JSON string.
/// JSON fields match the acceptance criteria (obtainedSimilarity, status, resultImageRef, etc.).
#[no_mangle]
pub extern "C" fn vt_compare_images(
    baseline_url: *const c_char,
    input_url: *const c_char,
    min_similarity: i32,
    noise_filter: i32,
    excluded_areas_json: *const c_char,
    meta_json: *const c_char,
) -> *const c_char {
    let _baseline = match cstr_to_str(baseline_url) { Some(s) => s, None => return to_c_string("{}".to_string()) };
    let _input = match cstr_to_str(input_url) { Some(s) => s, None => return to_c_string("{}".to_string()) };
    let excluded_areas_ffi: Option<Vec<FfiRect>> = cstr_to_str(excluded_areas_json)
        .and_then(|j| serde_json::from_str(j).ok());
    let _meta_ffi: FfiMeta = cstr_to_str(meta_json)
        .and_then(|j| serde_json::from_str(j).ok())
        .unwrap_or_default();

    #[cfg(feature = "real")]
    let result = {
        let excluded_areas: Option<Vec<core_crate::filters::Rect>> = excluded_areas_ffi.map(|v| v.into_iter().map(|r| core_crate::filters::Rect {
            top_left_x: r.top_left_x,
            top_left_y: r.top_left_y,
            bottom_right_x: r.bottom_right_x,
            bottom_right_y: r.bottom_right_y,
        }).collect());
        let meta = core_crate::filters::Meta {
            test_name: _meta_ffi.test_name,
            test_mode: _meta_ffi.test_mode,
            project_name: _meta_ffi.project_name,
            execution_name: _meta_ffi.execution_name,
        };
        let req = core_crate::compare::CompareRequest {
            baseline_image: _baseline.to_string(),
            input_image: _input.to_string(),
            min_similarity: if min_similarity >= 0 { Some(min_similarity) } else { None },
            noise_filter: if noise_filter >= 0 { Some(noise_filter) } else { None },
            excluded_areas,
            meta,
        };
        let res = core_crate::compare::compare_images(req);
        serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string())
    };

    #[cfg(feature = "mock")]
    let result = {
        // Minimal mock JSON
        let status_val = if min_similarity >= 0 { serde_json::Value::String("Failed".to_string()) } else { serde_json::Value::Null };
        let json = serde_json::json!({
            "obtainedSimilarity": 42.0,
            "status": status_val,
            "resultImageRef": serde_json::Value::Null,
            "noiseFilter": if noise_filter >= 0 { noise_filter } else { 20 },
            "excludedAreas": excluded_areas_ffi.unwrap_or_default(),
        });
        json.to_string()
    };

    to_c_string(result)
}

/// Search for a child image within a parent image. Returns JSON string.
#[no_mangle]
pub extern "C" fn vt_flex_search(
    parent_url: *const c_char,
    child_url: *const c_char,
    meta_json: *const c_char,
) -> *const c_char {
    let _parent = match cstr_to_str(parent_url) { Some(s) => s, None => return to_c_string("{}".to_string()) };
    let _child = match cstr_to_str(child_url) { Some(s) => s, None => return to_c_string("{}".to_string()) };
    let _meta_ffi: FfiMeta = cstr_to_str(meta_json)
        .and_then(|j| serde_json::from_str(j).ok())
        .unwrap_or_default();

    #[cfg(feature = "real")]
    let result = {
        let meta = core_crate::filters::Meta {
            test_name: _meta_ffi.test_name,
            test_mode: _meta_ffi.test_mode,
            project_name: _meta_ffi.project_name,
            execution_name: _meta_ffi.execution_name,
        };
        let req = core_crate::search::SearchRequest {
            parent_image: _parent.to_string(),
            child_image: _child.to_string(),
            meta,
        };
        let res = core_crate::search::flex_search(req);
        serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string())
    };

    #[cfg(feature = "mock")]
    let result = {
        let json = serde_json::json!({
            "status": "NotFound",
            "totalMatches": 0,
            "matches": [],
            "resultImageRef": serde_json::Value::Null,
        });
        json.to_string()
    };

    to_c_string(result)
}

/// Locate relative position of one element to another. Returns JSON string.
#[no_mangle]
pub extern "C" fn vt_flex_locate(
    container_url: *const c_char,
    main_url: *const c_char,
    relative_url: *const c_char,
    meta_json: *const c_char,
) -> *const c_char {
    let _container = match cstr_to_str(container_url) { Some(s) => s, None => return to_c_string("{}".to_string()) };
    let _main = match cstr_to_str(main_url) { Some(s) => s, None => return to_c_string("{}".to_string()) };
    let _relative = match cstr_to_str(relative_url) { Some(s) => s, None => return to_c_string("{}".to_string()) };
    let _meta_ffi: FfiMeta = cstr_to_str(meta_json)
        .and_then(|j| serde_json::from_str(j).ok())
        .unwrap_or_default();

    #[cfg(feature = "real")]
    let result = {
        let meta = core_crate::filters::Meta {
            test_name: _meta_ffi.test_name,
            test_mode: _meta_ffi.test_mode,
            project_name: _meta_ffi.project_name,
            execution_name: _meta_ffi.execution_name,
        };
        let req = core_crate::locate::LocateRequest {
            container_image: _container.to_string(),
            main_image: _main.to_string(),
            relative_image: _relative.to_string(),
            meta,
        };
        let res = core_crate::locate::flex_locate(req);
        serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string())
    };

    #[cfg(feature = "mock")]
    let result = {
        let json = serde_json::json!({
            "status": "NotFound",
            "resultImageRef": serde_json::Value::Null,
        });
        json.to_string()
    };

    to_c_string(result)
}
