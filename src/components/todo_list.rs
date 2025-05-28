use crate::components::todo_item::TodoItem;
use crate::models::Todo;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TodoListProps {
    pub todos: Vec<Todo>,
    pub on_toggle: Callback<uuid::Uuid>,
    pub on_delete: Callback<uuid::Uuid>,
}

#[function_component(TodoList)]
pub fn todo_list(props: &TodoListProps) -> Html {
    if props.todos.is_empty() {
        html! {
            <div class="empty-state">
                <p>{"No tasks yet. Add one to get started!"}</p>
            </div>
        }
    } else {
        html! {
            <ul class="todo-list">
                {props.todos.iter().map(|todo| {
                    html! {
                        <TodoItem
                            key={todo.id.to_string()}
                            todo={todo.clone()}
                            on_toggle={props.on_toggle.clone()}
                            on_delete={props.on_delete.clone()}
                        />
                    }
                }).collect::<Html>()}
            </ul>
        }
    }
}
