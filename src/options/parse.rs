use crate::RqliteConnectOptions;
use percent_encoding::utf8_percent_encode;
use percent_encoding::NON_ALPHANUMERIC;
use sqlx_core::Url;

impl RqliteConnectOptions {
    pub(crate) fn build_url(&self) -> Url {
        let mut url = Url::parse(&format!(
            "rqlite://{}:{}",
            /*self.inner.username, */ self.inner.host, self.inner.port
        ))
        .expect("BUG: generated un-parseable URL");

        if let Some(user) = &self.inner.user {
            let _ = url.set_username(&user);
        }

        if let Some(password) = &self.inner.pass {
            let password = utf8_percent_encode(&password, NON_ALPHANUMERIC).to_string();
            let _ = url.set_password(Some(&password));
        }
        url
    }
}
