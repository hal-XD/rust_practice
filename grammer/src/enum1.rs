
type UserName = String;
#[derive(Debug)]
pub enum Task {
    Open,
    AssignedTo(UserName),
    Working {
        assignee : UserName,
        remaining_hours : u16,
    },
    Done,
}