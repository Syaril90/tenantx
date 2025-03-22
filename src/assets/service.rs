use super::model::Asset;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn list_assets(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> sqlx::Result<(Vec<Asset>, i64)> {
    let assets =
        sqlx::query_as::<_, Asset>("SELECT * FROM assets ORDER BY name LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?;

    let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM assets")
        .fetch_one(pool)
        .await?;

    Ok((assets, total))
}

pub async fn get_asset(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<Asset>> {
    let result = sqlx::query_as::<_, Asset>("SELECT * FROM assets WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await;

    if let Err(err) = &result {
        eprintln!("Error in get_asset: {:?}", err);
    }

    result
}

pub async fn create_asset(pool: &PgPool, asset: Asset) -> sqlx::Result<Asset> {
    let address_json = match serde_json::to_value(&asset.address) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Serialization error: {:?}", e);
            return Err(sqlx::Error::Protocol(e.to_string().into()));
        }
    };

    let result = sqlx::query_as::<_, Asset>(
        "INSERT INTO assets (id, name, description, address) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(asset.id)
    .bind(&asset.name)
    .bind(&asset.description)
    .bind(address_json)
    .fetch_one(pool)
    .await;

    if let Err(err) = &result {
        eprintln!("Error in create_asset: {:?}", err);
    }

    result
}

pub async fn update_asset(pool: &PgPool, id: Uuid, asset: Asset) -> sqlx::Result<Option<Asset>> {
    let result = sqlx::query_as::<_, Asset>(
        "UPDATE assets SET name = $1, description = $2, address = $3 WHERE id = $4 RETURNING *",
    )
    .bind(&asset.name)
    .bind(&asset.description)
    .bind(&asset.address)
    .bind(id)
    .fetch_optional(pool)
    .await;

    if let Err(err) = &result {
        eprintln!("Error in update_asset: {:?}", err);
    }

    result
}

pub async fn delete_asset(pool: &PgPool, id: Uuid) -> sqlx::Result<bool> {
    let result = sqlx::query("DELETE FROM assets WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await;

    if let Err(err) = &result {
        eprintln!("Error in delete_asset: {:?}", err);
    }

    match result {
        Ok(res) => Ok(res.rows_affected() > 0),
        Err(e) => Err(e),
    }
}
