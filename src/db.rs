use once_cell::sync::Lazy;
use surrealdb::{
    engine::remote::ws::{Client, Wss},
    opt::auth::Root,
    Result, Surreal,
};

use crate::config::DatabaseSettings;

pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn connect_db(settings: DatabaseSettings) -> Result<()> {
    let _ = DB.connect::<Wss>(&settings.url).await?;
    let _ = DB
        .signin(Root {
            username: &settings.username,
            password: &settings.password,
        })
        .await;
    let _ = DB
        .use_ns(&settings.namespace)
        .use_db(&settings.dbname)
        .await?;
    Ok(())
}
