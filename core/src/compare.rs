use serde::{Deserialize, Serialize};

use crate::filters::{Meta, Rect};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompareRequest {
    pub baseline_image: String,
    pub input_image: String,
    pub min_similarity: Option<i32>,
    pub noise_filter: Option<i32>,
    pub excluded_areas: Option<Vec<Rect>>,
    #[serde(default)]
    pub meta: Meta,
}

#[derive(Debug, Clone, Serialize)]
pub enum CompareStatus {
    Passed,
    Failed,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompareResult {
    pub obtained_similarity: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CompareStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_image_ref: Option<String>,
    pub noise_filter: i32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_areas: Vec<Rect>,
}

/// Simple placeholder implementation.
/// Attempts to read both files and computes a byte-wise similarity ratio.
/// - If both files are identical, returns 100.0.
/// - If reading fails, returns 0.0.
/// NOTE: This is not a perceptual image comparison; it's a lightweight stand‑in
/// until a real image diff (SSIM/PSNR) is wired in.
pub fn compare_images(req: CompareRequest) -> CompareResult {
    // Attempt pixel-wise comparison using `image` crate.
    let (similarity, diff_path_opt) = match pixel_similarity(&req) {
        Ok(pair) => pair,
        Err(_) => {
            // Fallback: byte-wise if loading fails
            let sim = byte_similarity(&req.baseline_image, &req.input_image);
            (sim, None)
        }
    };

    let noise = req.noise_filter.unwrap_or(20).clamp(0, 100);
    let mut status = None;
    if let Some(min) = req.min_similarity {
        status = Some(if (similarity as i32) >= min {
            CompareStatus::Passed
        } else {
            CompareStatus::Failed
        });
    }

    CompareResult {
        obtained_similarity: similarity,
        status,
        result_image_ref: diff_path_opt,
        noise_filter: noise,
        excluded_areas: req.excluded_areas.unwrap_or_default(),
    }
}

fn byte_similarity(a: &str, b: &str) -> f32 {
    match (std::fs::read(a), std::fs::read(b)) {
        (Ok(bas), Ok(inp)) => {
            if bas == inp {
                return 100.0;
            }
            let n = bas.len().min(inp.len());
            if n == 0 { return 0.0; }
            let same = bas.iter().zip(inp.iter()).take(n).filter(|(x, y)| x == y).count();
            ((same as f32) / (n as f32) * 100.0).clamp(0.0, 100.0)
        }
        _ => 0.0,
    }
}

fn pixel_similarity(req: &CompareRequest) -> Result<(f32, Option<String>), String> {
    use image::{imageops::FilterType, GenericImageView, ImageBuffer, Luma};

    let img_a = image::open(&req.baseline_image).map_err(|e| format!("load A: {e}"))?;
    let img_b = image::open(&req.input_image).map_err(|e| format!("load B: {e}"))?;

    // Convert to grayscale to compare luminance
    let a_gray = img_a.to_luma8();
    let b_gray = img_b.to_luma8();

    // Normalize to the same size (square 256x256) for a robust, fast comparison
    let target_w = 256u32;
    let target_h = 256u32;
    let a_res = image::imageops::resize(&a_gray, target_w, target_h, FilterType::Lanczos3);
    let b_res = image::imageops::resize(&b_gray, target_w, target_h, FilterType::Lanczos3);

    // Build an exclusion mask scaled to target size
    let mut include_mask = vec![true; (target_w * target_h) as usize];
    if let Some(rects) = &req.excluded_areas {
        let (aw, ah) = (img_a.width().max(1), img_a.height().max(1));
        let sx = target_w as f32 / aw as f32;
        let sy = target_h as f32 / ah as f32;
        for r in rects {
            let x0 = ((r.top_left_x as f32) * sx).floor().max(0.0) as u32;
            let y0 = ((r.top_left_y as f32) * sy).floor().max(0.0) as u32;
            let x1 = ((r.bottom_right_x as f32) * sx).ceil().min(target_w as f32 - 1.0) as u32;
            let y1 = ((r.bottom_right_y as f32) * sy).ceil().min(target_h as f32 - 1.0) as u32;
            for y in y0..=y1 {
                for x in x0..=x1 {
                    include_mask[(y * target_w + x) as usize] = false;
                }
            }
        }
    }

    // Compute normalized L1 difference across included pixels
    let mut sum_abs: u64 = 0;
    let mut count: u64 = 0;
    for (i, (pa, pb)) in a_res.pixels().zip(b_res.pixels()).enumerate() {
        if include_mask.get(i).copied().unwrap_or(true) {
            let da = pa.0[0] as i32;
            let db = pb.0[0] as i32;
            sum_abs += (da - db).unsigned_abs() as u64;
            count += 1;
        }
    }

    let similarity = if count == 0 {
        0.0
    } else {
        let max_total = 255u64 * count;
        let score = 1.0 - (sum_abs as f64 / max_total as f64);
        (score.max(0.0).min(1.0) * 100.0) as f32
    };

    // Optionally generate a diff image (grayscale abs difference)
    let mut diff = ImageBuffer::<Luma<u8>, Vec<u8>>::new(target_w, target_h);
    for (i, (p, q)) in a_res.pixels().zip(b_res.pixels()).enumerate() {
        let v = (p.0[0] as i32 - q.0[0] as i32).unsigned_abs() as u8;
        let y = (i as u32) / target_w;
        let x = (i as u32) % target_w;
        diff.put_pixel(x, y, Luma([v]));
    }
    let out = std::env::temp_dir().join(format!("vt_diff_{}.png", nano_ts()));
    let _ = diff.save(&out); // best effort
    let diff_ref = out.to_string_lossy().to_string();

    Ok((similarity, Some(diff_ref)))
}

fn nano_ts() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos()
}
