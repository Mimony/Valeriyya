#![allow(dead_code)]
use bson::{doc, Array, oid::ObjectId};
use mongodb::{Client, error::Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelStruct {
    logs: String,
    welcome: String
}

impl ChannelStruct {
    pub fn set_logs(&mut self, value: &str) {
        self.logs = value.to_string();
    }

    pub fn set_welcome(&mut self, value: &str) {
        self.welcome = value.to_string();
    }

    pub fn delete_logs(&mut self) {
        self.logs = "".to_string();
    }

    pub fn delete_welcome(&mut self) {
        self.welcome = "".to_string();
    }

    pub fn delete_everything(&mut self) {
        self.delete_welcome();
        self.delete_logs();
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GuildDb {
    _id: ObjectId,
    gid: String,
    cases: Array,
    cases_number: u32,
    history: Array,
    channels: ChannelStruct
}


pub async fn access_guild(client: &Client) -> Result<Option<GuildDb>, Error>{
    let collection = client.database("myFirstDatabase").collection::<GuildDb>("guild");
    collection.find_one(
        doc! {
            "gid": "909850768947937290"
        },
        None
    ).await
}