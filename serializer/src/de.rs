use todo_txt_model::prelude::*;

/// Parse a task from a string.
/// # Arguments
/// * `value` - The string to parse. It will be trimmed and it should not be empty.
///
/// # Examples
///
/// ```
/// let task = todo_txt_serializer::from_str("x (A) 2020-01-01 2020-01-02 123 +project +project2 @context @ctx2 abc description");
/// let expected = todo_txt_model::Task {
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
/// assert_eq!(task.unwrap(), expected);
/// ```
#[tracing::instrument(parent = None)]
pub fn from_str(value: &str) -> Result<todo_txt_model::Task> {
    let value = value.trim();
    if value.is_empty() {
        return Err(TodoTxtRsError::Syntax);
    }

    let tokens = value
        .split(crate::TOKEN_SEPARATOR)
        .map(Token::from)
        .peekable();

    let (state, tokens) = get_task_state(tokens)?;
    tracing::debug!("state: {:?}", state);
    let (priority, tokens) = get_task_priority(tokens)?;
    tracing::debug!("priority: {:?}", priority);
    let (date_1, tokens) = get_date(tokens)?;
    let (date_2, tokens) = if date_1.is_some() {
        get_date(tokens)?
    } else {
        (None, tokens)
    };
    tracing::debug!("date_1: {:?}, date_2: {:?}", date_1, date_2);

    let mut description = String::new();
    let mut projects = Vec::new();
    let mut contexts = Vec::new();
    for token in tokens {
        match token {
            Token::Project(p) => {
                tracing::debug!("token project: {}, projects: {:?}", p, projects);
                projects.push(p.to_string());
            }
            Token::Context(c) => {
                tracing::debug!("token context: {}, contexts: {:?}", c, contexts);
                contexts.push(c.to_string());
            }
            Token::Description(d) => {
                if !description.is_empty() {
                    description.push(crate::TOKEN_SEPARATOR);
                }
                tracing::debug!("token description: {}, description: {}", d, description);
                description.push_str(d);
            }
            _ => return Err(TodoTxtRsError::Syntax),
        }
    }
    let projects = distinct_vec_hold_order(projects);
    let contexts = distinct_vec_hold_order(contexts);
    tracing::debug!("description: {:?}", description);
    tracing::debug!("projects: {:?}", projects);
    tracing::debug!("contexts: {:?}", contexts);

    if description.is_empty() {
        return Err(TodoTxtRsError::Syntax);
    }

    Ok(todo_txt_model::Task {
        state,
        priority,
        completed_date: match (state, date_1, date_2) {
            (todo_txt_model::TaskState::Done, Some(d), None) => Some(d),
            (_, Some(d1), Some(_d2)) => Some(d1),
            _ => None,
        },
        created_date: match (state, date_1, date_2) {
            (todo_txt_model::TaskState::Todo, Some(d), None) => Some(d),
            (_, Some(_d1), Some(d2)) => Some(d2),
            _ => None,
        },
        description: todo_txt_model::TaskDescription {
            value: description,
            project: projects,
            context: contexts,
        },
    })
}

fn distinct_vec_hold_order<T: Clone + Eq + std::hash::Hash>(vec: Vec<T>) -> Vec<T> {
    let mut set = std::collections::HashSet::new();
    let mut result = Vec::new();
    for item in vec {
        if set.insert(item.clone()) {
            result.push(item);
        }
    }
    result
}

#[derive(Debug)]
enum Token<'a> {
    Done,
    Priority(&'a str),
    Date(chrono::NaiveDate),
    Description(&'a str),
    Project(&'a str),
    Context(&'a str),
}

impl<'a> From<&'a str> for Token<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            t if t == crate::COMPLETE_MARKER.to_string() => Self::Done,
            t if t.starts_with(crate::PRIORITY_MARKER_PRE)
                && t.ends_with(crate::PRIORITY_MARKER_POST)
                && t.len() == 3 =>
            {
                Self::Priority(
                    t.trim_start_matches(crate::PRIORITY_MARKER_PRE)
                        .trim_end_matches(crate::PRIORITY_MARKER_POST),
                )
            }
            t if chrono::NaiveDate::parse_from_str(t, "%Y-%m-%d").is_ok() => {
                Self::Date(chrono::NaiveDate::parse_from_str(t, "%Y-%m-%d").unwrap())
            }
            t if t.starts_with(crate::PROJECT_MARKER) => {
                Self::Project(t.trim_start_matches(crate::PROJECT_MARKER))
            }
            t if t.starts_with(crate::CONTEXT_MARKER) => {
                Self::Context(t.trim_start_matches(crate::CONTEXT_MARKER))
            }
            t => Self::Description(t),
        }
    }
}

