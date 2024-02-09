use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::student::commands::StudentCommand;
use crate::student::events::{StudentError, StudentEvent};
use crate::student::types::EntityId;
use crate::student::lesson::Lesson;
use crate::student::services::StudentServices;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    id: EntityId,
    name: String,
    touched_lessons: Vec<Lesson>,
}

#[async_trait]
impl Aggregate for Student {
    type Command = StudentCommand;
    type Event = StudentEvent;
    type Error = StudentError;
    type Services = StudentServices;

    fn aggregate_type() -> String {
        "student".to_string()
    }

    async fn handle(&self,
                    command: Self::Command,
                    student_services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            StudentCommand::CreateStudent { student_id, name } => {
                Ok(vec![StudentEvent::StudentCreated { student_id, name }])
            }

            StudentCommand::StartLesson { student_id, lesson_id } =>
                Self::with_lesson(
                    student_services,
                    lesson_id,
                    move |lesson| Ok(
                        vec![
                            StudentEvent::LessonStarted {
                                student_id,
                                lesson: lesson.clone(),
                            },
                        ]
                    ),
                ).await?,

            StudentCommand::BookmarkVideo { student_id, lesson_id, step_id, seconds_into_video } => {
                Ok(vec![StudentEvent::VideoBookmarked { student_id, lesson_id, step_id, seconds_into_video }])
            }
            StudentCommand::CompleteVideo { student_id, lesson_id, step_id } => {
                Ok(vec![StudentEvent::VideoCompleted { student_id, lesson_id, step_id }])
            }
            StudentCommand::AnswerQuestion { student_id, lesson_id, step_id, answer } => {
                Ok(vec![StudentEvent::QuestionAnswered { student_id, lesson_id, step_id, answer }])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            StudentEvent::StudentCreated { student_id, name } => {
                self.id = student_id
            }
            StudentEvent::LessonStarted { student_id, lesson: lesson } => {
                self.touched_lessons.push(lesson)
            }
            StudentEvent::VideoBookmarked { student_id, lesson_id, step_id, seconds_into_video } => {}
            StudentEvent::VideoCompleted { student_id, lesson_id, step_id } => {}
            StudentEvent::QuestionAnswered { student_id, lesson_id, step_id, answer } => {}
        }
    }
}

impl Student {
    async fn with_lesson<F>(
        student_services: &StudentServices,
        lesson_id: EntityId,
        block: F,
    ) -> Result<Result<Vec<StudentEvent>, StudentError>, <Student as Aggregate>::Error>
        where F: FnOnce(&Lesson) -> Result<Vec<StudentEvent>, StudentError> + 'static,
    {
        Ok(
            student_services
                .services
                .find_lesson(lesson_id)
                .await
                .map_err(|_| StudentError::from(format!("Lesson {} not found", lesson_id).as_ref()))
                .and_then(|lesson| block(&lesson))
        )
    }
}

impl Default for Student {
    fn default() -> Self {
        Student {
            id: Uuid::nil(),
            name: "".to_string(),
            touched_lessons: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use std::sync::Mutex;
    use axum::handler::Handler;
    use cqrs_es::test::TestFramework;
    use crate::student::commands::StudentCommand;
    use crate::student::events::{StudentError, StudentEvent};
    use crate::student::lesson::Lesson;
    use crate::student::services::{StudentApi, StudentServices};
    use crate::student::student::Student;
    use crate::student::types::EntityId;

    type StudentTestFramework = TestFramework<Student>;

    #[test]
    fn should_publish_event_on_create_student() {
        let student_id = EntityId::new_v4();
        let name = "James Doe".to_string();

        let expected_event = StudentEvent::StudentCreated { student_id, name: name.clone() };
        let command = StudentCommand::CreateStudent { student_id, name: name.clone() };
        let services = StudentServices::new(Box::new(MockStudentServices::default()));

        StudentTestFramework::with(services)
            .given_no_previous_events()
            .when(command)
            .then_expect_events(vec![expected_event]);
    }

    #[test]
    fn should_publish_event_on_start_existing_lesson_that_has_not_been_started() {
        let student_id = EntityId::new_v4();
        let name = "James Doe".to_string();

        let previous = StudentEvent::StudentCreated { student_id, name: name.clone() };

        let lesson_id = EntityId::new_v4();
        let lesson = Lesson::new(lesson_id, "Lesson 1".to_string());

        let expected_event = StudentEvent::LessonStarted { student_id, lesson: lesson.clone() };
        let command = StudentCommand::StartLesson { student_id, lesson_id };
        let mock_services = MockStudentServices::default();
        mock_services.set_lesson_response(Ok(lesson.clone()));
        let services = StudentServices::new(Box::new(mock_services));

        StudentTestFramework::with(services)
            .given(vec![previous])
            .when(command)
            .then_expect_events(vec![expected_event]);
    }

    pub struct MockStudentServices {
        pub lesson_response: Mutex<Option<Result<Lesson, StudentError>>>,
    }

    impl Default for MockStudentServices {
        fn default() -> Self {
            MockStudentServices {
                lesson_response: Mutex::new(None)
            }
        }
    }

    impl MockStudentServices {
        fn set_lesson_response(&self, response: Result<Lesson, StudentError>) {
            *self.lesson_response.lock().unwrap() = Some(response);
        }
    }

    #[async_trait]
    impl StudentApi for MockStudentServices {
        async fn find_lesson(&self, _: EntityId) -> Result<Lesson, StudentError> {
            self.lesson_response.lock().unwrap().take().unwrap()
        }
    }

    impl Default for Lesson {
        fn default() -> Self {
            Lesson::new(EntityId::nil(), "Lesson 1".to_string())
        }
    }
}
