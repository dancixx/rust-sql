use ahash::AHashMap;
use common::enums::ProjectConnectionStatus;
use leptos::{error::Result, RwSignal, SignalGet, SignalSet, SignalUpdate};
use tauri_sys::tauri::invoke;

use crate::invoke::{
  Invoke, InvokePgsqlConnectorArgs, InvokePgsqlLoadSchemasArgs, InvokePgsqlLoadTablesArgs,
};

#[derive(Debug, Clone, Copy)]
pub struct Pgsql<'a> {
  pub project_id: RwSignal<String>,
  user: Option<&'a str>,
  password: Option<&'a str>,
  host: Option<&'a str>,
  port: Option<&'a str>,
  pub status: RwSignal<ProjectConnectionStatus>,
  pub schemas: RwSignal<Vec<String>>,
  pub tables: RwSignal<AHashMap<String, Vec<(String, String)>>>,
}

impl<'a> Pgsql<'a> {
  pub fn new(project_id: String) -> Self {
    Self {
      project_id: RwSignal::new(project_id),
      status: RwSignal::default(),
      schemas: RwSignal::default(),
      tables: RwSignal::default(),
      user: None,
      password: None,
      host: None,
      port: None,
    }
  }

  pub async fn connector(&self) -> Result<ProjectConnectionStatus> {
    self
      .status
      .update(|prev| *prev = ProjectConnectionStatus::Connecting);
    let connection_string = self.generate_connection_string();
    let status = invoke::<_, ProjectConnectionStatus>(
      Invoke::PgsqlConnector.as_ref(),
      &InvokePgsqlConnectorArgs {
        project_id: &self.project_id.get(),
        key: connection_string.as_str(),
      },
    )
    .await
    .unwrap();
    if status == ProjectConnectionStatus::Connected {
      self.load_schemas().await;
    }
    self.status.update(|prev| *prev = status.clone());
    Ok(status)
  }

  pub async fn load_schemas(&self) {
    let schemas = invoke::<_, Vec<String>>(
      Invoke::PgsqlLoadSchemas.as_ref(),
      &InvokePgsqlLoadSchemasArgs {
        project_id: &self.project_id.get(),
      },
    )
    .await
    .unwrap();
    self.schemas.set(schemas);
  }

  pub async fn load_tables(&self, schema: &str) {
    if self.tables.get().contains_key(schema) {
      return;
    }
    let tables = invoke::<_, Vec<(String, String)>>(
      Invoke::PgsqlLoadTables.as_ref(),
      &InvokePgsqlLoadTablesArgs {
        project_id: &self.project_id.get(),
        schema,
      },
    )
    .await
    .unwrap();
    self.tables.update(|prev| {
      prev.insert(schema.to_owned(), tables);
    });
  }

  #[allow(dead_code)]
  pub async fn run_query() {
    unimplemented!()
  }

  pub fn select_tables_by_schema(&self, schema: &str) -> Option<Vec<(String, String)>> {
    self.tables.get().get(schema).cloned()
  }

  pub fn load_connection_details(
    &mut self,
    user: &'a str,
    password: &'a str,
    host: &'a str,
    port: &'a str,
  ) {
    self.user = Some(user);
    self.password = Some(password);
    self.host = Some(host);
    self.port = Some(port);
  }

  fn generate_connection_string(&self) -> String {
    let connection_string = format!(
      "user={} password={} host={} port={}",
      self.user.as_ref().unwrap(),
      self.password.as_ref().unwrap(),
      self.host.as_ref().unwrap(),
      self.port.as_ref().unwrap(),
    );
    connection_string
  }
}
