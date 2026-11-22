use crate::demos::{
    customization::CustomizationDemo,
    multi_charts::MultiChartsDemo,
    panes::PanesDemo,
    price_volume::PriceVolumeDemo,
    scales::ScalesDemo,
    series::SeriesDemo,
    tooltips::TooltipsDemo,
};

use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Demo {
    Series,
    PriceVolume,
    Scales,
    Tooltips,
    Customization,
    Panes,
    MultiCharts,
}

impl Demo {
    const ALL: [Demo; 7] = [
        Demo::Series,
        Demo::PriceVolume,
        Demo::Scales,
        Demo::Tooltips,
        Demo::Customization,
        Demo::Panes,
        Demo::MultiCharts,
    ];

    fn title(self) -> &'static str {
        match self {
            Demo::Series => "Series",
            Demo::PriceVolume => "Price & volume",
            Demo::Scales => "Scales",
            Demo::Tooltips => "Tooltips & legends",
            Demo::Customization => "Customization",
            Demo::Panes => "Panes & watermarks",
            Demo::MultiCharts => "Multi-charts",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Demo::Series => {
                "The six series types, with markers (built-in, custom and price-positioned), price lines, whitespace \
                 data (the empty slots of the time scale) and a moving average. Changing the type mounts another \
                 series component."
            }
            Demo::PriceVolume => {
                "Prices and volumes on the same chart, with the price style, the bar interval (resampled data) and the \
                 visible range chosen with the buttons."
            }
            Demo::Scales => {
                "Two visible price scales, price scale modes, inverted scale and hidden time scale, applied as \
                 reactive chart options."
            }
            Demo::Tooltips => {
                "Tooltips rendered by Yew components, in the tracking, floating and magnifier modes, with the \
                 built-in legends and a custom legend overlay."
            }
            Demo::Customization => {
                "Time zone, locale, date format, price formatters (including a Rust closure), font and crosshair mode \
                 on real 5-minute data."
            }
            Demo::Panes => {
                "Several panes with stretch factors, text and image watermarks, pane swapping and screenshots."
            }
            Demo::MultiCharts => "Two charts in a synchronization group, sharing their crosshair and visible range.",
        }
    }
}

#[component]
pub fn App() -> Html {
    let demo = use_state(|| Demo::Series);
    let tabs = Demo::ALL.into_iter().map(|item| {
        let onclick = {
            let demo = demo.clone();

            move |_| demo.set(item)
        };

        html! {
            <button key={item.title()} class={classes!((*demo == item).then_some("active"))} {onclick}>
                {item.title()}
            </button>
        }
    });
    let content = match *demo {
        Demo::Series => html! { <SeriesDemo /> },
        Demo::PriceVolume => html! { <PriceVolumeDemo /> },
        Demo::Scales => html! { <ScalesDemo /> },
        Demo::Tooltips => html! { <TooltipsDemo /> },
        Demo::Customization => html! { <CustomizationDemo /> },
        Demo::Panes => html! { <PanesDemo /> },
        Demo::MultiCharts => html! { <MultiChartsDemo /> },
    };

    html! {
        <main>
            <h1>{"Trading Charts for Yew"}</h1>
            <nav>{for tabs}</nav>
            <p class="description">{demo.description()}</p>
            {content}
        </main>
    }
}
