use sqlx::PgPool;

use crate::{model::sensors::SensorsData, routes::TimePeriod};

pub struct SensorsRepository {
    pool: PgPool,
}

impl SensorsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_data(&self, tp: TimePeriod) -> Result<Vec<SensorsData>, sqlx::Error> {
        if let (Some(start_date), Some(end_date)) = (&tp.start_date, &tp.end_date) {
            sqlx::query_as!(
                SensorsData,
                "SELECT * FROM iot_sensors_data AS sd WHERE sd.timestamp >= $1 AND sd.timestamp <= $2",
                start_date,
                end_date
            )
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as!(SensorsData, "SELECT * FROM iot_sensors_data")
                .fetch_all(&self.pool)
                .await
        }
    }

    pub async fn put_data(&self, data: &SensorsData) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO iot_sensors_data (device_id, timestamp, temperature, humidity, weight) VALUES ($1, $2, $3, $4, $5)",
            data.device_id,
            data.timestamp,
            data.temperature,
            data.humidity,
            data.weight,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_data_last(&self) -> Result<SensorsData, sqlx::Error> {
        sqlx::query_as!(
            SensorsData,
            "SELECT * FROM iot_sensors_data ORDER BY timestamp DESC LIMIT 1"
        )
        .fetch_one(&self.pool)
        .await
    }
}
