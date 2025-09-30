#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Free strings returned by this library.
 */
void vt_free_string(const char *ptr);

/**
 * Compare images (URLs or file paths), returning a JSON string.
 * JSON fields match the acceptance criteria (obtainedSimilarity, status, resultImageRef, etc.).
 */
const char *vt_compare_images(const char *baseline_url,
                              const char *input_url,
                              int32_t min_similarity,
                              int32_t noise_filter,
                              const char *excluded_areas_json,
                              const char *meta_json);

/**
 * Search for a child image within a parent image. Returns JSON string.
 */
const char *vt_flex_search(const char *parent_url, const char *child_url, const char *meta_json);

/**
 * Locate relative position of one element to another. Returns JSON string.
 */
const char *vt_flex_locate(const char *container_url,
                           const char *main_url,
                           const char *relative_url,
                           const char *meta_json);
