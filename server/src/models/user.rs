use crate::{handlers::errors::AppError};
use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "users")]
pub struct User {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

impl User {
    pub async fn add_user(user: web::Json<User>, db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
        let user_info: User = user.into_inner();

        let client: Client = db_pool.get().await.map_err(AppError::PoolError)?;

        let new_user = User::store_user_to_db(&client, user_info).await?;

        Ok(HttpResponse::Ok().json(new_user))
    }

    pub async fn store_user_to_db(client: &Client, user_info: User) -> Result<User, AppError> {
        let config = crate::config::Config::from_file("Config.toml").unwrap();
        let db_name = config.pg.dbname.unwrap();
        let _stmt = format!("INSERT INTO {}.users(email, first_name, last_name, username)
                     VALUES ($1, $2, $3, $4)
                     RETURNING $table_fields;", &db_name);
        let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();

        client.query(&stmt, &[
                    &user_info.email,
                    &user_info.first_name,
                    &user_info.last_name,
                    &user_info.username,
                ])
            .await?
            .iter()
            .map(|row| User::from_row_ref(row).unwrap())
            .collect::<Vec<User>>()
            .pop()
            .ok_or(AppError::NotFound) // todo more applicable for SELECTs
    }
}