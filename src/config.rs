
/// Proxy authentication information.
pub struct ProxyAuth {
	/// Proxy username.
	pub username: String,

	/// Proxy password.
	pub password: String,
}

pub struct BrowserConfig {
	/// Disables the web browser GUI.
	pub headless: bool,

	/// Installs the browser if it is not present.
	pub ensure_installation: bool,

	/// Proxy server URL.
	pub proxy: Option<String>,

	/// Authentication information for authenticated proxies.
	pub proxy_auth: Option<ProxyAuth>,

	/// A set of extra arguments to pass to the browser spawning command.
	pub extra_args: Option<Vec<String>>,
}

impl Default for BrowserConfig {
	fn default() -> Self {
		Self {
			headless: false,
			ensure_installation: true,
			proxy: None,
			proxy_auth: None,
			extra_args: Some(Vec::new()),
		}
	}
}
