use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use crate::student::lesson::Lesson;
use crate::student::types::{Answer, EntityId, Id, Seconds};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StudentEvent {
    StudentCreated { student_id: EntityId, name: String },
    LessonStarted { student_id: EntityId, lesson: Lesson },
    VideoBookmarked { student_id: EntityId, lesson_id: EntityId, step_id: Id, seconds_into_video: Seconds},
    VideoCompleted { student_id: EntityId, lesson_id: EntityId, step_id: Id },
    QuestionAnswered { student_id: EntityId, lesson_id: EntityId, step_id: Id, answer: Answer },
}

impl DomainEvent for StudentEvent {
    fn event_type(&self) -> String {
        match self {
            StudentEvent::StudentCreated { .. } => "StudentCreated".to_string(),
            StudentEvent::LessonStarted { .. } => "LessonStarted".to_string(),
            StudentEvent::VideoBookmarked { .. } => "VideoBookmarked".to_string(),
            StudentEvent::VideoCompleted { .. } => "VideoCompleted".to_string(),
            StudentEvent::QuestionAnswered { .. } => "QuestionAnswered".to_string(),
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Debug)]
pub struct StudentError(String);

impl From<&str> for StudentError {
    fn from(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

impl Display for StudentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for StudentError {}
