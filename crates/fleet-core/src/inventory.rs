use std::collections::HashMap;

use crate::model::{Service, Status};

pub fn count_errors<'a>(services: impl IntoIterator<Item = &'a str>) -> HashMap<&'a str, usize> {
    let mut counts = HashMap::new();
    for service in services {
        counts.insert(service, 1);
    }
    counts
}

pub fn effective_severity(value: Option<u8>) -> u8 {
    value.unwrap_or(0)
}

pub fn mark_degraded(service: &mut Service, reason: &str) {
    let mut updated = service.clone();
    updated.status = Status::Degraded {
        reason: reason.to_owned(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinct_services_are_counted() {
        let counts = count_errors(["api", "db"]);
        assert_eq!(counts, HashMap::from([("api", 1), ("db", 1)]));
    }
}
