#![allow(dead_code)]
use bson::{doc, oid::ObjectId};
use mongodb::{Client, error::Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelStruct {
    logs: String,
    welcome: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Case {
    id: u32,
    action: String,
    guild_id: String,
    staff_id: String,
    target_id: String,
    date: u64,
    reason: String,
    expiration: u64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    id: String,
    ban: u16,
    kick: u16,
    mute: u16
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GuildDb {
    _id: ObjectId,
    gid: String,
    cases: Vec<Case>,
    cases_number: u32,
    history: Vec<History>,
    channels: ChannelStruct
}


pub async fn access_guild(client: &Client, gid: String) -> Result<Option<GuildDb>, Error> {
    let collection = client.database("myFirstDatabase").collection::<GuildDb>("guild");
    collection.find_one(
        doc! {
            "gid": gid
        },
        None
    ).await
}

