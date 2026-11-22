use super::BarSeriesOptions;
use crate::{
    data::{Candlestick, Marker, PriceLineOptions, SeriesMarkersOptions},
    mount::marker,
    series::mount::use_series,
};

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BarSeriesProps {
    pub data:            Vec<Candlestick>,
    #[prop_or_default]
    pub markers:         Vec<Marker>,
    #[prop_or_default]
    pub markers_options: Option<SeriesMarkersOptions>,
    #[prop_or_default]
    pub price_lines:     Vec<PriceLineOptions>,
    #[prop_or_default]
    pub options:         Option<BarSeriesOptions>,
}

/// Add a bar (OHLC) series to the enclosing `Chart`, in the enclosing `ChartPanel` if any.
///
/// The series is updated when `options`, `data`, `markers`, `markers_options` or `price_lines` change,
/// and is removed from the chart when the component is dropped.
#[component]
pub fn BarSeries(props: &BarSeriesProps) -> Html {
    let node = use_node_ref();

    use_series(
        &node,
        "bar",
        props.options.clone(),
        props.data.clone(),
        props.markers.clone(),
        props.markers_options.clone(),
        props.price_lines.clone(),
    );

    marker(&node)
}
