use zbus::{dbus_proxy, Result, blocking::Connection};


#[dbus_proxy(
    interface = "org.waayway.workspace_manager",
    default_service = "org.waayway.workspace_manager",
    default_path = "/org/waayway/workspace_manager"
)]
trait DbusClient {
    async fn message(&self, message: String) -> Result<()>;
}

pub fn main(args: Vec<String>) {
    let con = Connection::session().unwrap();
    let proxy = DbusClientProxyBlocking::new(&con).unwrap();
    proxy.message(args.join(" ")).unwrap();
}