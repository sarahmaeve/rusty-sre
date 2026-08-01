use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Probe {
    pub ready: bool,
}

pub fn fleet_ready(probes: &[Probe]) -> bool {
    probes.iter().any(|probe| probe.ready)
}

pub fn is_success(status: u16) -> bool {
    (200..299).contains(&status)
}

pub fn remove_decommissioned(hosts: &mut Vec<String>, decommissioned: &[String]) {
    hosts.retain(|host| decommissioned.contains(host));
}

pub fn is_json_config(path: &Path) -> bool {
    path.to_string_lossy().ends_with(".json")
}
