use sqlx::{query, query_as, Pool, Sqlite};
use uuid::Uuid;

use crate::structs::Activity;

pub async fn create_table(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    let sql = r#"
    CREATE TABLE IF NOT EXISTS res_type(
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL
    );
    
    CREATE TABLE IF NOT EXISTS res(
        id TEXT PRIMARY KEY,
        tid TEXT NOT NULL,
        aid TEXT NOT NULL,
        num UNSIGNED INTEGER NOT NULL DEFAULT 0
    );
    
    CREATE TABLE IF NOT EXISTS activity(
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        num UNSIGNED INTEGER NOT NULL DEFAULT 0,
        consume UNSIGNED INTEGER NOT NULL DEFAULT 0
    );
    "#;

    query(sql).execute(pool).await?;

    Ok(())
}

pub async fn create_activity(pool: &Pool<Sqlite>, activity: Activity) -> Result<(), sqlx::Error> {
    // let conn = AnyConnection::connect("sqlite://yys.db").await.unwrap();
    let sql = r#"
        insert into activity (id, name, num, consume)
        values (?, ?, ?, ?)
    "#;

    query(sql)
        .bind(Uuid::new_v4().to_string())
        .bind(activity.name)
        .bind(activity.num)
        .bind(activity.consume)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn select_activity(pool: &Pool<Sqlite>) -> Result<Vec<Activity>, sqlx::Error> {
    let sql = r#"
        select * from activity
    "#;

    let activities = query_as::<_, Activity>(sql)
        .fetch_all(pool)
        .await?;

    Ok(activities)
}

pub async fn delete_activity(pool: &Pool<Sqlite>, id: String) -> Result<(), sqlx::Error> {
    let sql = r#"
        delete from activity where id = ?
    "#;

    query(sql)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}