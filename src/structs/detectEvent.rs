#![allow(dead_code)]

extern crate chrono;
use chrono::{DateTime, Utc};
use std::net::IpAddr;

struct RansomwareReg {
    time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    user: String,
    target_object: String,
    details: String,
    rule_id: u32,
    matched_to: String,
    cluster_id: usize,
    attack_kind: String,
    confidence: f32,
    triage_scores: Option<Vec<TriageScore>>,
    agent_id: String,
}

struct RansomwareFile {
    time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    user: String,
    target_filename: String,
    creation_utc_time: DateTime<Utc>,
    rule_id: u32,
    matched_to: String,
    cluster_id: usize,
    attack_kind: String,
    confidence: f32,
    triage_scores: Option<Vec<TriageScore>>,
    agent_id: String,
}

struct Rootkit {
    time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    user: String,
    file_version: String,
    description: String,
    product: String,
    company: String,
    original_file_name: String,
    command_line: String,
    current_directory: String,
    logon_guid: String,
    logon_id: u32,
    terminal_session_id: u32,
    integrity_level: String,
    hashes: String,
    parent_process_guid: String,
    parent_process_id: u32,
    parent_image: String,
    parent_command_line: String,
    parent_user: String,
    db_name: String,
    rule_id: u32,
    matched_to: String,
    cluster_id: usize,
    attack_kind: String,
    confidence: f32,
    triage_scores: Option<Vec<TriageScore>>,
    agent_id: String,
}

struct InformationLeak {
    time: DateTime<Utc>,
    process_guid: String,
    process_id: u32,
    image: String,
    user: String,
    protocol: String,
    initiated: bool,
    source_is_ipv6: bool,
    source_ip: IpAddr,
    source_hostname: String,
    source_port: u16,
    source_port_name: String,
    destination_is_ipv6: bool,
    destination_ip: IpAddr,
    destination_hostname: String,
    destination_port: u16,
    destination_port_name: String,
    db_name: String,
    rule_id: u32,
    matched_to: String,
    cluster_id: usize,
    attack_kind: String,
    confidence: f32,
    triage_scores: Option<Vec<TriageScore>>,
    agent_id: String,
}

struct TriageScore {
    policy_id: u32,
    score: f64,
}
