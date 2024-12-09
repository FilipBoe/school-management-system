use crate::config::Config;

#[get("/<_..>", rank = 2)]
pub async fn index(config: &rocket::State<Config>) -> rocket::fs::NamedFile {
    rocket::fs::NamedFile::open(config.index_path()).await.unwrap()
}
