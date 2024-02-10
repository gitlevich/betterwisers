use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Id = i32;
pub type EntityId = Uuid;
pub type Seconds = i32;

pub struct Bookmark {
    seconds_into_video: Seconds,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Answer {
}
