use std::rc::Rc;

use chrono::{DateTime, Duration, Utc};
use yew::{
    function_component, html, platform::spawn_local, use_effect_with, use_state, Html, Properties,
    UseStateHandle,
};
use yew_chart::{
    axis::{Axis, Orientation, Scale},
    linear_axis_scale::LinearScale,
    series::{self, Labeller, Series, Tooltipper, Type},
    time_axis_scale::TimeScale,
};

use crate::api::{templog_dto::Measurement, templog_measurements::get_measurements};

const WIDTH: f32 = 400.0;
const HEIGHT: f32 = 200.0;
const MARGIN: f32 = 50.0;
const TICK_LENGTH: f32 = 10.0;

const STEPS_TEMPERATURE: f32 = 2.0;
const MAX_TEMPERATURE: f32 = 34.0;
const MIN_TEMPERATURE: f32 = 8.0;

const STEPS_HUMIDITY: f32 = 20.0;
const MAX_HUMIDITY: f32 = 100.0;
const MIN_HUMIDITY: f32 = 0.0;

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
    let temperature_scale: UseStateHandle<Rc<dyn Scale<Scalar = f32>>> = use_state(|| {
        Rc::new(LinearScale::new(
            MIN_TEMPERATURE..MAX_TEMPERATURE,
            STEPS_TEMPERATURE,
        )) as Rc<dyn Scale<Scalar = f32>>
    });
    let humidity_scale: UseStateHandle<Rc<dyn Scale<Scalar = f32>>> = use_state(|| {
        Rc::new(LinearScale::new(MIN_HUMIDITY..MAX_HUMIDITY, STEPS_HUMIDITY))
            as Rc<dyn Scale<Scalar = f32>>
    });

    {
        let humidity_data_set = humidity_data_set.clone();
        let humidity_scale = humidity_scale.clone();
        let temperature_data_set = temperature_data_set.clone();
        let temperature_scale = temperature_scale.clone();
        use_effect_with((), move |_| {
            let humidity_data_set = humidity_data_set.clone();
            let humidity_scale = humidity_scale.clone();
            let temperature_data_set = temperature_data_set.clone();
            let temperature_scale = temperature_scale.clone();
            spawn_local(async move {
                let result = get_measurements(start_date, end_date).await;
                match result {
                    Ok(data) => update_data(
                        data,
                        humidity_data_set,
                        humidity_scale,
                        temperature_data_set,
                        temperature_scale,
                    ),
                    Err(error) => {
                        log::error!("Request failed: {}.", error);
                    }
                }
            });
        });
    }

    let time_scale: Rc<dyn Scale<Scalar = _>> =
        Rc::new(TimeScale::new(timespan, Duration::days(1)));

    let tooltip: Rc<dyn Tooltipper<_, _>> = Rc::from(series::y_tooltip());

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

fn update_data(
    measurements: Vec<Measurement>,
    humidity_data_set: UseStateHandle<Rc<Vec<GraphEntry>>>,
    humidity_scale: UseStateHandle<Rc<dyn Scale<Scalar = f32>>>,
    temperature_data_set: UseStateHandle<Rc<Vec<GraphEntry>>>,
    temperature_scale: UseStateHandle<Rc<dyn Scale<Scalar = f32>>>,
) {
    // Update humidity data
    {
        let mut low = None;
        let mut high = None;
        let data: Vec<GraphEntry> = measurements
            .iter()
            .map(|measurement| {
                let time = &measurement.timestamp;
                let humidity = measurement.humidity as f32;

                // keep track of current max and min humidity
                if let Some(current_low) = low {
                    if humidity < current_low {
                        low = Some(humidity);
                    }
                } else {
                    low = Some(humidity);
                }

                if let Some(current_high) = high {
                    if humidity > current_high {
                        high = Some(humidity);
                    }
                } else {
                    high = Some(humidity);
                }

                (time.timestamp_millis(), humidity, None)
            })
            .collect();
        humidity_data_set.set(Rc::new(data));

        let scale_min = if let Some(found_low) = low {
            found_low - STEPS_HUMIDITY
        } else {
            MIN_HUMIDITY
        };
        let scale_high = if let Some(found_high) = high {
            found_high + STEPS_HUMIDITY
        } else {
            MAX_HUMIDITY
        };

        humidity_scale.set(Rc::new(LinearScale::new(
            scale_min..scale_high,
            STEPS_HUMIDITY,
        )));
    }

    // Update temperature data
    {
        let mut low = None;
        let mut high = None;
        let temperature_data: Vec<GraphEntry> = measurements
            .iter()
            .map(|measurement| {
                let time = measurement.timestamp;
                let temperature = measurement.temperature as f32;

                // keep track of current max and min temperature
                if let Some(current_low) = low {
                    if temperature < current_low {
                        low = Some(temperature);
                    }
                } else {
                    low = Some(temperature);
                }
                if let Some(current_high) = high {
                    if temperature > current_high {
                        high = Some(temperature);
                    }
                } else {
                    high = Some(temperature);
                }

                (time.timestamp_millis(), temperature, None)
            })
            .collect();
        temperature_data_set.set(Rc::new(temperature_data));

        let scale_min = if let Some(found_low) = low {
            found_low - STEPS_TEMPERATURE
        } else {
            MIN_TEMPERATURE
        };
        let scale_high = if let Some(found_high) = high {
            found_high + STEPS_TEMPERATURE
        } else {
            MAX_TEMPERATURE
        };

        temperature_scale.set(Rc::new(LinearScale::new(
            scale_min..scale_high,
            STEPS_TEMPERATURE,
        )));
    }
}
