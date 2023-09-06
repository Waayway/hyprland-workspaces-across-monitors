use hyprland::data::Monitor;
use hyprland::data::Monitors;
use hyprland::dispatch;
use hyprland::dispatch::*;
use hyprland::keyword::*;
use hyprland::prelude::*;

use self::dispatchers::Actions;

mod dbus;
mod dispatchers;

#[derive(Debug, Clone)]
struct Display {
    name: String,
    num: usize,
    cur_workspace: usize
}
impl Display {
    pub fn get_displays() -> Vec<Self> {
        let mut displays = Vec::new();
        let monitors = match Monitors::get() {
            Ok(monitors) => monitors,
            Err(_) => return displays
        };
        for mon in monitors.to_vec() {
            displays.push(Self {
                cur_workspace: mon.active_workspace.id as usize,
                name: mon.name,
                num: mon.id as usize
            })
        }
        displays
    }
}

#[derive(Debug, Clone)]
pub struct WorkspaceManager {
    displays: Vec<Display>,
    cur_global_workspace: usize,
    workspaces_per_display: usize
}

impl WorkspaceManager {
    pub fn new(workspaces_per_monitor: usize) -> Self {
        let displays = Display::get_displays();
        println!("Displays: {:?}", displays);
        Self {
            displays: displays,
            cur_global_workspace: 0,
            workspaces_per_display: workspaces_per_monitor
        }
    }
    pub fn dispatch(&mut self, msg: String) {
        let action = Actions::from_string(msg);
        let cur_monitor = match Monitor::get_active() {
            Ok(monitor) => monitor,
            Err(_) => return
        };
        match action {
            Actions::Invalid => println!("Invalid action"),
            Actions::Workspace(num) => {
                for dis in self.displays.clone().iter().filter(|dis| dis.num != cur_monitor.id as usize) {
                    let workspace_num = num + (dis.num * self.workspaces_per_display);
                    dispatch!(FocusMonitor, MonitorIdentifier::Name(&dis.name.clone()));
                    dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(workspace_num as i32));
                }
                let workspace_num = num + (cur_monitor.id as usize * self.workspaces_per_display);
                dispatch!(FocusMonitor, MonitorIdentifier::Name(&cur_monitor.name.clone()));
                dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(workspace_num as i32));
            },
            Actions::MoveToWorkspace(num) => {
                let workspace_num = num + (cur_monitor.id as usize * self.workspaces_per_display);
                dispatch!(FocusMonitor, MonitorIdentifier::Name(&cur_monitor.name.clone()));
                dispatch!(MoveToWorkspace, WorkspaceIdentifierWithSpecial::Id(workspace_num as i32), None);
            },
            Actions::MoveToWorkspaceSilent(num) => {
                let workspace_num = num + (cur_monitor.id as usize * self.workspaces_per_display);
                dispatch!(MoveToWorkspaceSilent, WorkspaceIdentifierWithSpecial::Id(workspace_num as i32), None);
            },
        }
    }
}

pub fn main() {
    println!("Hello from server!");
    let mut wm = WorkspaceManager::new(10);
    async_std::task::block_on(dbus::run_dbus(&mut wm));
} 