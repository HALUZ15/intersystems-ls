use crate::protocol::*;
use irisnative::{connection::*, Connection};
use once_cell::sync::OnceCell;
use log::trace;

#[allow(dead_code)]
struct ConnectionSettings {
  host: String,
  port: u16,
  super_port: u16,
  ns: String,
  username: String,
  password: String,
}

pub struct ConnectionManager {
  connection_settings: OnceCell<ConnectionSettings>,
}

impl ConnectionManager {
  pub fn new() -> Self {
    Self {
      connection_settings: OnceCell::new(),
    }
  }

  pub async fn reparse(&self, options: &Options) {
    trace!("config reparsee: {:?}", options);
    let objectscript = options.objectscript.as_ref().cloned().unwrap_or_default();
    let conn = objectscript.conn.unwrap_or_default();
    let host = conn.host.unwrap_or_default();
    let port = conn.port.unwrap_or_default();
    let super_port = conn.super_port.unwrap_or_default();
    let username = conn.username.unwrap_or_default();
    let password = conn.password.unwrap_or_default();
    let ns = conn.ns.unwrap_or_default();
    let active = conn.active.unwrap_or(true)
      && !host.is_empty()
      && !username.is_empty()
      && !password.is_empty()
      && !ns.is_empty()
      && super_port > 0;
    if active {
      // if let Ok(mut connection) = Connection::connect(host, super_port, ns, username, password) {
      // 	let version = connection.server_version();
      // 	trace!("Connected to: {}", version);
      // 	// let _ = self.connection.set(connection);
      // 	let _ = self
      // 		.client
      // 		.connected(InterSystemsConnectedParams { version });
      // }
      let _ = self.connection_settings.set(ConnectionSettings {
        host,
        port,
        super_port,
        ns,
        username,
        password,
      });
    }
  }

  pub fn connect(&self) -> Option<Connection> {
    if let Ok(connection) = Connection::connect(
      String::from("localhost"),
      1972,
      String::from("USER"),
      String::from("_SYSTEM"),
      String::from("SYS"),
    ) {
      Some(connection)
    } else {
      None
    }
    // if let Some(ConnectionSettings {
    // 	host, super_port, ns, username, password, ..
    // }) = self.connection_settings.get() {
    // 	if let Ok(connection) = Connection::connect(host.to_owned(), super_port.to_owned(), ns.to_owned(), username.to_owned(), password.to_owned()) {
    // 		Some(connection)
    // 	} else {
    // 		None
    // 	}
    // } else {
    // 	None
    // }
  }

  pub async fn productions(&self) -> Vec<Production> {
    if let Some(mut connection) = self.connect() {
      let mut list = Vec::new();
      let (curprod, curstate) = connection.production_state();
      let curstatus = match curstate {
        1 => "Running",
        2 => "Stopped",
        3 => "Suspended",
        4 => "Troubled",
        _ => "Unknown",
      };

      let mut rs = connection.query(String::from(
				"select Name from %Dictionary.ClassDefinition where super = 'Ens.Production' and abstract<>1",
			));
      while rs.next() {
        let id: String = rs.get(0).unwrap();
        let status = String::from(if id == curprod { curstatus } else { "Stopped" });
        list.push(Production { id, status });
      }
      list
    } else {
      Vec::new()
    }
  }

  pub async fn production_services(&self, production: String) -> Vec<ProductionService> {
    if let Some(mut connection) = self.connect() {
      let mut list = Vec::new();

      let mut rs = connection.query(format!(
				"select name from ens_config.item where production='{}'
        and 
        classname in (select name from %dictionary.classdefinition where super='Ens.BusinessService')", production
			));
      while rs.next() {
        let id: String = rs.get(0).unwrap();
        list.push(ProductionService { id });
      }
      list
    } else {
      Vec::new()
    }
  }

  pub async fn production_operations(&self, production: String) -> Vec<ProductionOperation> {
    if let Some(mut connection) = self.connect() {
      let mut list = Vec::new();

      let mut rs = connection.query(format!(
				"select name from ens_config.item where production='{}'
        and 
        classname in (select name from %dictionary.classdefinition where super='Ens.BusinessOperation')", production
			));
      while rs.next() {
        let id: String = rs.get(0).unwrap();
        list.push(ProductionOperation { id });
      }
      list
    } else {
      Vec::new()
    }
  }

  pub async fn production_processes(&self, production: String) -> Vec<ProductionProcess> {
    if let Some(mut connection) = self.connect() {
      let mut list = Vec::new();

      let mut rs = connection.query(format!(
				"select name from ens_config.item where production='{}'
        and 
        classname in (select name from %dictionary.classdefinition where super='Ens.BusinessProcess')", production
			));
      while rs.next() {
        let id: String = rs.get(0).unwrap();
        list.push(ProductionProcess { id });
      }
      list
    } else {
      Vec::new()
    }
  }

  pub async fn globals(&self) -> Vec<Global> {
    if let Some(mut connection) = self.connect() {
      let mut list = Vec::new();
      let mut rs = connection.query(String::from(
        "SELECT DISTINCT '^' || $piece(name,'(',1) Name from %SYS.GlobalQuery_NamespaceList()",
      ));
      while rs.next() {
        let name: String = rs.get(0).unwrap();
        list.push(Global { name });
      }
      list
    } else {
      Vec::new()
    }
  }
}
