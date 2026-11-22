use crate::{
    chart::use_chart,
    data::PaneOptions,
    mount::{marker, use_mount_queue, Alive, PanelSlot},
};

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChartPanelProps {
    /// Settings of the pane (stretch factor, height, preservation when empty), applied again when they change.
    #[prop_or_default]
    pub options:  Option<PaneOptions>,
    #[prop_or_default]
    pub children: Html,
}

/// Pane of the enclosing `Chart`: the series and watermarks given as children are drawn in this pane.
///
/// The panes are allocated in document order: the first `ChartPanel` is the main pane (index 0), the next ones are
/// stacked below it. The series which are not in a `ChartPanel` are drawn in the main pane.
#[component]
pub fn ChartPanel(props: &ChartPanelProps) -> Html {
    let chart = use_chart();
    let queue = use_mount_queue();
    let slot = use_memo((), |_| PanelSlot::default());
    let options = use_mut_ref(|| None::<PaneOptions>);
    let node = use_node_ref();

    *options.borrow_mut() = props.options.clone();

    use_effect_with((), {
        let chart = chart.clone();
        let slot = slot.clone();
        let node = node.clone();

        move |_| {
            let alive = Alive::new();

            queue.push(&node, {
                let alive = alive.clone();

                move || {
                    if !alive.get() {
                        return;
                    }

                    let panel = chart.allocate_panel();

                    slot.set(panel);
                    if let Some(options) = options.borrow().as_ref() {
                        if let Err(err) = chart.configure_pane(panel, options) {
                            err.with_prefix("Failed to configure the pane").log();
                        }
                    }
                }
            });

            move || alive.drop_component()
        }
    });

    use_effect_with(props.options.clone(), {
        let slot = slot.clone();

        move |options| {
            if let (Some(panel), Some(options)) = (slot.get(), options) {
                if let Err(err) = chart.configure_pane(panel, options) {
                    err.with_prefix("Failed to configure the pane").log();
                }
            }
        }
    });

    html! {
        <ContextProvider<PanelSlot> context={(*slot).clone()}>
            {marker(&node)}
            {props.children.clone()}
        </ContextProvider<PanelSlot>>
    }
}
