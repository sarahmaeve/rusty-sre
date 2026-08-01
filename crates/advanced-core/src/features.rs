pub fn enabled_checks() -> Vec<&'static str> {
    #[cfg(feature = "audit")]
    {
        vec!["audit"]
    }
    #[cfg(not(feature = "audit"))]
    {
        vec!["schema", "bounds"]
    }
}
