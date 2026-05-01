use crate::channel::ChannelState;

pub const USER_DOCS_URL: &str = "https://github.com/tzero86/Zterm";
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub const GITHUB_ISSUES_URL: &str = "https://github.com/tzero86/Zterm/issues";
pub const SLACK_URL: &str = "https://github.com/tzero86/Zterm/discussions";
pub const PRIVACY_POLICY_URL: &str = "https://github.com/tzero86/Zterm/blob/main/PRIVACY.md";

pub fn feedback_form_url() -> String {
    let mut url = url::Url::parse("https://github.com/tzero86/Zterm/issues/new/choose")
        .expect("Should not fail to parse");
    if let Some(version) = ChannelState::app_version() {
        url.query_pairs_mut().append_pair("zterm-version", version);
    }
    url.query_pairs_mut()
        .append_pair("os-version", &os_info::get().version().to_string());
    url.to_string()
}
