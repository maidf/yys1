use std::{str::FromStr, time::Duration};

use db_activity::{delete_activity, insert_activity, select_activity};
use sqlx::{
    Pool, Sqlite, migrate,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use structs::Activity;

mod db_activity;
mod db_res;
mod structs;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(2)
        .connect_with(
            SqliteConnectOptions::from_str("sqlite://yys.db")?
                .create_if_missing(true)
                .busy_timeout(Duration::from_secs(1)),
        )
        .await;

    let pool = match pool {
        Ok(pool) => {
            // create_table(&pool).await?;
            migrate!("./migrations")
                .run(&pool)
                .await
                .inspect(|_| println!("Database migration completed"))
                .expect("Failed to migrate database");
            pool
        }
        Err(e) => {
            eprintln!("Error connecting to database: {e}");
            return Err(e);
        }
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(pool)
        .invoke_handler(tauri::generate_handler![
            greet,
            add_activity,
            get_activity,
            rm_activity,
            add_res_type,
            get_res_type,
            rm_res_type
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(rename_all = "snake_case")]
async fn add_activity(
    activity: Activity,
    pool: tauri::State<'_, Pool<Sqlite>>,
) -> Result<String, String> {
    let res = insert_activity(&pool, activity).await;
    match res {
        Ok(_) => Ok("Activity added successfully".to_string()),
        Err(_) => Err(format!("Error adding activity")),
    }
}

#[tauri::command]
async fn get_activity(pool: tauri::State<'_, Pool<Sqlite>>) -> Result<Vec<Activity>, String> {
    let res = select_activity(&pool).await;
    match res {
        Ok(a) => Ok(a),
        Err(e) => Err(format!("Error get activity: {e}")),
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn rm_activity(id: &str, pool: tauri::State<'_, Pool<Sqlite>>) -> Result<String, String> {
    let res = delete_activity(&pool, id.to_string()).await;
    match res {
        Ok(_) => Ok("Activity rm successfully".to_string()),
        Err(_) => Err(format!("Error remove activity")),
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn add_res_type(name: &str, pool: tauri::State<'_, Pool<Sqlite>>) -> Result<String, String> {
    let res = db_res::insert_res_type(&pool, name).await;
    match res {
        Ok(_) => Ok("Resource type added successfully".to_string()),
        Err(_) => Err(format!("Error adding resource type")),
    }
}

#[tauri::command]
async fn get_res_type(
    pool: tauri::State<'_, Pool<Sqlite>>,
) -> Result<Vec<structs::ResType>, String> {
    let res = db_res::select_res_type(&pool).await;
    match res {
        Ok(a) => Ok(a),
        Err(_) => Err(format!("Error get resource type")),
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn rm_res_type(id: &str, pool: tauri::State<'_, Pool<Sqlite>>) -> Result<String, String> {
    let res = db_res::delete_res_type(&pool, id.to_string()).await;
    match res {
        Ok(_) => Ok("Resource type rm successfully".to_string()),
        Err(_) => Err(format!("Error remove resource type")),
    }
}
