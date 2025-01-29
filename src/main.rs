use std::net::TcpListener;

use actix_web::{dev::Server, web::Data, App, HttpServer};
use iot_practice::{
    repository::{sensors_repository::SensorsRepository, staff_repository::StaffRepository},
    routes::routes,
    secret::Secret,
};
use sqlx::PgPool;

pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let staff_repository = Data::new(StaffRepository::new(pool.clone()));
    let sensors_repository = Data::new(SensorsRepository::new(pool.clone()));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(staff_repository.clone())
            .app_data(sensors_repository.clone())
            .configure(routes)
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret = Secret::read();

    let pool = PgPool::connect(&secret.DATABASE_URL)
        .await
        .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind address.");

    run(listener, pool)?.await
}
