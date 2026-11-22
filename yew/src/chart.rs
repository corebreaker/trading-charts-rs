use crate::{
    data::{options::ChartOptions, LegendOptions, LogicalRange, MouseEventParams, SyncOptions, TimeRange},
    mount::MountQueue,
    ChartHandle,
    JsError,
    Subscription,
};

use web_sys::HtmlDivElement;
use yew::prelude::*;

/// Crosshair state of a chart, provided to the children of [`Chart`].
#[derive(Clone, PartialEq)]
struct CrosshairState(Option<MouseEventParams>);

/// Event callbacks of a chart, updated at each render so that the subscriptions call the latest ones.
#[derive(Clone, Default)]
struct ChartEventHandlers {
    crosshair_move:               Option<Callback<MouseEventParams>>,
    click:                        Option<Callback<MouseEventParams>>,
    dbl_click:                    Option<Callback<MouseEventParams>>,
    visible_time_range_change:    Option<Callback<Option<TimeRange>>>,
    visible_logical_range_change: Option<Callback<Option<LogicalRange>>>,
}

#[derive(Properties, PartialEq)]
pub struct ChartProps {
    /// Options of the chart, applied again when they change.
    #[prop_or_default]
    pub options:                         Option<ChartOptions>,
    /// Options of the built-in legend, which is removed with `None`.
    #[prop_or_default]
    pub legend:                          Option<LegendOptions>,
    /// IANA time zone in which the times are displayed, such as `Europe/Paris`; UTC by default.
    #[prop_or_default]
    pub time_zone:                       Option<AttrValue>,
    /// Synchronization group of the chart: the charts of a group share their crosshair and visible range.
    #[prop_or_default]
    pub sync_group:                      Option<AttrValue>,
    /// What is synchronized with the other charts of the synchronization group.
    #[prop_or_default]
    pub sync_options:                    SyncOptions,
    #[prop_or_default]
    pub style:                           Option<AttrValue>,
    #[prop_or_default]
    pub class:                           Classes,
    /// Called once with the chart handle, when the chart is created, to control the chart from outside.
    #[prop_or_default]
    pub on_ready:                        Option<Callback<ChartHandle>>,
    #[prop_or_default]
    pub on_crosshair_move:               Option<Callback<MouseEventParams>>,
    #[prop_or_default]
    pub on_click:                        Option<Callback<MouseEventParams>>,
    #[prop_or_default]
    pub on_dbl_click:                    Option<Callback<MouseEventParams>>,
    #[prop_or_default]
    pub on_visible_time_range_change:    Option<Callback<Option<TimeRange>>>,
    #[prop_or_default]
    pub on_visible_logical_range_change: Option<Callback<Option<LogicalRange>>>,
    #[prop_or_default]
    pub children:                        Html,
}

fn keep_subscription(subscription: Result<Subscription, JsError>, event: &str) -> Option<Subscription> {
    match subscription {
        Ok(subscription) => Some(subscription),
        Err(err) => {
            err.with_prefix(format!("Failed to subscribe to the {event} event"))
                .log();
            None
        }
    }
}

/// Subscribe to the events of the chart, calling the latest callbacks given by the props.
fn subscribe(
    chart: &ChartHandle,
    handlers: std::rc::Rc<std::cell::RefCell<ChartEventHandlers>>,
    crosshair: UseStateSetter<CrosshairState>,
) -> Vec<Option<Subscription>> {
    let handlers = move || handlers.borrow().clone();

    vec![
        keep_subscription(
            chart.on_crosshair_move({
                let handlers = handlers.clone();

                move |params: MouseEventParams| {
                    if let Some(handler) = handlers().crosshair_move {
                        handler.emit(params.clone());
                    }

                    crosshair.set(CrosshairState(Some(params)));
                }
            }),
            "crosshair move",
        ),
        keep_subscription(
            chart.on_click({
                let handlers = handlers.clone();

                move |params| {
                    if let Some(handler) = handlers().click {
                        handler.emit(params);
                    }
                }
            }),
            "click",
        ),
        keep_subscription(
            chart.on_dbl_click({
                let handlers = handlers.clone();

                move |params| {
                    if let Some(handler) = handlers().dbl_click {
                        handler.emit(params);
                    }
                }
            }),
            "double click",
        ),
        keep_subscription(
            chart.on_visible_time_range_change({
                let handlers = handlers.clone();

                move |range| {
                    if let Some(handler) = handlers().visible_time_range_change {
                        handler.emit(range);
                    }
                }
            }),
            "visible time range change",
        ),
        keep_subscription(
            chart.on_visible_logical_range_change(move |range| {
                if let Some(handler) = handlers().visible_logical_range_change {
                    handler.emit(range);
                }
            }),
            "visible logical range change",
        ),
    ]
}

