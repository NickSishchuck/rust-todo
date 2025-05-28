use crate::models::Todo;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TodoItemProps {
    pub todo: Todo,
    pub on_toggle: Callback<uuid::Uuid>,
    pub on_delete: Callback<uuid::Uuid>,
}

#[function_component(TodoItem)]
pub fn todo_item(props: &TodoItemProps) -> Html {
    let on_toggle = {
        let id = props.todo.id;
        let callback = props.on_toggle.clone();
        Callback::from(move |_| callback.emit(id))
    };

    let on_delete = {
        let id = props.todo.id;
        let callback = props.on_delete.clone();
        Callback::from(move |_| callback.emit(id))
    };

    html! {
        <li class={classes!("todo-item", props.todo.completed.then(|| "completed"))}>
            <label class="todo-checkbox-container">
                <input
                    type="checkbox"
                    checked={props.todo.completed}
                    onchange={on_toggle}
                    class="todo-checkbox"
                />
                <span class="todo-checkbox-custom"></span>
            </label>
            <span class="todo-text">{&props.todo.text}</span>
            <button
                class="todo-delete"
                onclick={on_delete}
                aria-label="Delete todo"
            >
                {"×"}
            </button>
        </li>
    }
}
