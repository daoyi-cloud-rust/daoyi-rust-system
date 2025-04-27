use daoyi_cloud_common::{config, db, redis};
use daoyi_cloud_common::config::{Data, Env, Figment, Format, ProfileActiveConfig, Toml};

fn local_init() -> Option<Data<Toml>> {
    let data = Toml::file(
        Env::var("APP_CONFIG")
            .as_deref()
            .unwrap_or("resources/config.toml"),
    );
    let raw_config = Figment::new()
        .merge(data)
        .merge(Env::prefixed("APP_").global());
    let profile_config = match raw_config.extract::<ProfileActiveConfig>() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("It looks like your config is invalid. The following error occurred: {e}");
            std::process::exit(1);
        }
    };
    println!("profile_active {:?}", profile_config.profile_active);
    Some(Toml::file(Env::var("APP_CONFIG").as_deref().unwrap_or(
        format!("resources/config-{}.toml", profile_config.profile_active).as_str(),
    )))
}

pub async fn app_init() {
    config::common_init(local_init());
    let config = config::get();
    redis::init_redis_pool(&config.redis).await;
    db::init(&config.db).await;
}
