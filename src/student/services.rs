use async_trait::async_trait;
use crate::student::events::StudentError;

use crate::student::lesson::Lesson;
use crate::student::types::EntityId;

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
