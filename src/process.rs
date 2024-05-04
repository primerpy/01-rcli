use anyhow::{Ok, Result};
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    #[serde(rename = "Data.Precipitation")]
    data_precipitation: f32,
    #[serde(rename = "Date.Full")]
    date_full: String,
    #[serde(rename = "Date.Month")]
    date_month: u16,
    #[serde(rename = "Date.Week of")]
    date_week_of: u16,
    #[serde(rename = "Date.Year")]
    date_year: u16,
    #[serde(rename = "Station.City")]
    station_city: String,
    #[serde(rename = "Station.Code")]
    station_code: String,
    #[serde(rename = "Station.Location")]
    station_location: String,
    #[serde(rename = "Station.State")]
    station_state: String,
    #[serde(rename = "Data.Temperature.Avg Temp")]
    data_temperature_avg_temp: i32,
    #[serde(rename = "Data.Temperature.Max Temp")]
    data_temperature_max_temp: i32,
    #[serde(rename = "Data.Temperature.Min Temp")]
    data_temperature_min_temp: i32,
    #[serde(rename = "Data.Wind.Direction")]
    data_wind_direction: u32,
    #[serde(rename = "Data.Wind.Speed")]
    data_wind_speed: f32,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Record = result?;
        ret.push(record);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}
