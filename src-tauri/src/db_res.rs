use sqlx::{Pool, Sqlite, query, query_as};
use uuid::Uuid;

use crate::structs::ResType;

pub async fn insert_res_type(pool: &Pool<Sqlite>, name: &str) -> Result<(), sqlx::Error> {
    let sql = r#"
        insert into res_type (id, name)
        values (?, ?)
    "#;

    let res = query(sql).bind(Uuid::now_v7().to_string()).bind(name).execute(pool).await;

    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn select_res_type(pool: &Pool<Sqlite>) -> Result<Vec<ResType>, sqlx::Error> {
    let sql = r#"
        select * from res_type
    "#;

    let types = query_as::<_, ResType>(sql).fetch_all(pool).await?;

    Ok(types)
}

pub async fn delete_res_type(pool: &Pool<Sqlite>, id: String) -> Result<(), sqlx::Error> {
    let sql = r#"
        delete from res_type where id = ?
    "#;

    query(sql).bind(id).execute(pool).await?;

    Ok(())
}