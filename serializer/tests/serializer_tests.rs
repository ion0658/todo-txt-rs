use todo_txt_serializer::prelude::*;

#[test]
fn test_from_str_minimal() {
    let task = from_str("task");
    assert!(task.is_ok());
    let task = task.unwrap();
    let str = to_string(&task);
    assert_eq!(str, "task");
    let task = from_str("x task");
    assert!(task.is_ok());
    let task = task.unwrap();
    let str = to_string(&task);
    assert_eq!(str, "x task");
}

#[test]
fn test_from_str_fully() {
    let task =
        from_str("x (A) 2021-01-01 2021-01-02 +proj1 task +project hello @context abc @ctx2");
    assert!(task.is_ok());
    let task = task.unwrap();
    let str = to_string(&task);
    assert_eq!(
        str,
        "x (A) 2021-01-01 2021-01-02 task hello abc +proj1 +project @context @ctx2"
    );
}

#[test]
fn test_to_string() {
    let task = todo_txt_model::Task {
        state: todo_txt_model::TaskState::Done,
        priority: Some(todo_txt_model::TaskPriority::A),
        completed_date: chrono::NaiveDate::from_ymd_opt(2021, 1, 1),
        created_date: chrono::NaiveDate::from_ymd_opt(2021, 1, 2),
        description: todo_txt_model::TaskDescription {
            value: "task hello abc".to_string(),
            project: vec!["proj1".to_string(), "project".to_string()],
            context: vec!["context".to_string(), "ctx2".to_string()],
        },
    };
    let str = to_string(&task);
    assert_eq!(
        str,
        "x (A) 2021-01-01 2021-01-02 task hello abc +proj1 +project @context @ctx2"
    );
    let expected = from_str(&str);
    assert!(expected.is_ok());
    let expected = expected.unwrap();
    assert_eq!(expected, task);
}
