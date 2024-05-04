// rcli csv -i input.csv -o output.json --header - ','
use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct Record {
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

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about=None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to Other Formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser=verify_input_file)]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}
fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;
            let mut ret = Vec::with_capacity(128);
            for result in reader.deserialize() {
                let record: Record = result?;
                ret.push(record);
            }
            let json = serde_json::to_string_pretty(&ret)?;
            std::fs::write(opts.output, json)?;
        }
    }
    Ok(())
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
