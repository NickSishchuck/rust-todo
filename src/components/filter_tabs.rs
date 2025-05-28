use crate::models::Filter;
use web_sys::MouseEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FilterTabsProps {
    pub current_filter: Filter,
    pub on_change: Callback<Filter>,
    pub counts: (usize, usize, usize), // (all, active, completed)
}

#[function_component(FilterTabs)]
pub fn filter_tabs(props: &FilterTabsProps) -> Html {
    let (all, active, completed) = props.counts;

    let on_all_click = {
        let on_change = props.on_change.clone();
        Callback::from(move |_: MouseEvent| on_change.emit(Filter::All))
    };

    let on_active_click = {
        let on_change = props.on_change.clone();
        Callback::from(move |_: MouseEvent| on_change.emit(Filter::Active))
    };

    let on_completed_click = {
        let on_change = props.on_change.clone();
        Callback::from(move |_: MouseEvent| on_change.emit(Filter::Completed))
    };

    html! {
        <div class="filter-tabs">
            <button
                class={classes!("filter-tab", (props.current_filter == Filter::All).then(|| "active"))}
                onclick={on_all_click}
            >
                {"All"}
                <span class="filter-count">{format!(" ({})", all)}</span>
            </button>
            <span class="filter-separator">{"·"}</span>
            <button
                class={classes!("filter-tab", (props.current_filter == Filter::Active).then(|| "active"))}
                onclick={on_active_click}
            >
                {"Active"}
                <span class="filter-count">{format!(" ({})", active)}</span>
            </button>
            <span class="filter-separator">{"·"}</span>
            <button
                class={classes!("filter-tab", (props.current_filter == Filter::Completed).then(|| "active"))}
                onclick={on_completed_click}
            >
                {"Completed"}
                <span class="filter-count">{format!(" ({})", completed)}</span>
            </button>
        </div>
    }
}
