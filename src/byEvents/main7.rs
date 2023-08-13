#![allow(deprecated)]

// External Dependecys, import through Cargo.toml
use tokio;

// Import Enviroments with secrect key (settings)
#[path = "../envs/mod.rs"]
mod envs;
// Import Sysmon event structs
#[path = "../structs/mod.rs"]
mod structs;

// use Imports
use envs::env::*;
use envs::byEventsEnv::*;
use structs::events::Event7;

// const EVENT_CODE: &str = EVE_CODE;
const EVENT_CODE: &str = "7";

fn parse_output(data: &serde_json::Value) -> Vec<Event7> {
    let mut entries = Vec::new();

    if let Some(hits) = data["hits"]["hits"].as_array() {
        for hit in hits {
            if let Some(message) = hit["_source"]["message"].as_str() {
                let mut entry = Event7 {
                    agent_name: None,
                    agent_id: None,
                    event_action: Some("Image loaded".to_string()),
                    utc_time: None,
                    process_guid: None,
                    process_id: None,
                    image: None,
                    image_loaded: None,
                    file_version: None,
                    description: None,
                    product: None,
                    company: None,
                    original_file_name: None,
                    hashes: None,
                    signed: None,
                    signature: None,
                    signature_status: None,
                    user: None,
                };

                if let Some(agent_name) = hit["_source"]["agent"]["name"].as_str() {
                    entry.agent_name = Some(agent_name.to_string());
                }

                if let Some(agent_id) = hit["_source"]["agent"]["id"].as_str() {
                    entry.agent_id = Some(agent_id.to_string());
                }

                for part in message.split('\n') {
                    let segments: Vec<_> = part.splitn(2, ':').collect();
                    // println!("{:?}", segments); // Debug prints
                    if segments.len() == 2 {
                        let key = segments[0].trim();
                        let value = segments[1].trim();
                        match key {
                            "UtcTime" => entry.utc_time = Some(value.to_string()),
                            "ProcessGuid" => entry.process_guid = Some(value.to_string()),
                            "ProcessId" => entry.process_id = Some(value.to_string()),
                            "Image" => entry.image = Some(value.to_string()),
                            "ImageLoaded" => entry.image_loaded = Some(value.to_string()),
                            "FileVersion" => entry.file_version = Some(value.to_string()),
                            "Description" => entry.description = Some(value.to_string()),
                            "Product" => entry.product = Some(value.to_string()),
                            "Company" => entry.company = Some(value.to_string()),
                            "OriginalFileName" => {
                                entry.original_file_name = Some(value.to_string())
                            }
                            "Hashes" => entry.hashes = Some(value.to_string()),
                            "Signed" => entry.signed = Some(value.to_string()),
                            "Signature" => entry.signature = Some(value.to_string()),
                            "SignatureStatus" => entry.signature_status = Some(value.to_string()),
                            "User" => entry.user = Some(value.to_string()),
                            _ => {}
                        }
                    }
                }

                entries.push(entry);
            }
        }
    }

    entries
}

fn write_to_csv(entries: Vec<Event7>, filename: &str) -> std::io::Result<()> {
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_path(filename)?;
    for entry in entries {
        wtr.serialize(entry)?;
    }
    wtr.flush()?;
    Ok(())
}

#[tokio::main]
async fn main() {
    match fetch_data_from_es().await {
        Ok(data) => {
            let entries = parse_output(&data);
            let filenames = format!("{}{}{}", SAVELOCATION, EVENT_CODE, CSVNAME);
            if let Err(e) = write_to_csv(entries, &filenames) {
                eprintln!("Error writing to CSV: {:?}", e);
            }
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}