use std::{error::Error, future::pending};
use zbus::{ConnectionBuilder, dbus_interface};

use super::WorkspaceManager;

struct DbusServer {
    wm: WorkspaceManager
}

#[dbus_interface(name = "org.waayway.workspace_manager")]
impl DbusServer {
    fn message(&mut self, message: String) {
        self.wm.dispatch(message);
    }
}


pub async fn run_dbus(wm: &mut WorkspaceManager) {
    let server = DbusServer { wm: wm.clone() };

    let _conn = ConnectionBuilder::session().unwrap()
        .name("org.waayway.workspace_manager").unwrap()
        .serve_at("/org/waayway/workspace_manager", server).unwrap()
        .build()
        .await.unwrap();

    pending::<()>().await;
}