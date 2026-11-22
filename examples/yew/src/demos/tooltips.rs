use super::{base_options, Choice};
use crate::data::{candlesticks, daily_ohlcv, moving_average};

use charts::{
    chart::{use_crosshair, Chart},
    data::{
        options::{crosshair::CrosshairMode, crosshair::CrosshairOptions},
        LegendLayout,
        LegendOptions,
        LegendPosition,
        MouseEventParams,
        TooltipMode,
    },
    overlay::{ChartOverlay, Tooltip},
    series::{
        candlesticks::{CandleStickSeries, CandlestickOptions},
        lines::{LineSeries, LineSeriesOptions},
    },
};
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Legend {
    Series,
    ThreeLines,
    Custom,
}

/// OHLC prices of the candle under the cursor, if any.
fn ohlc(params: &MouseEventParams) -> Option<Html> {
    let candle = params
        .series_data
        .iter()
        .find(|data| data.series_type == "candlestick")?;
    let item = &candle.data;
    let title = candle.title.clone().unwrap_or_default();
    let change = item.change_percent().unwrap_or_default();
    let color = if change >= 0.0 { "#16a34a" } else { "#dc2626" };
    let date = params.time?.into_datetime().format("%a %d %b %Y").to_string();
    let rows = [
        ("Open", item.open),
        ("High", item.high),
        ("Low", item.low),
        ("Close", item.close),
    ]
    .into_iter()
    .map(|(label, price)| {
        html! {
            <tr>
                <td>{label}</td>
                <td>{format!("{:.2}", price.unwrap_or_default())}</td>
            </tr>
        }
    });

    Some(html! {
        <>
            <div class="tooltip-title">{title}</div>
            <div class="tooltip-date">{date}</div>
            <table>{for rows}</table>
            <div style={format!("color:{color};font-weight:600")}>{format!("{change:+.2}%")}</div>
        </>
    })
}

/// Content of the tooltip: the OHLC prices of the candle under the cursor.
#[component]
fn OhlcTooltip() -> Html {
    use_crosshair().as_ref().and_then(ohlc).unwrap_or_default()
}

/// Custom legend: the values of every series at the crosshair position.
#[component]
fn CustomLegend() -> Html {
    let lines = use_crosshair()
        .map(|params| {
            params
                .series_data
                .iter()
                .map(|data| {
                    let title = data.title.clone().unwrap_or_else(|| data.series_type.clone());
                    let price = data.data.price().unwrap_or_default();

                    format!("{title}: {price:.2}")
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    html! {
        <div class="custom-legend">
            <strong>{"Custom legend"}</strong>
            {for lines.into_iter().map(|line| html! { <div>{line}</div> })}
        </div>
    }
}

#[component]
pub fn TooltipsDemo() -> Html {
    let data = use_memo((), |_| {
        let data = daily_ohlcv(5, 150, 80.0);

        (candlesticks(&data), moving_average(&data, 10))
    });
    let mode = use_state(|| TooltipMode::Floating);
    let legend_kind = use_state(|| Legend::Series);
    let legend = match *legend_kind {
        Legend::Series => Some(LegendOptions::new().with_text("Built-in legend")),
        Legend::ThreeLines => Some(
            LegendOptions::new()
                .with_text("ACME Corp.")
                .with_layout(LegendLayout::ThreeLines)
                .with_position(LegendPosition::TopLeft)
                .with_time_format("EEEE d MMMM yyyy"),
        ),
        Legend::Custom => None,
    };
    let options = base_options().with_crosshair(CrosshairOptions::new().with_mode(CrosshairMode::Normal));
    let ma_options = LineSeriesOptions::new()
        .with_title("MA 10")
        .with_color("#f97316")
        .with_price_line_visible(false);
    let custom_legend = (*legend_kind == Legend::Custom).then(|| {
        html! {
            <ChartOverlay style="top:8px;left:12px"><CustomLegend /></ChartOverlay>
        }
    });

    html! {
        <>
            <Choice<TooltipMode>
                label="Tooltip"
                value={mode.clone()}
                options={vec![
                    (TooltipMode::Tracking, "Tracking"),
                    (TooltipMode::Floating, "Floating"),
                    (TooltipMode::Magnifier, "Magnifier"),
                ]}
            />
            <Choice<Legend>
                label="Legend"
                value={legend_kind}
                options={vec![(Legend::Series, "Series"), (Legend::ThreeLines, "3 lines"), (Legend::Custom, "Custom")]}
            />
            <Chart {options} {legend} class="chart">
                <CandleStickSeries data={data.0.clone()} options={CandlestickOptions::new().with_title("ACME")} />
                <LineSeries data={data.1.clone()} options={ma_options} />
                <Tooltip mode={*mode} class="tooltip"><OhlcTooltip /></Tooltip>
                {custom_legend}
            </Chart>
        </>
    }
}
