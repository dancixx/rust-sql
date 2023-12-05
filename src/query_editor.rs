use std::{cell::RefCell, rc::Rc};

use leptos::*;
use monaco::{
    api::{CodeEditor, CodeEditorOptions},
    sys::editor::IDimension,
};
use wasm_bindgen::JsCast;

use crate::store::{db::DBStore, editor::EditorState};

pub type ModelCell = Rc<RefCell<Option<CodeEditor>>>;

#[component]
pub fn QueryEditor() -> impl IntoView {
    let query = use_context::<DBStore>().unwrap().query;
    let set_editor = use_context::<EditorState>().unwrap().editor;
    let node_ref = create_node_ref();

    node_ref.on_load(move |node| {
        let div_element: &web_sys::HtmlDivElement = &node;
        let html_element = div_element.unchecked_ref::<web_sys::HtmlElement>();
        let editor = CodeEditor::create(
            html_element,
            Some(CodeEditorOptions {
                value: query.get(),
                language: Some("sql".to_string()),
                automatic_layout: Some(true),
                dimension: Some(IDimension::new(0, 240)),
                ..Default::default()
            }),
        );

        set_editor.update(|prev| {
            prev.replace(Some(editor));
        });
    });

    view! { <div class="border-b-1 border-neutral-200 sticky" _ref=node_ref></div> }
}

