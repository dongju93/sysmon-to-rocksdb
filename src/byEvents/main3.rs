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
use structs::events::Event3;

// const EVENT_CODE: &str = EVE_CODE;
const EVENT_CODE: &str = "3";

fn parse_output(data: &serde_json::Value) -> Vec<Event3> {
    let mut entries = Vec::new();

    if let Some(hits) = data["hits"]["hits"].as_array() {
        for hit in hits {
            if let Some(message) = hit["_source"]["message"].as_str() {
                let mut entry = Event3 {
                    agent_name: None,
                    agent_id: None,
                    event_action: Some("Network connection detected".to_string()),
                    utc_time: None,
                    process_guid: None,
                    process_id: None,
                    image: None,
                    user: None,
                    protocol: None,
                    initiated: None,
                    source_is_ipv6: None,
                    source_ip: None,
                    source_hostname: None,
                    source_port: None,
                    source_port_name: None,
                    destination_is_ipv6: None,
                    destination_ip: None,
                    destination_hostname: None,
                    destination_port: None,
                    destination_port_name: None,
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
                            "User" => entry.image = Some(value.to_string()),
                            "Protocol" => entry.protocol = Some(value.to_string()),
                            "Initiated" => entry.initiated = Some(value.to_string()),
                            "SourceIsIpv6" => entry.source_is_ipv6 = Some(value.to_string()),
                            "SourceIp" => entry.source_ip = Some(value.to_string()),
                            "SourceHostname" => entry.source_hostname = Some(value.to_string()),
                            "SourcePort" => entry.source_port = Some(value.to_string()),
                            "SourcePortName" => entry.source_port_name = Some(value.to_string()),
                            "DestinationIsIpv6" => {
                                entry.destination_is_ipv6 = Some(value.to_string())
                            }
                            "DestinationIp" => entry.destination_ip = Some(value.to_string()),
                            "DestinationHostname" => {
                                entry.destination_hostname = Some(value.to_string())
                            }
                            "DestinationPort" => entry.destination_port = Some(value.to_string()),
                            "DestinationPortName" => {
                                entry.destination_port_name = Some(value.to_string())
                            }
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

fn write_to_csv(entries: Vec<Event3>, filename: &str) -> std::io::Result<()> {
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
