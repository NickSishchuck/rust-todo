use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, KeyboardEvent, SubmitEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TodoInputProps {
    pub on_submit: Callback<String>,
    pub on_cancel: Callback<()>,
}

#[function_component(TodoInput)]
pub fn todo_input(props: &TodoInputProps) -> Html {
    let input_ref = use_node_ref();
    let value = use_state(String::new);

    // Focus input on mount
    {
        let input_ref = input_ref.clone();
        use_effect_with_deps(
            move |_| {
                if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    let _ = input.focus();
                }
                || ()
            },
            (),
        );
    }

    let on_change = {
        let value = value.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            value.set(input.value());
        })
    };

    let on_submit = {
        let value = value.clone();
        let on_submit = props.on_submit.clone();
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if !value.trim().is_empty() {
                on_submit.emit((*value).clone());
            } else {
                on_cancel.emit(());
            }
        })
    };

    let on_keydown = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Escape" {
                on_cancel.emit(());
            }
        })
    };

    let on_cancel_click = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| on_cancel.emit(()))
    };

    html! {
        <div class="todo-input-container">
            <form onsubmit={on_submit} class="todo-input-form">
                <input
                    ref={input_ref}
                    type="text"
                    class="todo-input"
                    placeholder="What needs to be done?"
                    value={(*value).clone()}
                    onchange={on_change}
                    onkeydown={on_keydown}
                />
                <div class="todo-input-actions">
                    <button type="submit" class="todo-input-submit">
                        {"Add"}
                    </button>
                    <button
                        type="button"
                        class="todo-input-cancel"
                        onclick={on_cancel_click}
                    >
                        {"Cancel"}
                    </button>
                </div>
            </form>
        </div>
    }
}
