use std::fmt::Display;

use common::enums::Project;
use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
pub enum Invoke {
  delete_project,
  delete_query,
  insert_project,
  insert_query,
  plugin_context_menu,
  postgresql_connector,
  select_projects,
  select_queries,
  select_schema_relations,
  select_schema_tables,
  select_sql_result,
}

impl Display for Invoke {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Invoke::delete_project => write!(f, "delete_project"),
      Invoke::delete_query => write!(f, "delete_query"),
      Invoke::insert_project => write!(f, "insert_project"),
      Invoke::insert_query => write!(f, "insert_query"),
      Invoke::postgresql_connector => write!(f, "postgresql_connector"),
      Invoke::plugin_context_menu => write!(f, "plugin:context_menu|show_context_menu"),
      Invoke::select_projects => write!(f, "select_projects"),
      Invoke::select_queries => write!(f, "select_queries"),
      Invoke::select_schema_relations => write!(f, "select_schema_relations"),
      Invoke::select_schema_tables => write!(f, "select_schema_tables"),
      Invoke::select_sql_result => write!(f, "select_sql_result"),
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct InvokePostgresConnectionArgs<'a> {
  pub project_name: &'a str,
  pub key: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct InvokeSchemaRelationsArgs<'a> {
  pub project_name: &'a str,
  pub schema: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct InvokeSchemaTablesArgs<'a> {
  pub project_name: &'a str,
  pub schema: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct InvokeSqlResultArgs<'a> {
  pub project_name: &'a str,
  pub sql: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct InvokeSelectProjectsArgs;

#[derive(Serialize, Deserialize)]
pub struct InvokeInsertProjectArgs {
  pub project: Project,
}

#[derive(Serialize, Deserialize)]
pub struct InvokeDeleteProjectArgs<'a> {
  pub project_name: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct InvokeInsertQueryArgs<'a> {
  pub key: &'a str,
  pub sql: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct InvokeSelectQueriesArgs;

#[derive(Serialize, Deserialize)]
pub struct InvokeDeleteQueryArgs<'a> {
  pub key: &'a str,
}

#[derive(Default, Serialize, Deserialize)]
pub struct InvokeContextMenuArgs<'a> {
  pub pos: Option<InvokeContextMenuPosition>,
  #[serde(borrow)]
  pub items: Option<Vec<InvokeContextMenuItem<'a>>>,
}

#[derive(Serialize, Deserialize)]
pub struct InvokeContextMenuItem<'a> {
  pub label: Option<&'a str>,
  pub disabled: Option<bool>,
  pub shortcut: Option<&'a str>,
  pub event: Option<&'a str>,
  pub payload: Option<&'a str>,
  pub subitems: Option<Vec<InvokeContextMenuItem<'a>>>,
  pub icon: Option<InvokeContextItemIcon<'a>>,
  pub checked: Option<bool>,
  pub is_separator: Option<bool>,
}

impl<'a> Default for InvokeContextMenuItem<'a> {
  fn default() -> Self {
    Self {
      label: None,
      disabled: Some(false),
      shortcut: None,
      event: None,
      payload: None,
      subitems: None,
      icon: None,
      checked: Some(false),
      is_separator: Some(false),
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct InvokeContextItemIcon<'a> {
  pub path: &'a str,
  pub width: Option<u32>,
  pub height: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct InvokeContextMenuPosition {
  pub x: f64,
  pub y: f64,
  pub is_absolute: Option<bool>,
}
