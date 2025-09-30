//! Core implementation of the SDK.

pub mod filters;
pub mod compare;
pub mod search;
pub mod locate;

pub use compare::{compare_images, CompareRequest, CompareResult, CompareStatus};
pub use search::{flex_search, SearchRequest, SearchResult, MatchRegion};
pub use locate::{flex_locate, LocateRequest, LocateResult, RelativePosition};
