use crate::model::study::events::StudentError;
use crate::model::study::lesson::Lesson;
use crate::model::types::EntityId;
use async_trait::async_trait;

pub struct StudentServices {
    pub services: Box<dyn StudentApi>,
}

impl StudentServices {
    pub fn new(services: Box<dyn StudentApi>) -> Self {
        Self { services }
    }
}

#[async_trait]
pub trait StudentApi: Sync + Send {
    async fn find_lesson(&self, lesson_id: EntityId) -> Result<Lesson, StudentError>;
}

pub struct HappyPathStudentServices;

#[async_trait]
impl StudentApi for HappyPathStudentServices {
    async fn find_lesson(&self, lesson_id: EntityId) -> Result<Lesson, StudentError> {
        todo!()
    }
}
