use once_cell::sync::Lazy;
use shuttle_runtime::SecretStore;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Result, Surreal,
};

pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn connect_db(secrets: SecretStore) -> Result<()> {
    let _ = DB
        .connect::<Ws>(&format!(
            "{}:{}",
            &secrets.get("URL").expect("database url should be set"),
            &secrets.get("PORT").expect("database port should be set")
        ))
        .await?;
    let _ = DB
        .signin(Root {
            username: &secrets
                .get("USERNAME")
                .expect("database username should be set"),
            password: &secrets
                .get("PASSWORD")
                .expect("database password should be set"),
        })
        .await;
    let _ = DB
        .use_ns(
            &secrets
                .get("NAMESPACE")
                .expect("database namespace should be set"),
        )
        .use_db(&secrets.get("DBNAME").expect("database name should be set"))
        .await?;
    Ok(())
}
