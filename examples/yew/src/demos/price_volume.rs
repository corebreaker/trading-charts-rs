use super::{base_options, keep_chart, with_chart, ChartRef, Choice};
use crate::data::daily_ohlcv;

use charts::{
    chart::Chart,
    data::{
        resample::{resample_ohlcv, Interval},
        PriceSeriesStyle,
        RangePreset,
    },
    series::price_volume::PriceVolumeSeries,
    series_data::{
        areas::AreaSeriesOptions,
        bars::BarSeriesOptions,
        candlesticks::CandlestickOptions,
        lines::LineSeriesOptions,
    },
};
use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Style {
    Candles,
    Bars,
    Line,
    Area,
}

impl Style {
    fn series_style(self) -> PriceSeriesStyle {
        match self {
            Style::Candles => PriceSeriesStyle::Candlestick(CandlestickOptions::new().with_title("ACME")),
            Style::Bars => PriceSeriesStyle::Bar(BarSeriesOptions::new().with_title("ACME")),
            Style::Line => PriceSeriesStyle::Line(LineSeriesOptions::new().with_title("ACME").with_color("#2563eb")),
            Style::Area => PriceSeriesStyle::Area(
                AreaSeriesOptions::new()
                    .with_title("ACME")
                    .with_line_color("#2563eb")
                    .with_top_color("rgba(37, 99, 235, 0.35)")
                    .with_bottom_color("rgba(37, 99, 235, 0.02)"),
            ),
        }
    }
}

const RANGES: [(RangePreset, &str); 7] = [
    (RangePreset::Months(1), "1M"),
    (RangePreset::Months(3), "3M"),
    (RangePreset::Months(6), "6M"),
    (RangePreset::YearToDate, "YTD"),
    (RangePreset::Years(1), "1Y"),
    (RangePreset::Bars(50), "50 bars"),
    (RangePreset::All, "All"),
];

#[component]
pub fn PriceVolumeDemo() -> Html {
    let daily = use_memo((), |_| daily_ohlcv(42, 730, 120.0));
    let style = use_state(|| Style::Candles);
    let interval = use_state(|| Interval::Days(1));
    let volume_height = use_state(|| 0.25);
    let chart: ChartRef = use_mut_ref(|| None);
    let data = use_memo(*interval, move |interval| match interval {
        Interval::Days(1) => daily.to_vec(),
        interval => resample_ohlcv(&daily, *interval),
    });

    // Show all the data when the interval changes.
    use_effect_with(*interval, {
        let chart = chart.clone();

        move |_| with_chart(&chart, |chart| chart.show_range(RangePreset::All))
    });

    let ranges = RANGES.into_iter().map(|(preset, text)| {
        let onclick = {
            let chart = chart.clone();

            move |_| with_chart(&chart, |chart| chart.show_range(preset))
        };

        html! { <button key={text} {onclick}>{text}</button> }
    });

    html! {
        <>
            <Choice<Style>
                label="Price style"
                value={style.clone()}
                options={vec![
                    (Style::Candles, "Candles"),
                    (Style::Bars, "Bars"),
                    (Style::Line, "Line"),
                    (Style::Area, "Area"),
                ]}
            />
            <Choice<Interval>
                label="Interval"
                value={interval}
                options={vec![(Interval::Days(1), "1D"), (Interval::Weeks(1), "1W"), (Interval::Months(1), "1M")]}
            />
            <Choice<f64>
                label="Volume height"
                value={volume_height.clone()}
                options={vec![(0.15, "15%"), (0.25, "25%"), (0.4, "40%")]}
            />
            <div class="toolbar">
                <span class="label">{"Range"}</span>
                {for ranges}
            </div>
            <Chart options={base_options()} class="chart" on_ready={keep_chart(&chart)}>
                // The type of the price series is chosen at mount: the key mounts the component again for another
                // style.
                <PriceVolumeSeries
                    key={format!("{:?}", *style)}
                    data={(*data).clone()}
                    style={style.series_style()}
                    volume_height={*volume_height}
                />
            </Chart>
        </>
    }
}
