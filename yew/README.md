[![Crates.io]](https://crates.io/crates/trading-charts-yew)
[![Docs.rs]](https://docs.rs/trading-charts-yew/)

# Trading-Charts for Yew
[Yew] components for [Lightweight Charts] 5.3, the financial charts library of TradingView.

The crate targets the client-side rendering of Yew (`csr`): the charts need a browser DOM.

It is built on [`trading-charts-core`](https://crates.io/crates/trading-charts-core), which it re-exports:
`trading_charts_yew::data` holds the data and option types, `trading_charts_yew::series_data` the series options,
and `ChartHandle` drives a chart imperatively.

## Components

| Component           | Module                 | Role                                              |
|---------------------|------------------------|---------------------------------------------------|
| `Chart`             | `chart`                | The chart, which renders its children.            |
| `ChartPanel`        | `panel`                | A pane, with its series and watermarks.           |
| `CandleStickSeries` | `series::candlesticks` | OHLC data drawn as candlesticks.                  |
| `BarSeries`         | `series::bars`         | OHLC data drawn as bars.                          |
| `LineSeries`        | `series::lines`        | Single values drawn as a line.                    |
| `AreaSeries`        | `series::areas`        | Single values drawn as a filled area.             |
| `BaselineSeries`    | `series::baselines`    | Single values drawn above and below a base value. |
| `HistogramSeries`   | `series::histograms`   | Single values drawn as columns.                   |
| `PriceVolumeSeries` | `series::price_volume` | Prices and volumes on the same chart.             |
| `Tooltip`           | `overlay`              | A tooltip following the crosshair.                |
| `ChartOverlay`      | `overlay`              | An element over the chart, for custom legends.    |
| `TextWatermark`     | `watermark`            | A text watermark of a pane.                       |
| `ImageWatermark`    | `watermark`            | An image watermark of a pane.                     |

The series components update the chart when the value of a prop changes, and remove their series from the chart
when they are dropped.

## Example

```rust
use yew::prelude::*;
use trading_charts_yew::{
    chart::{use_crosshair, Chart},
    data::{options::ChartOptions, Candlestick, LegendOptions, TooltipMode},
    overlay::Tooltip,
    panel::ChartPanel,
    series::candlesticks::CandleStickSeries,
};

#[derive(Properties, PartialEq)]
struct PriceChartProps {
    data: Vec<Candlestick>,
}

#[component]
fn PriceChart(props: &PriceChartProps) -> Html {
    html! {
        <Chart
            options={ChartOptions::new().with_auto_size(true)}
            legend={LegendOptions::new().with_text("ACME")}
            time_zone="Europe/Paris"
            style="height:400px"
        >
            <ChartPanel>
                <CandleStickSeries data={props.data.clone()} />
            </ChartPanel>
            <Tooltip mode={TooltipMode::Floating}><Price /></Tooltip>
        </Chart>
    }
}

/// Tooltip content: the close price under the cursor.
#[component]
fn Price() -> Html {
    let price = use_crosshair()
        .as_ref()
        .and_then(|params| params.main_series())
        .and_then(|series| series.data.price())
        .map(|price| format!("{price:.2}"))
        .unwrap_or_default();

    html! { {price} }
}
```

## Chart

The chart fills a `div` sized by its `style` and `class` props, which is also the positioning context of the
overlays. Set the `autoSize` option for the chart to follow the size of the `div`.

Its props give the options, the built-in legend, the time zone, the synchronization group of the chart
(the charts of a group share their crosshair and visible range) and the event callbacks:
`on_crosshair_move`, `on_click`, `on_dbl_click`, `on_visible_time_range_change` and
`on_visible_logical_range_change`. The `on_ready` callback gives the `ChartHandle`, to control the chart from outside:
show a range, take a screenshot, move the crosshair...

The children of a chart get the chart handle with the `use_chart` hook and the crosshair state with
the `use_crosshair` hook, which renders the component again each time the crosshair moves.

## Panes

The first `ChartPanel` is the main pane, the next ones are stacked below it; the series outside of a panel are drawn
in the main pane. The `options` prop of a panel sets its stretch factor, height and preservation when empty.

[Crates.io]: https://img.shields.io/crates/v/trading-charts-yew?style=for-the-badge
[Docs.rs]: https://img.shields.io/docsrs/trading-charts-yew?style=for-the-badge
[Lightweight Charts]: https://github.com/tradingview/lightweight-charts
[Yew]: https://yew.rs/
