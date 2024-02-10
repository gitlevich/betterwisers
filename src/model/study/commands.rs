use serde::{Deserialize, Serialize};
use crate::model::types::{Answer, EntityId, Id, Seconds};

#[derive(Debug, Serialize, Deserialize)]
pub enum StudentCommand {
    CreateStudent { student_id: EntityId, name: String },
    StartLesson { student_id: EntityId, lesson_id: EntityId },
    BookmarkVideo { student_id: EntityId, lesson_id: EntityId, step_id: Id, seconds_into_video: Seconds },
    CompleteVideo { student_id: EntityId, lesson_id: EntityId, step_id: Id },
    AnswerQuestion { student_id: EntityId, lesson_id: EntityId, step_id: Id, answer: Answer },
}
