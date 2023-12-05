use leptos::{create_rw_signal, use_context, RwSignal, SignalGetUntracked, SignalUpdate};

use crate::{invoke::InvokeQueryArgs, wasm_functions::invoke};

use super::editor::EditorState;

#[derive(Clone, Copy, Debug)]
pub struct QueryState {
    pub query_result: RwSignal<Option<(Vec<String>, Vec<Vec<String>>)>>,
    pub is_loading: RwSignal<bool>,
}

impl Default for QueryState {
    fn default() -> Self {
        Self::new()
    }
}

impl QueryState {
    pub fn new() -> Self {
        Self {
            query_result: create_rw_signal(Some((Vec::new(), Vec::new()))),
            is_loading: create_rw_signal(false),
        }
    }

    pub async fn run_query(&self) {
        self.is_loading.update(|prev| {
            *prev = true;
        });
        let editor = use_context::<EditorState>().unwrap().editor.get_untracked();
        let code = editor
            .borrow()
            .as_ref()
            .unwrap()
            .get_model()
            .unwrap()
            .get_value();

        let args = serde_wasm_bindgen::to_value(&InvokeQueryArgs {
            sql: code.to_string(),
        })
        .unwrap();

        let data = invoke("get_query_result", args).await;
        let data = serde_wasm_bindgen::from_value::<(Vec<String>, Vec<Vec<String>>)>(data).unwrap();
        self.query_result.update(|prev| {
            *prev = Some(data);
        });
        self.is_loading.update(|prev| {
            *prev = false;
        });
    }
}

