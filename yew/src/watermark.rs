use crate::{
    chart::use_chart,
    data::{ImageWatermarkOptions, TextWatermarkOptions},
    mount::{marker, use_mount_queue, use_panel, Alive},
};

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextWatermarkProps {
    pub options: TextWatermarkOptions,
}

/// Text watermark drawn in the enclosing `ChartPanel`, or in the main pane outside of a panel.
///
/// The watermark is updated when its options change, and removed when the component is dropped.
#[component]
pub fn TextWatermark(props: &TextWatermarkProps) -> Html {
    let chart = use_chart();
    let queue = use_mount_queue();
    let panel = use_panel();
    let node = use_node_ref();
    let id = use_mut_ref(|| None::<String>);
    let options = use_mut_ref(|| props.options.clone());

    *options.borrow_mut() = props.options.clone();

    use_effect_with((), {
        let chart = chart.clone();
        let id = id.clone();
        let node = node.clone();

        move |_| {
            let alive = Alive::new();

            queue.push(&node, {
                let alive = alive.clone();
                let chart = chart.clone();
                let id = id.clone();

                move || {
                    if !alive.get() {
                        return;
                    }

                    let panel = panel.and_then(|panel| panel.get()).unwrap_or(0);

                    match chart.add_text_watermark(panel, &options.borrow()) {
                        Ok(added) => *id.borrow_mut() = Some(added),
                        Err(err) => err.with_prefix("Failed to add the text watermark").log(),
                    }
                }
            });

            move || {
                alive.drop_component();
                if let Some(id) = id.borrow_mut().take() {
                    if let Err(err) = chart.remove_watermark(&id) {
                        err.with_prefix("Failed to remove the text watermark").log();
                    }
                }
            }
        }
    });

    use_effect_with(props.options.clone(), move |options| {
        if let Some(id) = id.borrow().as_ref() {
            if let Err(err) = chart.update_text_watermark(id, options) {
                err.with_prefix("Failed to update the text watermark").log();
            }
        }
    });

    marker(&node)
}

#[derive(Properties, PartialEq)]
pub struct ImageWatermarkProps {
    /// URL of the image, which can be a data URL.
    pub url:     AttrValue,
    #[prop_or_default]
    pub options: ImageWatermarkOptions,
}

/// Image watermark drawn in the enclosing `ChartPanel`, or in the main pane outside of a panel.
///
/// The watermark is updated when its URL or options change, and removed when the component is dropped.
#[component]
pub fn ImageWatermark(props: &ImageWatermarkProps) -> Html {
    let chart = use_chart();
    let queue = use_mount_queue();
    let panel = use_panel();
    let node = use_node_ref();
    // URL and identifier of the watermark, once it is added.
    let current = use_mut_ref(|| None::<(AttrValue, String)>);
    let latest = use_mut_ref(|| (props.url.clone(), props.options.clone()));
    let panel_index = move || panel.as_ref().and_then(|panel| panel.get()).unwrap_or(0);

    *latest.borrow_mut() = (props.url.clone(), props.options.clone());

    use_effect_with((), {
        let chart = chart.clone();
        let current = current.clone();
        let node = node.clone();
        let panel_index = panel_index.clone();

        move |_| {
            let alive = Alive::new();

            queue.push(&node, {
                let alive = alive.clone();
                let chart = chart.clone();
                let current = current.clone();

                move || {
                    if !alive.get() {
                        return;
                    }

                    let (url, options) = latest.borrow().clone();

                    match chart.add_image_watermark(panel_index(), &url, &options) {
                        Ok(id) => *current.borrow_mut() = Some((url, id)),
                        Err(err) => err.with_prefix("Failed to add the image watermark").log(),
                    }
                }
            });

            move || {
                alive.drop_component();
                if let Some((_, id)) = current.borrow_mut().take() {
                    if let Err(err) = chart.remove_watermark(&id) {
                        err.with_prefix("Failed to remove the image watermark").log();
                    }
                }
            }
        }
    });

    // Before the watermark is added, there is nothing to update: it is added with the latest URL and options.
    use_effect_with((props.url.clone(), props.options.clone()), move |(url, options)| {
        let Some((current_url, id)) = current.borrow_mut().take() else {
            return;
        };
        let res = if current_url == *url {
            let res = chart.update_image_watermark(&id, options);

            current.replace(Some((current_url, id)));
            res
        } else {
            if let Err(err) = chart.remove_watermark(&id) {
                err.with_prefix("Failed to remove the image watermark").log();
            }

            chart.add_image_watermark(panel_index(), url, options).map(|id| {
                current.replace(Some((url.clone(), id)));
            })
        };

        if let Err(err) = res {
            err.with_prefix("Failed to update the image watermark").log();
        }
    });

    marker(&node)
}
