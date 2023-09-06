#[derive(Debug, Clone)]
pub enum Actions {
    Invalid,
    Workspace(usize),
    MoveToWorkspace(usize),
    MoveToWorkspaceSilent(usize),
}

impl Actions {
    pub fn from_string(action: String) -> Self {
        let mut act = action.split(" ").collect::<Vec<&str>>();
        let first = act.remove(0);
        match first {
            "workspace" => match act.first() {
                Some(num) => Actions::Workspace(num.parse::<usize>().unwrap_or(0)),
                None => Actions::Invalid
            }, 
            "movetoworkspace" => match act.first() {
                Some(num) => Actions::MoveToWorkspace(num.parse::<usize>().unwrap_or(0)),
                None => Actions::Invalid
            },
            "movetoworkspacesilent" => match act.first() {
                Some(num) => Actions::MoveToWorkspaceSilent(num.parse::<usize>().unwrap_or(0)),
                None => Actions::Invalid
            },
            _ => Actions::Invalid
        }
    }
}