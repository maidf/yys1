use std::{str::FromStr, time::Duration};

use db::{create_activity, create_table, delete_activity, select_activity};
use sqlx::{
    Pool, Sqlite,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use structs::Activity;

mod db;
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
            create_table(&pool).await?;
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
        .invoke_handler(tauri::generate_handler![greet, add_activity, get_activity, rm_activity])
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
    let res = create_activity(&pool, activity).await;
    match res {
        Ok(_) => Ok("Activity added successfully".to_string()),
        Err(e) => Err(format!("Error adding activity: {e}")),
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
async fn rm_activity(id: String, pool: tauri::State<'_, Pool<Sqlite>>) -> Result<String, String> {
    let res = delete_activity(&pool, id).await;
    match res {
        Ok(_) => Ok("Activity rm successfully".to_string()),
        Err(e) => Err(format!("Error rm activity: {e}")),
    }
}
