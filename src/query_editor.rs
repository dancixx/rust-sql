use std::{cell::RefCell, rc::Rc, sync::Arc};

use futures::lock::Mutex;
use leptos::*;
use leptos_use::{use_document, use_event_listener};
use monaco::{
  api::{CodeEditor, CodeEditorOptions, TextModel},
  sys::{
    editor::{IDimension, IEditorMinimapOptions},
    KeyCode, KeyMod,
  },
};
use wasm_bindgen::{closure::Closure, JsCast};

use crate::{modals::custom_query::CustomQuery, store::tabs::TabsStore};

pub type ModelCell = Rc<RefCell<Option<CodeEditor>>>;
pub const MODE_ID: &str = "pgsql";

#[component]
pub fn QueryEditor() -> impl IntoView {
  let tabs_store = Rc::new(RefCell::new(use_context::<TabsStore>().unwrap()));
  let show = create_rw_signal(false);
  let _ = use_event_listener(use_document(), ev::keydown, move |event| {
    if event.key() == "Escape" {
      show.set(false);
    }
  });
  let node_ref = create_node_ref();

  let tabs_store_clone = tabs_store.clone();
  node_ref.on_load(move |node| {
    let div_element: &web_sys::HtmlDivElement = &node;
    let html_element = div_element.unchecked_ref::<web_sys::HtmlElement>();
    let options = CodeEditorOptions::default().to_sys_options();
    let text_model =
      TextModel::create("SELECT * FROM users LIMIT 100;", Some(MODE_ID), None).unwrap();
    options.set_model(Some(text_model.as_ref()));
    options.set_language(Some(MODE_ID));
    options.set_automatic_layout(Some(true));
    options.set_dimension(Some(&IDimension::new(0, 240)));
    let minimap_settings = IEditorMinimapOptions::default();
    minimap_settings.set_enabled(Some(false));
    options.set_minimap(Some(&minimap_settings));

    let e = CodeEditor::create(html_element, Some(options));
    let keycode = KeyMod::win_ctrl() as u32 | KeyCode::Enter.to_value();
    // TODO: Fix this
    e.as_ref().add_command(
      keycode.into(),
      Closure::<dyn Fn()>::new(|| ()).as_ref().unchecked_ref(),
      None,
    );

    // TODO: Fix this
    let e = Rc::new(RefCell::new(Some(e)));
    tabs_store_clone.borrow_mut().add_editor(e);
  });
  let tabs_store = Arc::new(Mutex::new(use_context::<TabsStore>().unwrap()));
  let run_query = create_action(move |tabs_store: &Arc<Mutex<TabsStore>>| {
    let tabs_store = tabs_store.clone();
    async move {
      tabs_store.lock().await.run_query().await.unwrap();
    }
  });
  let tabs_store_clone = tabs_store.clone();
  let _ = use_event_listener(node_ref, ev::keydown, move |event| {
    if event.key() == "Enter" && event.ctrl_key() {
      run_query.dispatch(tabs_store_clone.clone());
    }
  });

  view! {
      <div _ref=node_ref class="border-b-1 border-neutral-200 h-72 sticky">
          <CustomQuery show=show/>
          <div class="absolute bottom-0 items-center flex justify-end px-4 left-0 w-full h-10 bg-gray-50">
              <div class="flex flex-row gap-2 text-xs">
                  <button
                      class="p-1 border-1 border-neutral-200 bg-white hover:bg-neutral-200 rounded-md"
                      on:click=move |_| show.set(true)
                  >
                      "Save Query"
                  </button>
                  <button
                      class="p-1 border-1 border-neutral-200 bg-white hover:bg-neutral-200 rounded-md"
                      on:click=move |_| run_query.dispatch(tabs_store.clone())
                  >
                      "Query"
                  </button>
              </div>
          </div>
      </div>
  }
}

