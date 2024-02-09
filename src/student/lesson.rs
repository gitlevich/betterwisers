use serde::{Deserialize, Serialize};
use crate::student::types::{EntityId, Id};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Lesson {
    id: EntityId,
    name: String,
    steps: Vec<Step>,
}

impl Lesson {
    pub fn new(id: EntityId, name: String) -> Self {
        Self { id, name, steps: vec![] }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Step {
    Video { id: Id, url: String },
    Question { id: Id, question: String },
}

pub trait LessonStep {
    fn id(&self) -> Id;
    fn new_video(id: Id, url: String) -> Step {
        Step::Video { id, url }
    }
    fn new_question(id: Id, question: String) -> Step {
        Step::Question { id, question }
    }
}

impl LessonStep for Step {
    fn id(&self) -> Id {
        match self {
            Step::Video { id, .. } => *id,
            Step::Question { id, .. } => *id,
        }
    }
}
