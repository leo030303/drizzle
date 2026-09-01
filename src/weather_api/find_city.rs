use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GeoResponse {
    pub name: String,
    pub country: String,
    pub admin1: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GeoResponseList {
    pub results: Vec<GeoResponse>,
}

pub async fn search_city_list(search_string: &str) -> Result<Vec<GeoResponse>, String> {
    const GEOCODING_BASE_URL: &str = "https://geocoding-api.open-meteo.com/v1/search";
    const NUM_SEARCH_RESULTS_TO_RETURN: u8 = 5;

    let geo_url =
        format!("{GEOCODING_BASE_URL}?name={search_string}&count={NUM_SEARCH_RESULTS_TO_RETURN}",);
    let geo_response = reqwest::get(geo_url)
        .await
        .map_err(|e| e.to_string())?
        .json::<GeoResponseList>()
        .await
        .map_err(|e| e.to_string())?;
    Ok(geo_response.results)
}
