use std::rc::Rc;

use chrono::{DateTime, Duration, Utc};
use yew::{
    function_component, html, platform::spawn_local, use_effect, use_state, Html, Properties,
    UseStateHandle,
};
use yew_chart::{
    axis::{Axis, Orientation, Scale},
    linear_axis_scale::LinearScale,
    series::{self, Labeller, Series, Tooltipper, Type},
    time_axis_scale::TimeScale,
};

use crate::api::templog_measurements::get_measurements;

const WIDTH: f32 = 400.0;
const HEIGHT: f32 = 200.0;
const MARGIN: f32 = 50.0;
const TICK_LENGTH: f32 = 10.0;

#[derive(Properties, PartialEq)]
pub struct GraphProps {
    pub from_date: DateTime<Utc>,
    pub to_date: DateTime<Utc>,
}

type GraphEntry = (i64, f32, Option<Rc<dyn Labeller>>);

#[function_component(Graph)]
pub fn graph(props: &GraphProps) -> Html {
    let start_date = props.to_date;
    let end_date = props.from_date;
    let timespan = start_date..end_date;
    let humidity_data_set: UseStateHandle<Rc<Vec<GraphEntry>>> = use_state(|| Rc::new(vec![]));
    let temperature_data_set: UseStateHandle<Rc<Vec<GraphEntry>>> = use_state(|| Rc::new(vec![]));

    {
        let humidity_data_set = humidity_data_set.clone();
        let temperature_data_set = temperature_data_set.clone();
        use_effect(move || {
            let humidity_data_set = humidity_data_set.clone();
            let temperature_data_set = temperature_data_set.clone();
            spawn_local(async move {
                let result = get_measurements(start_date, end_date).await;
                match result {
                    Ok(data) => {
                        let humidity_data: Vec<GraphEntry> = data
                            .iter()
                            .map(|measurement| {
                                let time = &measurement.timestamp;
                                let humidity = &measurement.humidity;
                                (time.timestamp_millis(), *humidity as f32, None)
                            })
                            .collect();
                        humidity_data_set.set(Rc::new(humidity_data));

                        let temperature_data: Vec<GraphEntry> = data
                            .iter()
                            .map(|measurement| {
                                let time = measurement.timestamp;
                                let temperature = measurement.temperature;
                                (time.timestamp_millis(), temperature as f32, None)
                            })
                            .collect();
                        temperature_data_set.set(Rc::new(temperature_data));
                    }
                    Err(error) => {
                        log::error!("Request failed: {}.", error);
                    }
                }
            });
        });
    }

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
                    data={Rc::clone(&temperature_data_set)}
                    horizontal_scale={Rc::clone(&time_scale)}
                    horizontal_scale_step={Duration::days(2).num_milliseconds()}
                    tooltipper={Rc::clone(&tooltip)}
                    vertical_scale={Rc::clone(&temperature_scale)}
                    x={MARGIN} y={MARGIN} width={WIDTH - (MARGIN * 2.0)} height={HEIGHT - (MARGIN * 2.0)}
                />

                <Series<i64, f32>
                    series_type={Type::Line}
                    name="humidity-graph"
                    data={Rc::clone(&humidity_data_set)}
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
