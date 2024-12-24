# tdr (Todo.txt rs) CLI User Guide

Show more options ```bash $ tdr --help```

## List of tasks
```bash
$ tdr       # Default command is list all tasks
$ tdr list  # List all tasks
$ tdr ls    # List alias
```

## Add a task
```bash
$ tdr add "Task description"    # Add a simply task
$ tdr a "Task description"      # Add alias
$ tdr a "Task description +Project @Context"  # Add a task with project and context
$ tdr a "Task description" "+Project" "@Context"  # Add a task with project and context
$ tdr a "+Project" "@Context" "Task description"  # You can put project and context before or after the task description
$ tdr a "(A) Task description"  # Add a task with priority
$ tdr a "(A) 2000-01-01 Task description"  # Add a task with priority and due date
$ tdr a "Task description" -p A --project Project --context Context  # Add a task with options
```

## Done a task
```bash
$ tdr done 1  # Done task number 1
$ tdr do 1    # Done alias
```

