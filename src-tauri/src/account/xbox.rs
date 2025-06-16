use lazy_static::lazy_static;

pub mod live;
pub mod xsts;

const XBOX_LIVE_AUTH_DOMAIN: &str = "auth.xboxlive.com";

lazy_static! {
    static ref XBOX_LIVE_AUTH_URL: String =
        format!("https://user.{}/user/authenticate", XBOX_LIVE_AUTH_DOMAIN);
    static ref XSTS_AUTH_URL: String =
        format!("https://xsts.{}/xsts/authorize", XBOX_LIVE_AUTH_DOMAIN);
}
