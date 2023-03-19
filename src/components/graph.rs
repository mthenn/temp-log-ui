use std::{
    ops::{Add, Sub},
    rc::Rc,
};

use chrono::{Duration, Utc};
use yew::{function_component, html, Html};
use yew_chart::{
    axis::{Axis, Orientation, Scale},
    linear_axis_scale::LinearScale,
    series::{self, Series, Tooltipper, Type},
    time_axis_scale::TimeScale,
};

const WIDTH: f32 = 400.0;
const HEIGHT: f32 = 200.0;
const MARGIN: f32 = 50.0;
const TICK_LENGTH: f32 = 10.0;

#[function_component(Graph)]
pub fn graph() -> Html {
    let end_date = Utc::now();
    let start_date = end_date.sub(Duration::days(4));
    let timespan = start_date..end_date;

    let humidity_data_set = Rc::new(vec![
        (start_date.timestamp_millis(), 42.0, None),
        (
            start_date.add(Duration::days(1)).timestamp_millis(),
            65.0,
            None,
        ),
        (
            start_date.add(Duration::days(2)).timestamp_millis(),
            40.0,
            None,
        ),
        (
            start_date.add(Duration::days(3)).timestamp_millis(),
            80.0,
            None,
        ),
        (
            start_date.add(Duration::days(4)).timestamp_millis(),
            60.0,
            None,
        ),
    ]);

    let temperature_data_set = Rc::new(vec![
        (start_date.timestamp_millis(), 14.0, None),
        (
            start_date.add(Duration::days(1)).timestamp_millis(),
            12.0,
            None,
        ),
        (
            start_date.add(Duration::days(2)).timestamp_millis(),
            22.0,
            None,
        ),
        (
            start_date.add(Duration::days(3)).timestamp_millis(),
            15.0,
            None,
        ),
        (
            start_date.add(Duration::days(4)).timestamp_millis(),
            18.0,
            None,
        ),
    ]);

    let time_scale =
        Rc::new(TimeScale::new(timespan, Duration::days(1))) as Rc<dyn Scale<Scalar = _>>;
    let temperature_scale = Rc::new(LinearScale::new(8.0..30.0, 2.0)) as Rc<dyn Scale<Scalar = _>>;
    let humidity_scale = Rc::new(LinearScale::new(0.0..100.0, 20.0)) as Rc<dyn Scale<Scalar = _>>;

    let tooltip = Rc::from(series::y_tooltip()) as Rc<dyn Tooltipper<_, _>>;

    html! {
            <svg class="chart" viewBox={format!("0 0 {} {}", WIDTH, HEIGHT)}>

                <Series<i64, f32>
                    series_type={Type::Line}
                    name="temperature-graph"
                    data={temperature_data_set}
                    horizontal_scale={Rc::clone(&time_scale)}
                    horizontal_scale_step={Duration::days(2).num_milliseconds()}
                    tooltipper={Rc::clone(&tooltip)}
                    vertical_scale={Rc::clone(&temperature_scale)}
                    x={MARGIN} y={MARGIN} width={WIDTH - (MARGIN * 2.0)} height={HEIGHT - (MARGIN * 2.0)}
                />

                <Series<i64, f32>
                    series_type={Type::Line}
                    name="humidity-graph"
                    data={humidity_data_set}
                    horizontal_scale={Rc::clone(&time_scale)}
                    horizontal_scale_step={Duration::days(2).num_milliseconds()}
                    tooltipper={Rc::clone(&tooltip)}
                    vertical_scale={Rc::clone(&humidity_scale)}
                    x={MARGIN} y={MARGIN} width={WIDTH - (MARGIN * 2.0)} height={HEIGHT - (MARGIN * 2.0)}
                 />

                <Axis<f32>
                    name="humidity-axis"
                    orientation={Orientation::Right}
                    scale={Rc::clone(&humidity_scale)}
                    x1={WIDTH - (MARGIN)} y1={MARGIN} xy2={HEIGHT - MARGIN}
                    tick_len={TICK_LENGTH}
                    title={"Humidity".to_string()} />

                <Axis<f32>
                    name="temperature-axis"
                    orientation={Orientation::Left}
                    scale={Rc::clone(&temperature_scale)}
                    x1={MARGIN} y1={MARGIN} xy2={HEIGHT - MARGIN}
                    tick_len={TICK_LENGTH}
                    title={"Temperature".to_string()} />

                <Axis<i64>
                    name="time-axis"
                    orientation={Orientation::Bottom}
                    scale={Rc::clone(&time_scale)}
                    x1={MARGIN} y1={HEIGHT - MARGIN} xy2={WIDTH - MARGIN}
                    tick_len={TICK_LENGTH}
                    title={"Time".to_string()} />

            </svg>
    }
}
