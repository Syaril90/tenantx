use super::model::Asset;
use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

pub async fn list_assets(pool: &PgPool) -> sqlx::Result<Vec<Asset>> {
    sqlx::query_as::<_, Asset>("SELECT * FROM assets")
        .fetch_all(pool)
        .await
        .map_err(|err| {
            error!(error = %err, "Error in service::list_assets");
            err
        })
}

pub async fn get_asset(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<Asset>> {
    sqlx::query_as::<_, Asset>("SELECT * FROM assets WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|err| {
            error!(error = %err, asset_id = %id, "Error in service::get_asset");
            err
        })
}

pub async fn create_asset(pool: &PgPool, asset: Asset) -> sqlx::Result<Asset> {
    sqlx::query_as::<_, Asset>(
        "INSERT INTO assets (id, name, description, address) VALUES ($1, $2, $3, $4) RETURNING *",
    )
    .bind(asset.id)
    .bind(&asset.name)
    .bind(&asset.description)
    .bind(&asset.address)
    .fetch_one(pool)
    .await
    .map_err(|err| {
        error!(error = %err, asset_id = %asset.id, "Error in service::create_asset");
        err
    })
}

pub async fn update_asset(pool: &PgPool, id: Uuid, asset: Asset) -> sqlx::Result<Option<Asset>> {
    sqlx::query_as::<_, Asset>(
        "UPDATE assets SET name = $1, description = $2, address = $3 WHERE id = $4 RETURNING *",
    )
    .bind(&asset.name)
    .bind(&asset.description)
    .bind(&asset.address)
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|err| {
        error!(error = %err, asset_id = %id, "Error in service::update_asset");
        err
    })
}

pub async fn delete_asset(pool: &PgPool, id: Uuid) -> sqlx::Result<bool> {
    let result = sqlx::query("DELETE FROM assets WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|err| {
            error!(error = %err, asset_id = %id, "Error in service::delete_asset");
            err
        })?;

    Ok(result.rows_affected() > 0)
}
