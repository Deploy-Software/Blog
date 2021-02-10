use {
    crate::records::settings::{NewSetting, Settings},
    async_graphql::{Context, Result},
    sqlx::PgPool,
    std::collections::HashMap,
};

pub async fn get_all<'a>(ctx: &'a Context<'_>) -> Result<HashMap<String, String>> {
    let pg_pool = ctx.data::<PgPool>()?;
    let settings = Settings::all(&pg_pool).await?;
    let mut settings_map = HashMap::new();

    for setting in settings {
        settings_map.insert(setting.key.clone(), setting.value.clone());
    }

    Ok(settings_map)
}

pub async fn add<'a>(ctx: &'a Context<'_>, key: String, value: String) -> Result<&'a str> {
    let pg_pool = ctx.data::<PgPool>()?;
    let new_setting = NewSetting::new(&key, &value)?;
    let _setting = new_setting.insert(&pg_pool).await?;
    Ok("OK")
}
