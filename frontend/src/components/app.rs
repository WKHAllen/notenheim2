use crate::hooks::use_backend;
use commands::FrontendCommands;
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let backend = use_backend();

    let (number_x, set_number_x) = create_signal("0".to_owned());
    let (number_y, set_number_y) = create_signal("0".to_owned());
    let (result, set_result) = create_signal(None);

    let action = create_action(move |(x, y): &(String, String)| {
        let x = x.clone();
        let y = y.clone();
        async move {
            match (x.parse::<i32>(), y.parse::<i32>()) {
                (Ok(x), Ok(y)) => {
                    let res = backend.add(x, y).await;
                    set_result(Some(res.to_string()));
                }
                _ => set_result(Some("Not a valid i32".to_owned())),
            }
        }
    });

    view! {
        <input
            type="text"
            prop:value=number_x
            on:input=move |ev| set_number_x(event_target_value(&ev))
        />
        <input
            type="text"
            prop:value=number_y
            on:input=move |ev| set_number_y(event_target_value(&ev))
        />
        <button on:click=move |_| action.dispatch((number_x(), number_y()))>
            "Add"
        </button>
        <p>{result}</p>
    }
}
