use super::AreaSeriesOptions;
use crate::{
    data::{ValueData, Marker, PriceLineOptions, SeriesMarkersOptions},
    mount::marker,
    series::mount::use_series,
};

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AreaSeriesProps {
    pub data:            Vec<ValueData>,
    #[prop_or_default]
    pub markers:         Vec<Marker>,
    #[prop_or_default]
    pub markers_options: Option<SeriesMarkersOptions>,
    #[prop_or_default]
    pub price_lines:     Vec<PriceLineOptions>,
    #[prop_or_default]
    pub options:         Option<AreaSeriesOptions>,
}

/// Add an area series to the enclosing `Chart`, in the enclosing `ChartPanel` if any.
///
/// The series is updated when `options`, `data`, `markers`, `markers_options` or `price_lines` change,
/// and is removed from the chart when the component is dropped.
#[component]
pub fn AreaSeries(props: &AreaSeriesProps) -> Html {
    let node = use_node_ref();

    use_series(
        &node,
        "area",
        props.options.clone(),
        props.data.clone(),
        props.markers.clone(),
        props.markers_options.clone(),
        props.price_lines.clone(),
    );

    marker(&node)
}
