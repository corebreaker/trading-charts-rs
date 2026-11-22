pub mod customization;
pub mod multi_charts;
pub mod panes;
pub mod price_volume;
pub mod scales;
pub mod series;
pub mod tooltips;

use charts::{
    data::options::{layout::LayoutOptions, ChartOptions},
    ChartHandle,
    JsError,
};
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;

/// Chart handle kept from the `on_ready` event of a chart.
pub type ChartRef = Rc<RefCell<Option<ChartHandle>>>;

/// Chart options shared by the demos: the chart follows the size of its container.
pub fn base_options() -> ChartOptions {
    ChartOptions::new()
        .with_auto_size(true)
        .with_layout(LayoutOptions::new().with_attribution_logo(false))
}

/// Callback keeping the chart handle given by the `on_ready` event of a chart.
pub fn keep_chart(chart: &ChartRef) -> Callback<ChartHandle> {
    let chart = chart.clone();

    Callback::from(move |handle| {
        chart.replace(Some(handle));
    })
}

/// Run an action on the chart handle kept from the `on_ready` event of a chart, logging its error.
pub fn with_chart(chart: &ChartRef, action: impl FnOnce(&ChartHandle) -> Result<(), JsError>) {
    if let Some(chart) = chart.borrow().as_ref() {
        if let Err(err) = action(chart) {
            err.log();
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ChoiceProps<T: Clone + PartialEq + 'static> {
    pub label:   &'static str,
    pub options: Vec<(T, &'static str)>,
    pub value:   UseStateHandle<T>,
}

/// Toolbar of buttons selecting one value among several.
#[component]
pub fn Choice<T: Clone + PartialEq + 'static>(props: &ChoiceProps<T>) -> Html {
    let buttons = props.options.iter().map(|(option, text)| {
        let active = *props.value == *option;
        let onclick = {
            let value = props.value.clone();
            let option = option.clone();

            move |_| value.set(option.clone())
        };

        html! {
            <button key={*text} class={classes!(active.then_some("active"))} {onclick}>{*text}</button>
        }
    });

    html! {
        <div class="toolbar">
            <span class="label">{props.label}</span>
            {for buttons}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ToggleProps {
    pub label: &'static str,
    pub value: UseStateHandle<bool>,
}

/// Checkbox bound to a boolean state.
#[component]
pub fn Toggle(props: &ToggleProps) -> Html {
    let onchange = {
        let value = props.value.clone();

        move |_| value.set(!*value)
    };

    html! {
        <label class="toggle">
            <input type="checkbox" checked={*props.value} {onchange} />
            {props.label}
        </label>
    }
}
