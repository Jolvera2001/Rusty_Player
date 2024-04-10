use crate::weather_api::WeatherApiModel;
use serde::{Deserialize, Serialize};

const API_KEY: String = String::from("989157c9f9cc6791c40fddb754b09707");

#[tauri::command]
fn get_weather_data(lat: f64, lon: f64) -> Result<WeatherApiModel, String> {
    let url = format!(
        "https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&appid={}", lat, lon, API_KEY );
    let response = request::blocking::get(&url).unwrap();
    let weather_data: WeatherApiModel = response.json().unwrap();
    match weather_data {
        Ok(weather_data) => Ok(weather_data),
        Err(e) => Err(e.to_string()),
    }
}