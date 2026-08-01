pub fn deployment_mode(runtime_value: Option<&str>) -> &str {
    let _ = runtime_value;
    option_env!("RUSTY_SRE_DEPLOYMENT").unwrap_or("stable")
}
