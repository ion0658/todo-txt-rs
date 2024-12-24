mod error;

pub mod prelude {
    pub use crate::error::{Error as TodoTxtRsError, Result};
    pub use crate::*;
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Task {
    pub state: TaskState,
    pub priority: Option<TaskPriority>,
    pub completed_date: Option<chrono::NaiveDate>,
    pub created_date: Option<chrono::NaiveDate>,
    pub description: TaskDescription,
}

impl Task {
    pub fn is_done(&self) -> bool {
        self.state == TaskState::Done
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, clap::ValueEnum)]
pub enum TaskState {
    #[default]
    Todo,
    Done,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, clap::ValueEnum)]
#[clap(rename_all = "uppercase")]
pub enum TaskPriority {
    #[default]
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl From<TaskPriority> for char {
    fn from(value: TaskPriority) -> Self {
        match value {
            TaskPriority::A => 'A',
            TaskPriority::B => 'B',
            TaskPriority::C => 'C',
            TaskPriority::D => 'D',
            TaskPriority::E => 'E',
            TaskPriority::F => 'F',
            TaskPriority::G => 'G',
            TaskPriority::H => 'H',
            TaskPriority::I => 'I',
            TaskPriority::J => 'J',
            TaskPriority::K => 'K',
            TaskPriority::L => 'L',
            TaskPriority::M => 'M',
            TaskPriority::N => 'N',
            TaskPriority::O => 'O',
            TaskPriority::P => 'P',
            TaskPriority::Q => 'Q',
            TaskPriority::R => 'R',
            TaskPriority::S => 'S',
            TaskPriority::T => 'T',
            TaskPriority::U => 'U',
            TaskPriority::V => 'V',
            TaskPriority::W => 'W',
            TaskPriority::X => 'X',
            TaskPriority::Y => 'Y',
            TaskPriority::Z => 'Z',
        }
    }
}

impl From<char> for TaskPriority {
    fn from(value: char) -> Self {
        let value = value.to_ascii_uppercase();
        match value {
            'A' => TaskPriority::A,
            'B' => TaskPriority::B,
            'C' => TaskPriority::C,
            'D' => TaskPriority::D,
            'E' => TaskPriority::E,
            'F' => TaskPriority::F,
            'G' => TaskPriority::G,
            'H' => TaskPriority::H,
            'I' => TaskPriority::I,
            'J' => TaskPriority::J,
            'K' => TaskPriority::K,
            'L' => TaskPriority::L,
            'M' => TaskPriority::M,
            'N' => TaskPriority::N,
            'O' => TaskPriority::O,
            'P' => TaskPriority::P,
            'Q' => TaskPriority::Q,
            'R' => TaskPriority::R,
            'S' => TaskPriority::S,
            'T' => TaskPriority::T,
            'U' => TaskPriority::U,
            'V' => TaskPriority::V,
            'W' => TaskPriority::W,
            'X' => TaskPriority::X,
            'Y' => TaskPriority::Y,
            'Z' => TaskPriority::Z,
            _ => TaskPriority::A,
        }
    }
}

impl From<i32> for TaskPriority {
    fn from(value: i32) -> Self {
        match value {
            1 => TaskPriority::A,
            2 => TaskPriority::B,
            3 => TaskPriority::C,
            4 => TaskPriority::D,
            5 => TaskPriority::E,
            6 => TaskPriority::F,
            7 => TaskPriority::G,
            8 => TaskPriority::H,
            9 => TaskPriority::I,
            10 => TaskPriority::J,
            11 => TaskPriority::K,
            12 => TaskPriority::L,
            13 => TaskPriority::M,
            14 => TaskPriority::N,
            15 => TaskPriority::O,
            16 => TaskPriority::P,
            17 => TaskPriority::Q,
            18 => TaskPriority::R,
            19 => TaskPriority::S,
            20 => TaskPriority::T,
            21 => TaskPriority::U,
            22 => TaskPriority::V,
            23 => TaskPriority::W,
            24 => TaskPriority::X,
            25 => TaskPriority::Y,
            26 => TaskPriority::Z,
            _ => TaskPriority::A,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct TaskDescription {
    pub value: String,
    pub project: Vec<String>,
    pub context: Vec<String>,
}
