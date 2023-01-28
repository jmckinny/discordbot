use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serenity::model::prelude::UserId;
use std::fs::File;
use std::io::{Error, Write};
use tracing::info;

pub fn save_data(data: HashMap<UserId, u64>) -> Result<(), Error> {
    let database = Database { data };
    let json = serde_json::to_string(&database)?;
    let mut f = File::create("database.json")?;
    f.write_all(json.as_bytes())?;
    info!("Database saved successfully");

    Ok(())
}

pub fn load_data() -> Result<HashMap<UserId, u64>, Error> {
    let json = std::fs::read_to_string("database.json")?;
    let database: Database = serde_json::from_str(&json)?;
    info!("Loaded {} entries successfully", database.data.len());
    Ok(database.data)
}

#[derive(Serialize, Deserialize)]
struct Database {
    data: HashMap<UserId, u64>,
}
