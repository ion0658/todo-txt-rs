/// Parse a task from a string.
///
/// # Arguments
///
/// * `value` - The task object to parse.
///
/// # Examples
///
/// ```
/// let task = todo_txt_model::Task {
///                    state: todo_txt_model::TaskState::Done,
///                    priority: Some(todo_txt_model::TaskPriority::A),
///                    completed_date: chrono::NaiveDate::from_ymd_opt(2020, 1, 1),
///                    created_date: chrono::NaiveDate::from_ymd_opt(2020, 1, 2),
///                    description: todo_txt_model::TaskDescription {
///                        value: "123 abc description".to_string(),
///                        project: vec!["project".to_string(), "project2".to_string()],
///                        context: vec!["context".to_string(), "ctx2".to_string()],
///                    }
///                };
/// let result = todo_txt_serializer::to_string(&task);
/// let expected = "x (A) 2020-01-01 2020-01-02 123 abc description +project +project2 @context @ctx2";
/// assert_eq!(result, expected);
#[tracing::instrument(parent = None)]
pub fn to_string(value: &todo_txt_model::Task) -> String {
    let result = write_task_state(String::new(), value.state);
    let result = write_task_priority(result, value.priority);
    let result = write_task_date(result, value.completed_date);
    let result = write_task_date(result, value.created_date);
    write_task_description(result, &value.description)
}

/// Write the task state to the output string.
#[tracing::instrument(parent = None, skip(state))]
fn write_task_state(mut out: String, state: todo_txt_model::TaskState) -> String {
    tracing::debug!("state: {:?}", state);
    if state == todo_txt_model::TaskState::Done {
        out.push(crate::COMPLETE_MARKER);
    }
    out
}

/// Write the task priority to the output string.
#[tracing::instrument(parent = None, skip(priority))]
fn write_task_priority(mut out: String, priority: Option<todo_txt_model::TaskPriority>) -> String {
    tracing::debug!("priority: {:?}", priority);
    if let Some(priority) = priority {
        if !out.is_empty() {
            out.push(crate::TOKEN_SEPARATOR);
        }
        out.push(crate::PRIORITY_MARKER_PRE);
        out.push(char::from(priority));
        out.push(crate::PRIORITY_MARKER_POST);
    }
    out
}

/// Write the task date to the output string.
#[tracing::instrument(parent = None, skip(date))]
fn write_task_date(mut out: String, date: Option<chrono::NaiveDate>) -> String {
    tracing::debug!("date: {:?}", date);
    if let Some(date) = date {
        if !out.is_empty() {
            out.push(crate::TOKEN_SEPARATOR);
        }
        out.push_str(&date.format("%Y-%m-%d").to_string());
    }
    out
}

/// Write the task description to the output string.
#[tracing::instrument(parent = None, skip(description))]
fn write_task_description(
    mut out: String,
    description: &todo_txt_model::TaskDescription,
) -> String {
    tracing::debug!("description: {:?}", description.value);
    if !out.is_empty() {
        out.push(crate::TOKEN_SEPARATOR);
    }
    out.push_str(&description.value);
    let out = write_task_project(out, &description.project);
    write_task_context(out, &description.context)
}

/// Write the task project to the output string.
#[tracing::instrument(parent = None, skip(project))]
fn write_task_project(mut out: String, project: &[String]) -> String {
    for project in project {
        tracing::debug!("project: {:?}", project);
        if !out.is_empty() {
            out.push(crate::TOKEN_SEPARATOR);
        }
        out.push(crate::PROJECT_MARKER);
        out.push_str(project);
    }
    out
}

/// Write the task context to the output string.
#[tracing::instrument(parent = None, skip(context))]
fn write_task_context(mut out: String, context: &[String]) -> String {
    for context in context {
        tracing::debug!("context: {:?}", context);
        if !out.is_empty() {
            out.push(crate::TOKEN_SEPARATOR);
        }
        out.push(crate::CONTEXT_MARKER);
        out.push_str(context);
    }
    out
}

#[cfg(test)]
mod test {
    #[test]
    fn test_write_task_state() {
        let state = todo_txt_model::TaskState::Todo;
        let result = super::write_task_state(String::new(), state);
        assert_eq!(result, "");
        let state = todo_txt_model::TaskState::Done;
        let result = super::write_task_state(String::new(), state);
        assert_eq!(result, "x");
    }

    #[test]
    fn test_write_task_priority() {
        let result = super::write_task_priority(String::new(), None);
        assert_eq!(result, "");
        let priority = todo_txt_model::TaskPriority::A;
        let result = super::write_task_priority(String::new(), Some(priority));
        assert_eq!(result, "(A)");
    }

    #[test]
    fn test_write_date() {
        let result = super::write_task_date(String::new(), None);
        assert_eq!(result, "");
        let created_date = chrono::NaiveDate::from_ymd_opt(2021, 1, 1);
        let result = super::write_task_date(String::new(), created_date);
        assert_eq!(result, "2021-01-01");
    }

    #[test]
    fn test_write_task_project() {
        let project = vec!["project".to_string()];
        let result = super::write_task_project(String::new(), &project);
        assert_eq!(result, "+project");
        let project = vec![];
        let result = super::write_task_project(String::new(), &project);
        assert_eq!(result, "");
    }

    #[test]
    fn test_write_task_context() {
        let context = vec!["context".to_string()];
        let result = super::write_task_context(String::new(), &context);
        assert_eq!(result, "@context");
        let context = vec![];
        let result = super::write_task_context(String::new(), &context);
        assert_eq!(result, "");
    }

    #[test]
    fn test_write_task_description() {
        let description = todo_txt_model::TaskDescription {
            value: "description".to_string(),
            project: vec!["project".to_string()],
            context: vec!["context".to_string()],
        };
        let result = super::write_task_description(String::new(), &description);
        assert_eq!(result, "description +project @context");

        let description = todo_txt_model::TaskDescription {
            value: "description".to_string(),
            project: vec![],
            context: vec![],
        };
        let result = super::write_task_description(String::new(), &description);
        assert_eq!(result, "description");
    }

    #[test]
    fn test_to_string() {
        let task = todo_txt_model::Task {
            state: todo_txt_model::TaskState::Todo,
            priority: None,
            completed_date: None,
            created_date: None,
            description: todo_txt_model::TaskDescription {
                value: "description".to_string(),
                project: vec!["project".to_string()],
                context: vec!["context".to_string()],
            },
        };
        let result = super::to_string(&task);
        assert_eq!(result, "description +project @context");
        let task = todo_txt_model::Task {
            state: todo_txt_model::TaskState::Done,
            priority: Some(todo_txt_model::TaskPriority::A),
            completed_date: chrono::NaiveDate::from_ymd_opt(2021, 1, 1),
            created_date: chrono::NaiveDate::from_ymd_opt(2021, 1, 1),
            description: todo_txt_model::TaskDescription {
                value: "description".to_string(),
                project: vec![],
                context: vec![],
            },
        };
        let result = super::to_string(&task);
        assert_eq!(result, "x (A) 2021-01-01 2021-01-01 description");
    }
}
