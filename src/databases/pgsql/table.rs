use std::rc::Rc;

use leptos::*;
use leptos_icons::*;

use crate::databases::pgsql::driver::Pgsql;

#[component]
pub fn Table(table: (String, String), schema: String) -> impl IntoView {
  let table = Rc::new(table);
  let schema = Rc::new(schema);
  let pgsql = expect_context::<Pgsql>();
  let query = create_action(move |(schema, table, pgsql): &(String, String, Pgsql)| {
    let pgsql = *pgsql;
    let schema = schema.clone();
    let table = table.clone();

    async move {
      pgsql
        .run_default_table_query(&format!("SELECT * FROM {}.{} LIMIT 100;", schema, table))
        .await;
    }
  });

  view! {
      <div
          class="flex flex-row justify-between items-center hover:font-semibold cursor-pointer"
          on:click={
              let table = table.clone();
              move |_| { query.dispatch((schema.clone().to_string(), table.0.clone(), pgsql)) }
          }
      >

          <div class="flex flex-row items-center gap-1">
              <Icon icon=icondata::HiTableCellsOutlineLg width="12" height="12"/>
              <p>{table.0.to_string()}</p>
          </div>
          <p>{table.1.to_string()}</p>
      </div>
  }
}