/// Chart, which renders its series, panes, tooltips and overlays given as children.
///
/// The chart fills a `div` sized by the `style` and `class` props, which is also the positioning context of the
/// overlays: give it a size, for instance with `style="width:100%;height:400px"`, and set the `autoSize` chart
/// option to follow it.
///
/// The children can access the chart with [`use_chart`] and the crosshair state with [`use_crosshair`].
#[component]
pub fn Chart(props: &ChartProps) -> Html {
    let chart = use_memo((), {
        let options = props.options.clone();

        move |_| match ChartHandle::new(options.as_ref()) {
            Ok(chart) => Some(chart),
            Err(err) => {
                err.with_prefix("Failed to create chart").log();
                None
            }
        }
    });
    let mount_queue = use_memo((), |_| MountQueue::default());
    let crosshair = use_state(|| CrosshairState(None));
    let handlers = use_mut_ref(ChartEventHandlers::default);
    let node = use_node_ref();

    *handlers.borrow_mut() = ChartEventHandlers {
        crosshair_move:               props.on_crosshair_move.clone(),
        click:                        props.on_click.clone(),
        dbl_click:                    props.on_dbl_click.clone(),
        visible_time_range_change:    props.on_visible_time_range_change.clone(),
        visible_logical_range_change: props.on_visible_logical_range_change.clone(),
    };

    use_effect_with((), {
        let chart = chart.clone();
        let on_ready = props.on_ready.clone();
        let setter = crosshair.setter();
        let node = node.clone();

        move |_| {
            let subscriptions = chart.as_ref().as_ref().map(|chart| {
                if let Some(div) = node.cast::<HtmlDivElement>() {
                    if let Err(err) = chart.bind(div) {
                        err.with_prefix("Failed to bind chart").log();
                    }
                }

                if let Some(on_ready) = on_ready {
                    on_ready.emit(chart.clone());
                }

                subscribe(chart, handlers, setter)
            });

            // The subscriptions are cancelled when the component is dropped.
            move || drop(subscriptions)
        }
    });

    use_effect_with(props.options.clone(), {
        let chart = chart.clone();

        move |options| {
            if let (Some(chart), Some(options)) = (chart.as_ref(), options) {
                if let Err(err) = chart.apply_chart_options(options) {
                    err.with_prefix("Failed to apply chart options").log();
                }
            }
        }
    });

    use_effect_with(props.time_zone.clone(), {
        let chart = chart.clone();

        move |time_zone| {
            if let Some(chart) = chart.as_ref() {
                if let Err(err) = chart.set_time_zone(time_zone.as_deref()) {
                    err.with_prefix("Failed to set the time zone").log();
                }
            }
        }
    });

    use_effect_with((props.sync_group.clone(), props.sync_options), {
        let chart = chart.clone();

        move |(sync_group, sync_options)| {
            if let Some(chart) = chart.as_ref() {
                if let Err(err) = chart.set_sync_group(sync_group.as_deref(), sync_options) {
                    err.with_prefix("Failed to set the synchronization group").log();
                }
            }
        }
    });

    use_effect_with(props.legend.clone(), {
        let chart = chart.clone();

        move |legend| {
            if let Some(chart) = chart.as_ref() {
                let res = match legend {
                    Some(legend) => chart.apply_legend_options(legend),
                    None => chart.remove_legend(),
                };

                if let Err(err) = res {
                    err.with_prefix("Failed to update the legend").log();
                }
            }
        }
    });

    let Some(chart) = chart.as_ref().clone() else {
        return html! {};
    };
    let style = format!("position:relative;{}", props.style.as_deref().unwrap_or_default());

    html! {
        <div {style} class={props.class.clone()}>
            <div ref={node} style="position:absolute;inset:0"></div>
            <ContextProvider<ChartHandle> context={chart}>
                <ContextProvider<MountQueue> context={(*mount_queue).clone()}>
                    <ContextProvider<CrosshairState> context={(*crosshair).clone()}>
                        {props.children.clone()}
                    </ContextProvider<CrosshairState>>
                </ContextProvider<MountQueue>>
            </ContextProvider<ChartHandle>>
        </div>
    }
}

/// The chart handle of the enclosing [`Chart`].
///
/// # Panics
///
/// Panics when called outside of a `Chart`.
#[hook]
pub fn use_chart() -> ChartHandle {
    use_context::<ChartHandle>().expect("Chart context not found. Ensure the component is rendered inside <Chart>.")
}

/// State of the crosshair of the enclosing [`Chart`]: the component is rendered again each time the crosshair moves.
///
/// It is `None` before the first move, and always `None` outside of a `Chart`.
#[hook]
pub fn use_crosshair() -> Option<MouseEventParams> {
    use_context::<CrosshairState>().and_then(|state| state.0)
}
