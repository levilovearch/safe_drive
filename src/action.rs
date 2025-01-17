#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GoalStatus {
    Unknown,
    Accepted,
    Executing,
    Canceling,
    Succeeded,
    Canceled,
    Aborted,
}

impl From<i8> for GoalStatus {
    fn from(s: i8) -> Self {
        match s {
            0 => GoalStatus::Unknown,
            1 => GoalStatus::Accepted,
            2 => GoalStatus::Executing,
            3 => GoalStatus::Canceling,
            4 => GoalStatus::Succeeded,
            5 => GoalStatus::Canceled,
            6 => GoalStatus::Aborted,
            _ => unreachable!(),
        }
    }
}

impl From<GoalStatus> for i8 {
    fn from(s: GoalStatus) -> Self {
        match s {
            GoalStatus::Unknown => 0,
            GoalStatus::Accepted => 1,
            GoalStatus::Executing => 2,
            GoalStatus::Canceling => 3,
            GoalStatus::Succeeded => 4,
            GoalStatus::Canceled => 5,
            GoalStatus::Aborted => 6,
        }
    }
}
