use insta::Settings;

pub(crate) fn with_snapshot_insta_settings(assertion: impl FnOnce()) {
    let mut settings: Settings = Settings::clone_current();
    settings.set_prepend_module_to_snapshot(false);
    settings.bind(assertion);
}
