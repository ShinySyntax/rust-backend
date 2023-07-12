use crate::bounded_context::domain::task_repository::TaskRepository;
use uuid::Uuid;

pub struct FinishTaskInput {
    pub id: String,
}

#[derive(Debug)]
pub struct FinishTaskOutput {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
}

pub struct FinishTask {
    task_repository: Box<dyn TaskRepository>,
}

impl FinishTask {
    pub fn new(task_repository: Box<dyn TaskRepository>) -> Self {
        FinishTask { task_repository }
    }

    pub fn execute(
        &mut self,
        input: FinishTaskInput,
    ) -> Result<FinishTaskOutput, Box<dyn std::error::Error>> {
        let id = Uuid::parse_str(&input.id.to_string()).unwrap();

        match self.task_repository.get_by_id(id) {
            Ok(mut task) => {
                task.finish_task();
                self.task_repository.save(task.clone());

                Ok(FinishTaskOutput {
                    id: task.id.to_string(),
                    title: task.title,
                    description: task.description,
                    status: task.status.to_string(),
                })
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_context::domain::task_status::TaskStatus;
    use crate::bounded_context::mocks::mock_task_repository::{
        MockTaskRepository, MockTaskRepositoryError,
    };
    use crate::bounded_context::mocks::mock_task_repository::{DEF_DESCRIPTION, DEF_ID, DEF_TITLE};
    use std::cell::RefCell;

    #[test]
    fn test_finish_task_execute() {
        let mock_repository = MockTaskRepository {
            selected_task: RefCell::new(None),
            saved_task: RefCell::new(None),
        };
        let mut sut = FinishTask::new(Box::new(mock_repository));

        let input = FinishTaskInput {
            id: DEF_ID.to_string(),
        };
        let output = sut.execute(input).unwrap();

        assert_eq!(output.id, DEF_ID.to_string());
        assert_eq!(output.title, DEF_TITLE.to_string());
        assert_eq!(output.description, DEF_DESCRIPTION.to_string());
        assert_eq!(output.status, TaskStatus::Done.to_string());
    }

    #[test]
    fn test_finish_task_execute_with_error() {
        let expected_error_message = "Entity not Found".to_string();
        let mock_repository = MockTaskRepositoryError {
            error_message: expected_error_message.clone(),
        };
        let mut sut = FinishTask::new(Box::new(mock_repository));

        let input = FinishTaskInput {
            id: DEF_ID.to_string(),
        };
        let output = sut.execute(input);

        assert!(output.is_err(), "Expected an error");

        let error = output.unwrap_err();

        assert_eq!(expected_error_message, error.to_string());
    }
}
