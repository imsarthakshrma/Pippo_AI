use anyhow::Result;
use chrono::NaiveDate;
use reqwest::Client;
use serde::Deserialize;

/// A scraper for fetching historical weather data from the NOAA CDO API.
pub struct NOAAHistoricalScraper {
    client: Client,
    #[allow(dead_code)]
    api_token: String,
}

impl NOAAHistoricalScraper {
    /// Creates a new `NOAAHistoricalScraper` with the provided API token.
    pub fn new(api_token: String) -> Self {
        Self {
            client: Client::new(),
            api_token,
        }
    }

    /// Fetches weather data for a specific location and date.
    /// 
    /// currently this is a stub that returns placeholder data.
    pub async fn fetch_weather_for_date(
        &self,
        _location: &str,
        _date: NaiveDate,
    ) -> Result<WeatherData> {
        // Placeholder for real NOAA API call
        Ok(WeatherData {
            temp_high: 15.0,
            temp_low: 5.0,
            precipitation: 0.0,
            conditions: "Clear".to_string(),
        })
    }
}

/// Represents weather information for a specific date and location.
#[derive(Debug, Deserialize)]
pub struct WeatherData {
    /// Maximum temperature in Celsius.
    pub temp_high: f64,
    /// Minimum temperature in Celsius.
    pub temp_low: f64,
    /// Total precipitation in mm.
    pub precipitation: f64,
    /// Summary of weather conditions (e.g., "Clear", "Rain").
    pub conditions: String,
}
