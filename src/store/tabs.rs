use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use leptos::{
  create_rw_signal, error::Result, use_context, RwSignal, SignalGetUntracked, SignalSet,
  SignalUpdate,
};
use monaco::api::CodeEditor;
use tauri_sys::tauri::invoke;

use crate::{
  invoke::{Invoke, InvokeSqlResultArgs},
  query_editor::ModelCell,
};

use super::{active_project::ActiveProjectStore, projects::ProjectsStore, query::QueryStore};

#[derive(Clone, Debug)]
struct QueryInfo {
  query: String,
  #[allow(dead_code)]
  start_line: f64,
  #[allow(dead_code)]
  end_line: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct TabsStore {
  pub active_tabs: RwSignal<usize>,
  pub selected_tab: RwSignal<String>,
  pub editors: RwSignal<BTreeMap<String, ModelCell>>,
  pub sql_results: RwSignal<BTreeMap<String, (Vec<String>, Vec<Vec<String>>)>>,
  pub is_loading: RwSignal<bool>,
}

impl Default for TabsStore {
  fn default() -> Self {
    Self::new()
  }
}

impl TabsStore {
  pub fn new() -> Self {
    Self {
      active_tabs: create_rw_signal(1),
      selected_tab: create_rw_signal(String::from("0")),
      editors: create_rw_signal(BTreeMap::new()),
      sql_results: create_rw_signal(BTreeMap::new()),
      is_loading: create_rw_signal(false),
    }
  }

  pub async fn run_query(&self) -> Result<()> {
    self.is_loading.set(true);
    let active_project = use_context::<ActiveProjectStore>().unwrap();
    let active_project = active_project.0.get_untracked().unwrap();
    let projects_store = use_context::<ProjectsStore>().unwrap();
    projects_store.connect(&active_project).await?;
    let active_editor = self.select_active_editor();
    let position = active_editor
      .borrow()
      .as_ref()
      .unwrap()
      .as_ref()
      .get_position()
      .unwrap();
    let sql = self.select_active_editor_value();
    let sql = self
      .find_query_for_line(&sql, position.line_number())
      .unwrap();
    let data = invoke::<_, (Vec<String>, Vec<Vec<String>>)>(
      &Invoke::select_sql_result.to_string(),
      &InvokeSqlResultArgs {
        project_name: &active_project,
        sql: &sql.query,
      },
    )
    .await?;
    self.sql_results.update(|prev| {
      prev.insert(self.selected_tab.get_untracked(), data);
    });
    self.is_loading.set(false);
    Ok(())
  }

  pub fn load_query(&self, key: &str) -> Result<()> {
    let active_project = use_context::<ActiveProjectStore>().unwrap();
    let splitted_key = key.split(':').collect::<Vec<&str>>();
    active_project.0.set(Some(splitted_key[0].to_string()));
    let query_store = use_context::<QueryStore>().unwrap();
    let query_store = query_store.0.get_untracked();
    let query = query_store.get(key).unwrap();

    self.set_editor_value(&query);
    Ok(())
  }

  pub fn select_active_editor_sql_result(&self) -> Option<(Vec<String>, Vec<Vec<String>>)> {
    match self
      .sql_results
      .get_untracked()
      .get(&self.selected_tab.get_untracked())
    {
      Some(result) => Some(result.clone()),
      None => None,
    }
  }

  pub fn add_editor(&mut self, editor: Rc<RefCell<Option<CodeEditor>>>) {
    self.editors.update(|prev| {
      prev.insert((self.active_tabs.get_untracked() - 1).to_string(), editor);
    });
  }

  #[allow(dead_code)]
  pub fn remove_editor(&mut self, tab_key: &str) {
    self.editors.update(|prev| {
      prev.remove(tab_key);
    });
  }

  pub fn select_active_editor(&self) -> ModelCell {
    self
      .editors
      .get_untracked()
      .get(&self.selected_tab.get_untracked())
      .unwrap()
      .clone()
  }

  pub fn select_active_editor_value(&self) -> String {
    self
      .editors
      .get_untracked()
      .get(&self.selected_tab.get_untracked())
      .unwrap()
      .borrow()
      .as_ref()
      .unwrap()
      .get_model()
      .unwrap()
      .get_value()
  }

  pub fn set_editor_value(&self, value: &str) {
    self
      .editors
      .get_untracked()
      .get(&self.selected_tab.get_untracked())
      .unwrap()
      .borrow()
      .as_ref()
      .unwrap()
      .get_model()
      .unwrap()
      .set_value(value);
  }

  // TODO: improve this
  pub(self) fn find_query_for_line(&self, queries: &str, line_number: f64) -> Option<QueryInfo> {
    let mut start_line = 1f64;
    let mut end_line = 1f64;
    let mut current_query = String::new();

    for line in queries.lines() {
      if !current_query.is_empty() {
        current_query.push('\n');
      }
      current_query.push_str(line);
      end_line += 1f64;

      if line.ends_with(';') {
        if line_number >= start_line && line_number < end_line {
          return Some(QueryInfo {
            query: current_query.clone(),
            start_line,
            end_line: end_line - 1f64,
          });
        }
        start_line = end_line;
        current_query.clear();
      }
    }

    None
  }
}
