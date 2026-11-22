use crate::{
    chart::use_crosshair,
    data::{TooltipMode, TooltipPlacement},
};

use yew::prelude::*;

const TOOLTIP_STYLE: &str = "display:block;position:absolute;z-index:3100;pointer-events:none;";
const OVERLAY_STYLE: &str = "position:absolute;z-index:3050;pointer-events:none;";

#[derive(Properties, PartialEq)]
pub struct TooltipProps {
    /// How the tooltip follows the crosshair.
    #[prop_or_default]
    pub mode:      TooltipMode,
    /// Series followed by a floating tooltip; the series under the cursor, or else the first series, by default.
    #[prop_or_default]
    pub series_id: Option<AttrValue>,
    /// Distance in pixels between the tooltip and the point it follows. Default: `12`.
    #[prop_or(12.0)]
    pub offset:    f64,
    #[prop_or_default]
    pub class:     Classes,
    #[prop_or_default]
    pub style:     Option<AttrValue>,
    #[prop_or_default]
    pub children:  Html,
}

/// Tooltip following the crosshair of the enclosing `Chart`, which renders its children.
///
/// The children read the crosshair state with [`use_crosshair`](crate::chart::use_crosshair) to show the data under
/// the cursor. The tooltip is hidden when the cursor is outside of the data. It does not capture the mouse events,
/// unless its style sets `pointer-events:auto`.
#[component]
pub fn Tooltip(props: &TooltipProps) -> Html {
    let crosshair = use_crosshair();
    let placement = crosshair
        .as_ref()
        .and_then(|params| TooltipPlacement::compute(props.mode, params, props.series_id.as_deref(), props.offset));
    let style = match placement {
        Some(placement) => {
            format!(
                "{TOOLTIP_STYLE}{};{}",
                placement.css(),
                props.style.as_deref().unwrap_or_default()
            )
        }
        None => String::from("display:none"),
    };

    html! {
        <div class={props.class.clone()} {style}>
            {props.children.clone()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ChartOverlayProps {
    #[prop_or_default]
    pub class:    Classes,
    #[prop_or_default]
    pub style:    Option<AttrValue>,
    #[prop_or_default]
    pub children: Html,
}

/// Element drawn over the enclosing `Chart`, positioned with its style (for instance `top:8px;left:8px`).
///
/// It is the building block of custom legends: its children can read the crosshair state with
/// [`use_crosshair`](crate::chart::use_crosshair). It does not capture the mouse events, unless its style sets
/// `pointer-events:auto`.
#[component]
pub fn ChartOverlay(props: &ChartOverlayProps) -> Html {
    let style = format!("{OVERLAY_STYLE}{}", props.style.as_deref().unwrap_or_default());

    html! {
        <div class={props.class.clone()} {style}>
            {props.children.clone()}
        </div>
    }
}
