use crate::{
    data::{Marker, OhlcvData, PriceLineOptions, PriceSeriesStyle, PriceVolume, SeriesMarkersOptions, VolumeColors},
    mount::{marker, use_mount_queue},
    series::mount::{use_series, SeriesCell},
    series_data::histograms::HistogramSeriesOptions,
};

use serde::Serialize;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PriceVolumeSeriesProps {
    pub data:            Vec<OhlcvData>,
    #[prop_or_default]
    pub style:           PriceSeriesStyle,
    #[prop_or_default]
    pub volume_options:  Option<HistogramSeriesOptions>,
    #[prop_or_default]
    pub volume_colors:   VolumeColors,
    /// Ratio of the pane height taken by the volumes, from 0 to 1.
    #[prop_or(0.25)]
    pub volume_height:   f64,
    #[prop_or_default]
    pub markers:         Vec<Marker>,
    #[prop_or_default]
    pub markers_options: Option<SeriesMarkersOptions>,
    #[prop_or_default]
    pub price_lines:     Vec<PriceLineOptions>,
}

/// Apply the price scale margins of the prices and the volumes, once both series are added.
fn apply_scale_margins<Dat, Opt, VolDat, VolOpt>(
    price: &SeriesCell<Dat, Opt>,
    volume: &SeriesCell<VolDat, VolOpt>,
    volume_height: f64,
) where
    Dat: Serialize + Clone,
    Opt: Serialize + Clone,
    VolDat: Serialize + Clone,
    VolOpt: Serialize + Clone, {
    if let (Some(price), Some(volume)) = (price.borrow().as_ref(), volume.borrow().as_ref()) {
        let (price_scale, volume_scale) = PriceVolume::price_scale_options(volume_height);
        let res = price
            .apply_price_scale_options(&price_scale)
            .and_then(|_| volume.apply_price_scale_options(&volume_scale));

        if let Err(err) = res {
            err.with_prefix("Failed to apply the price and volume scale margins")
                .log();
        }
    }
}

/// Add prices and volumes to the enclosing `Chart`, in the enclosing `ChartPanel` if any.
///
/// The prices are drawn with the `style` (candlesticks by default), and the volumes as a histogram at the bottom of
/// the pane, on an overlay price scale, taking `volume_height` of the pane height. The volume columns are colored by
/// direction of the periods. The markers and price lines belong to the price series.
///
/// The type of the price series is chosen when the component is mounted: changing the style to another type has no
/// effect until the component is mounted again, for instance with another `key`.
#[component]
pub fn PriceVolumeSeries(props: &PriceVolumeSeriesProps) -> Html {
    let node = use_node_ref();
    let queue = use_mount_queue();
    let price_data = PriceVolume::price_data(&props.data, &props.style);
    let volume_data = PriceVolume::volume_data(&props.data, &props.volume_colors);
    let price = use_series(
        &node,
        props.style.series_type(),
        Some(props.style.clone()),
        price_data,
        props.markers.clone(),
        props.markers_options.clone(),
        props.price_lines.clone(),
    );
    let volume = use_series(
        &node,
        "histogram",
        Some(PriceVolume::volume_options(props.volume_options.clone())),
        volume_data,
        Vec::new(),
        None,
        Vec::new(),
    );
    let volume_height = use_mut_ref(|| props.volume_height);

    *volume_height.borrow_mut() = props.volume_height;

    // Queued after the mounts of both series, for the same marker: it runs once they are added.
    use_effect_with((), {
        let price = price.clone();
        let volume = volume.clone();
        let node = node.clone();

        move |_| {
            queue.push(&node, move || {
                apply_scale_margins(&price, &volume, *volume_height.borrow())
            });
        }
    });

    use_effect_with(props.volume_height, move |volume_height| {
        apply_scale_margins(&price, &volume, *volume_height)
    });

    marker(&node)
}
