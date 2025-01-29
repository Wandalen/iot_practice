use actix_web::{
    get, post,
    web::{Data, Json, Query, ServiceConfig},
    Error,
};
use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::{model::sensors::SensorsData, repository::sensors_repository::SensorsRepository};

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Deserialize)]
pub struct TimePeriod {
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
}

#[get("/api/get_data")]
async fn get_data(
    query: Query<TimePeriod>,
    sensors_repository: Data<SensorsRepository>,
) -> Result<Json<Vec<SensorsData>>, Error> {
    let res = sensors_repository
        .get_data(query.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(Json(res))
}

// curl --data '{"message_id":"test","device_id":"bob","timestamp":"2025-01-29T10:03:59.964","temperature":36.0,"humidity":50.0}' --header 'Content-Type: application/json' http://localhost:8080/api/put_data
#[post("/api/put_data")]
async fn put_data(
    Json(data): Json<SensorsData>,
    sensors_repository: Data<SensorsRepository>,
) -> Result<&'static str, Error> {
    sensors_repository
        .put_data(&data)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok("Success")
}

#[get("/api/alerts")]
async fn poll_alerts(sensors_repository: Data<SensorsRepository>) -> Result<&'static str, Error> {
    let poll = tokio::spawn(async move {
        while let Ok(data) = sensors_repository.get_data_last().await {
            if data.humidity < 20. || data.humidity > 70. {
                return Ok("humidity");
            } else if data.temperature < 16. || data.temperature > 40. {
                return Ok("temperature");
            }
        }
        Err("Failed fetching data")
    });
    let timeout = tokio::time::sleep(std::time::Duration::from_secs(10));

    tokio::select! {
      res = poll => {
        res.map_err(actix_web::error::ErrorInternalServerError)?
          .map_err(actix_web::error::ErrorInternalServerError)
      }
      _ = timeout => {
        Ok("")
      }
    }
}

pub fn routes(app: &mut ServiceConfig) {
    app.service(index);
    app.service(get_data);
    app.service(put_data);
    app.service(poll_alerts);
}