#[tracing::instrument(parent = None, skip(tokens))]
fn get_task_state<'a, IT>(
    mut tokens: std::iter::Peekable<IT>,
) -> Result<(todo_txt_model::TaskState, std::iter::Peekable<IT>)>
where
    IT: Iterator<Item = Token<'a>>,
{
    let token = tokens.peek().ok_or(TodoTxtRsError::Syntax)?;
    tracing::debug!("token {:?}", token);
    let state = match token {
        Token::Done => {
            tokens.next();
            todo_txt_model::TaskState::Done
        }
        _ => todo_txt_model::TaskState::Todo,
    };
    Ok((state, tokens))
}

#[tracing::instrument(parent = None, skip(tokens))]
fn get_task_priority<'a, IT>(
    mut tokens: std::iter::Peekable<IT>,
) -> Result<(
    Option<todo_txt_model::TaskPriority>,
    std::iter::Peekable<IT>,
)>
where
    IT: Iterator<Item = Token<'a>>,
{
    let token = tokens.peek().ok_or(TodoTxtRsError::Syntax)?;
    tracing::debug!("token {:?}", token);
    let priority = match *token {
        Token::Priority(p) => {
            if let Some(p) = p.chars().next() {
                tokens.next();
                Some(todo_txt_model::TaskPriority::from(p))
            } else {
                return Err(TodoTxtRsError::Syntax);
            }
        }
        _ => None,
    };
    Ok((priority, tokens))
}

#[tracing::instrument(parent = None, skip(tokens))]
fn get_date<'a, IT>(
    mut tokens: std::iter::Peekable<IT>,
) -> Result<(Option<chrono::NaiveDate>, std::iter::Peekable<IT>)>
where
    IT: Iterator<Item = Token<'a>>,
{
    let token = tokens.peek().ok_or(TodoTxtRsError::Syntax)?;
    tracing::debug!("token {:?}", token);
    let date = match *token {
        Token::Date(d) => {
            tokens.next();
            Some(d)
        }
        _ => None,
    };
    Ok((date, tokens))
}

#[cfg(test)]
mod test {

    #[test]
    fn test_from_str_fully() {
        let task = super::from_str(
            "x (A) 2020-01-01 2020-01-02 123 +project +project2 @context @ctx2 abc description",
        );
        assert!(task.is_ok());
        let task = task.unwrap();
        assert_eq!(
            task,
            todo_txt_model::Task {
                state: todo_txt_model::TaskState::Done,
                priority: Some(todo_txt_model::TaskPriority::A),
                completed_date: chrono::NaiveDate::from_ymd_opt(2020, 1, 1),
                created_date: chrono::NaiveDate::from_ymd_opt(2020, 1, 2),
                description: todo_txt_model::TaskDescription {
                    value: "123 abc description".to_string(),
                    project: vec!["project".to_string(), "project2".to_string()],
                    context: vec!["context".to_string(), "ctx2".to_string()],
                }
            }
        )
    }

    #[test]
    fn test_from_str_minimal() {
        let task = super::from_str("description");
        assert!(task.is_ok());
        let task = task.unwrap();
        assert_eq!(
            task,
            todo_txt_model::Task {
                state: todo_txt_model::TaskState::Todo,
                priority: None,
                completed_date: None,
                created_date: None,
                description: todo_txt_model::TaskDescription {
                    value: "description".to_string(),
                    project: Vec::new(),
                    context: Vec::new(),
                }
            }
        );
        let task = super::from_str("x description");
        assert!(task.is_ok());
        let task = task.unwrap();
        assert_eq!(
            task,
            todo_txt_model::Task {
                state: todo_txt_model::TaskState::Done,
                priority: None,
                completed_date: None,
                created_date: None,
                description: todo_txt_model::TaskDescription {
                    value: "description".to_string(),
                    project: Vec::new(),
                    context: Vec::new(),
                }
            }
        )
    }
}
