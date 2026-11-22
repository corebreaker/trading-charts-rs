use crate::{
    chart::use_chart,
    data::{series::Series, Marker, PriceLineOptions, SeriesMarkersOptions},
    mount::{use_mount_queue, use_panel, Alive},
    SeriesHandle,
};

use serde::Serialize;
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;

/// Shared handle of a series, set once the series is added to the chart.
pub(crate) type SeriesCell<Dat, Opt> = Rc<RefCell<Option<SeriesHandle<Dat, Opt>>>>;

/// Latest values given to a series.
struct SeriesValues<Dat, Opt> {
    options:         Option<Opt>,
    data:            Vec<Dat>,
    markers:         Vec<Marker>,
    markers_options: Option<SeriesMarkersOptions>,
    price_lines:     Vec<PriceLineOptions>,
}

/// Add a series with the given values, after its pane is allocated.
fn add_series<Dat, Opt>(
    chart: &crate::ChartHandle,
    series_type: &'static str,
    panel: Option<crate::PanelId>,
    values: &SeriesValues<Dat, Opt>,
) -> Option<SeriesHandle<Dat, Opt>>
where
    Dat: Serialize + Clone,
    Opt: Serialize + Clone, {
    let mut series: Series<Dat, Opt> = Series::new(series_type);
    if let Some(panel) = panel {
        series.set_panel(panel);
    }
    if let Some(options) = &values.options {
        series.set_options(options.clone());
    }

    let handle = match chart.add_series(&series) {
        Ok(handle) => handle,
        Err(err) => {
            err.with_prefix("Failed to add series").log();
            return None;
        }
    };

    if let Err(err) = handle.set_data(&values.data) {
        err.with_prefix("Failed to set data")
            .with_serializable_data(&values.data)
            .log();
    }
    if !values.markers.is_empty() {
        if let Err(err) = handle.set_markers(&values.markers) {
            err.with_prefix("Failed to set markers")
                .with_serializable_data(&values.markers)
                .log();
        }
    }
    if let Some(options) = &values.markers_options {
        if let Err(err) = handle.apply_markers_options(options) {
            err.with_prefix("Failed to apply markers options").log();
        }
    }
    if !values.price_lines.is_empty() {
        if let Err(err) = handle.set_price_lines(&values.price_lines) {
            err.with_prefix("Failed to set price lines")
                .with_serializable_data(&values.price_lines)
                .log();
        }
    }

    Some(handle)
}

/// Add a series to the chart of the context, in the pane of the context, and keep it in sync with the given values.
///
/// The series is added in document order, placed by the `marker` element rendered by the component, after the
/// effects of the render. It is updated when a value changes, and removed from the chart when the component is
/// dropped. The returned cell holds the series handle once the series is added.
#[hook]
pub(crate) fn use_series<Dat, Opt>(
    marker: &NodeRef,
    series_type: &'static str,
    options: Option<Opt>,
    data: Vec<Dat>,
    markers: Vec<Marker>,
    markers_options: Option<SeriesMarkersOptions>,
    price_lines: Vec<PriceLineOptions>,
) -> SeriesCell<Dat, Opt>
where
    Dat: Serialize + Clone + PartialEq + 'static,
    Opt: Serialize + Clone + PartialEq + 'static, {
    let chart = use_chart();
    let queue = use_mount_queue();
    let panel = use_panel();
    let handle: SeriesCell<Dat, Opt> = use_mut_ref(|| None);
    let values = use_mut_ref(|| None::<SeriesValues<Dat, Opt>>);

    *values.borrow_mut() = Some(SeriesValues {
        options:         options.clone(),
        data:            data.clone(),
        markers:         markers.clone(),
        markers_options: markers_options.clone(),
        price_lines:     price_lines.clone(),
    });

    use_effect_with((), {
        let handle = handle.clone();
        let marker = marker.clone();

        move |_| {
            let alive = Alive::new();

            queue.push(&marker, {
                let alive = alive.clone();
                let handle = handle.clone();

                move || {
                    if alive.get() {
                        if let Some(values) = values.borrow().as_ref() {
                            let panel = panel.and_then(|panel| panel.get());

                            *handle.borrow_mut() = add_series(&chart, series_type, panel, values);
                        }
                    }
                }
            });

            move || {
                alive.drop_component();
                if let Some(handle) = handle.borrow_mut().take() {
                    if let Err(err) = handle.remove() {
                        err.with_prefix("Failed to remove series").log();
                    }
                }
            }
        }
    });

    // The handle is not set yet on the first render: the series is added with the latest values.
    use_effect_with(options, {
        let handle = handle.clone();

        move |options| {
            if let (Some(handle), Some(options)) = (handle.borrow().as_ref(), options) {
                if let Err(err) = handle.apply_options(options) {
                    err.with_prefix("Failed to update series options")
                        .with_serializable_data(options)
                        .log();
                }
            }
        }
    });

    use_effect_with(data, {
        let handle = handle.clone();

        move |data| {
            if let Some(handle) = handle.borrow().as_ref() {
                if let Err(err) = handle.set_data(data) {
                    err.with_prefix("Failed to update data")
                        .with_serializable_data(data)
                        .log();
                }
            }
        }
    });

    use_effect_with(markers, {
        let handle = handle.clone();

        move |markers| {
            if let Some(handle) = handle.borrow().as_ref() {
                if let Err(err) = handle.set_markers(markers) {
                    err.with_prefix("Failed to set markers")
                        .with_serializable_data(markers)
                        .log();
                }
            }
        }
    });

    use_effect_with(markers_options, {
        let handle = handle.clone();

        move |markers_options| {
            if let (Some(handle), Some(options)) = (handle.borrow().as_ref(), markers_options) {
                if let Err(err) = handle.apply_markers_options(options) {
                    err.with_prefix("Failed to apply markers options").log();
                }
            }
        }
    });

    use_effect_with(price_lines, {
        let handle = handle.clone();

        move |price_lines| {
            if let Some(handle) = handle.borrow().as_ref() {
                if let Err(err) = handle.set_price_lines(price_lines) {
                    err.with_prefix("Failed to set price lines")
                        .with_serializable_data(price_lines)
                        .log();
                }
            }
        }
    });

    handle
}
