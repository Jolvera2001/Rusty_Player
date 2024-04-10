use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherApiModel {
    lat: f64,
    lon: f64,
    timezone: String,
    timezone_offset: i64,
    current: Current,
    minutely: Vec<Minutely>,
    hourly: Vec<Hourly>,
    daily: Vec<Daily>,
    alerts: Option<Vec<Alert>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Current {
    dt: i64,
    sunrise: i64,
    sunset: i64,
    temp: f64,
    feels_like: f64,
    pressure: i64,
    humidity: i64,
    dew_point: f64,
    uvi: f64,
    clouds: i64,
    visibility: i64,
    wind_speed: f64,
    wind_deg: i64,
    wind_gust: f64,
    weather: Vec<Weather>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Minutely {
    dt: i64,
    precipitation: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Hourly {
    dt: i64,
    temp: f64,
    feels_like: f64,
    pressure: i64,
    humidity: i64,
    dew_point: f64,
    uvi: f64,
    clouds: i64,
    visibility: i64,
    wind_speed: f64,
    wind_deg: i64,
    wind_gust: f64,
    weather: Vec<Weather>,
    pop: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Daily {
    dt: i64,
    sunrise: i64,
    sunset: i64,
    moonrise: i64,
    moonset: i64,
    moon_phase: f64,
    temp: Temp,
    feels_like: FeelsLike,
    pressure: i64,
    humidity: i64,
    dew_point: f64,
    wind_speed: f64,
    wind_deg: i64,
    wind_gust: f64,
    weather: Vec<Weather>,
    clouds: i64,
    pop: f64,
    uvi: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Weather {
    id: i64,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Alert {
    sender_name: String,
    event: String,
    start: i64,
    end: i64,
    description: String,
    tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Temp {
    day: f64,
    min: f64,
    max: f64,
    night: f64,
    eve: f64,
    morn: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FeelsLike {
    day: f64,
    night: f64,
    eve: f64,
    morn: f64,
}