use {
    crate::{
        records::{
            settings::{Settings},
        },
    },
    async_graphql::{Result, Context},
    sqlx::PgPool,
    std::collections::HashMap,
};

pub async fn get_all<'a>(ctx: &'a Context<'_>) -> Result<HashMap<String, String>> {
    let pg_pool = ctx.data::<PgPool>()?;
    let settings = Settings::all(&pg_pool).await?;
    let mut settings_map = HashMap::new();

    for setting in settings {
        settings_map.insert(
            setting.key.clone(),
            setting.value.clone(),
        );
    }

    Ok(settings_map)
}