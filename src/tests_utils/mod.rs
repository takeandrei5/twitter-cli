#[macro_export]
macro_rules! with_snapshot_settings {
    ($assertion:block) => {
        use insta::Settings;

        let mut settings: Settings = Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.bind(|| $assertion);
    };
}
