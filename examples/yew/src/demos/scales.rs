use super::{base_options, Choice, Toggle};
use crate::data::{closes, daily_ohlcv};

use charts::{
    chart::Chart,
    data::{
        options::{PriceScaleMode, PriceScaleOptions, PriceScaleSide, TimeScaleOptions},
        ValueData,
    },
    series::lines::{LineSeries, LineSeriesOptions},
};
use yew::prelude::*;

#[component]
pub fn ScalesDemo() -> Html {
    let data = use_memo((), |_| {
        let acme: Vec<ValueData> = closes(&daily_ohlcv(3, 365, 100.0));
        let globex: Vec<ValueData> = closes(&daily_ohlcv(11, 365, 2_500.0));

        (acme, globex)
    });
    let mode = use_state(|| PriceScaleMode::Normal);
    let left_visible = use_state(|| true);
    let inverted = use_state(|| false);
    let time_visible = use_state(|| true);

    let options = base_options()
        .with_left_price_scale(
            PriceScaleOptions::new()
                .with_visible(*left_visible)
                .with_mode(*mode)
                .with_border_color("#2563eb"),
        )
        .with_right_price_scale(
            PriceScaleOptions::new()
                .with_mode(*mode)
                .with_invert_scale(*inverted)
                .with_border_color("#dc2626"),
        )
        .with_time_scale(TimeScaleOptions::new().with_visible(*time_visible));
    let acme_options = LineSeriesOptions::new()
        .with_title("ACME (left)")
        .with_color("#2563eb")
        .with_price_scale_id(PriceScaleSide::Left);
    let globex_options = LineSeriesOptions::new()
        .with_title("GLOBEX (right)")
        .with_color("#dc2626")
        .with_price_scale_id(PriceScaleSide::Right);

    html! {
        <>
            <Choice<PriceScaleMode>
                label="Price scale mode"
                value={mode}
                options={vec![
                    (PriceScaleMode::Normal, "Normal"),
                    (PriceScaleMode::Logarithmic, "Logarithmic"),
                    (PriceScaleMode::Percentage, "Percentage"),
                    (PriceScaleMode::IndexedTo100, "Indexed to 100"),
                ]}
            />
            <div class="toolbar">
                <Toggle label="Left price scale" value={left_visible} />
                <Toggle label="Invert the right price scale" value={inverted} />
                <Toggle label="Time scale" value={time_visible} />
            </div>
            <Chart {options} class="chart">
                <LineSeries data={data.0.clone()} options={acme_options} />
                <LineSeries data={data.1.clone()} options={globex_options} />
            </Chart>
        </>
    }
}
