pub mod oauth;

const MICROSOFT_OAUTH_API_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0";

const MICROSOFT_CLIENT_ID: &str = env!("MICROSOFT_CLIENT_ID", "MICROSOFT_CLIENT_ID is not defined");
const REDIRECT_URI: &str = "vault-launcher://account/login/callback";
