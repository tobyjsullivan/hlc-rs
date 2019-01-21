use serde::{Deserialize, Serialize};
use serde_json::Result as SResult;
use std::fs::{read_dir, File};
use std::io::BufReader;
use std::io::{Read, Result};
use std::path::Path;

use crate::data::{
    Store,
};

pub fn load(store: &mut Store, data_dir: &str) -> Result<()> {
    for entry in read_dir(data_dir)? {
        let entry = entry?;
        println!("Loading {:?}", entry);
        load_file(store, &entry.path())?;
    }

    Ok(())
}

fn load_file(store: &mut Store, file_path: &Path) -> Result<()> {
    let file_content = read_file(file_path)?;

    for acct in file_content.accounts {
        store.mark_account(acct.id);
    }

    Ok(())
}

fn read_file(file_path: &Path) -> Result<FileContent> {
    let f = File::open(file_path)?;
    let mut buf = BufReader::new(f);
    let mut content = String::new();

    buf.read_to_string(&mut content)?;

    let data: FileContent = serde_json::from_str(&content)?;

    Ok(data)
}

#[derive(Debug, Serialize, Deserialize)]
struct FileContent {
    accounts: Vec<Account>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    id: u32,
    email: String,
    fname: Option<String>,
    sname: Option<String>,
    phone: Option<String>,
    sex: String,
    birth: Timestamp,
    country: Option<String>,
    city: Option<String>,
    joined: Timestamp,
    status: String,
    interests: Option<Vec<String>>,
    premium: Option<Premium>,
    likes: Option<Vec<Like>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Premium {
    start: Timestamp,
    finish: Timestamp,
}

#[derive(Debug, Serialize, Deserialize)]
struct Like {
    id: u32,
    ts: Timestamp,
}

type Timestamp = u64;
