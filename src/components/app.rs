use crate::components::{
    filter_tabs::FilterTabs, header::Header, todo_input::TodoInput, todo_list::TodoList,
};
use crate::models::{Filter, Todo};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let todos = use_state(|| Vec::<Todo>::new());
    let filter = use_state(|| Filter::All);
    let show_input = use_state(|| false);

    // Load todos from localStorage on mount
    {
        let todos = todos.clone();
        use_effect_with((), move |_| {
            if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
                if let Ok(Some(stored)) = storage.get_item("todos") {
                    if let Ok(loaded_todos) = serde_json::from_str::<Vec<Todo>>(&stored) {
                        todos.set(loaded_todos);
                    }
                }
            }
            || ()
        });
    }

    // Save todos to localStorage whenever they change
    {
        let todos_dep = todos.clone();
        use_effect_with(todos_dep, move |todos| {
            if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
                let _ = storage.set_item("todos", &serde_json::to_string(&**todos).unwrap());
            }
            || ()
        });
    }

    let add_todo = {
        let todos = todos.clone();
        let show_input = show_input.clone();
        Callback::from(move |text: String| {
            if !text.trim().is_empty() {
                let mut new_todos = (*todos).clone();
                new_todos.push(Todo::new(text));
                todos.set(new_todos);
                show_input.set(false);
            }
        })
    };

    let toggle_todo = {
        let todos = todos.clone();
        Callback::from(move |id: uuid::Uuid| {
            let mut new_todos = (*todos).clone();
            if let Some(todo) = new_todos.iter_mut().find(|t| t.id == id) {
                todo.completed = !todo.completed;
            }
            todos.set(new_todos);
        })
    };

    let delete_todo = {
        let todos = todos.clone();
        Callback::from(move |id: uuid::Uuid| {
            let new_todos: Vec<Todo> = (*todos).iter().filter(|t| t.id != id).cloned().collect();
            todos.set(new_todos);
        })
    };

    let show_add_input = {
        let show_input = show_input.clone();
        Callback::from(move |_| {
            show_input.set(true);
        })
    };

    let hide_input = {
        let show_input = show_input.clone();
        Callback::from(move |_| {
            show_input.set(false);
        })
    };

    let change_filter = {
        let filter = filter.clone();
        Callback::from(move |new_filter: Filter| {
            filter.set(new_filter);
        })
    };

    let filtered_todos: Vec<Todo> = (*todos)
        .iter()
        .filter(|todo| match *filter {
            Filter::All => true,
            Filter::Active => !todo.completed,
            Filter::Completed => todo.completed,
        })
        .cloned()
        .collect();

    let counts = (
        (*todos).len(),
        (*todos).iter().filter(|t| !t.completed).count(),
        (*todos).iter().filter(|t| t.completed).count(),
    );

    html! {
        <div class="app">
            <Header on_add_click={show_add_input} />

            {if *show_input {
                html! {
                    <TodoInput
                        on_submit={add_todo}
                        on_cancel={hide_input}
                    />
                }
            } else {
                html! {}
            }}

            <main class="main-content">
                <TodoList
                    todos={filtered_todos}
                    on_toggle={toggle_todo}
                    on_delete={delete_todo}
                />

                {if !(*todos).is_empty() {
                    html! {
                        <FilterTabs
                            current_filter={*filter}
                            on_change={change_filter}
                            counts={counts}
                        />
                    }
                } else {
                    html! {}
                }}
            </main>
        </div>
    }
}
