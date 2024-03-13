use crate::rqlite;

#[derive(Debug, Clone)]
pub struct RqliteConnectOptions {
    pub(crate) inner: rqlite::ConnectOptions,
}

impl RqliteConnectOptions {
  pub fn host(mut self, host: &str) -> Self {
    self.inner.host = host.to_owned();
    self
  }
  pub fn port(mut self, port: u16) -> Self {  
    self.inner.port = port;
    self
  }
  pub fn user(mut self, user: Option<String>) -> Self {  
    self.inner.user = user;
    self
  }
  pub fn password(mut self, pwd: Option<String>) -> Self {  
    self.inner.pass = pwd;
    self
  }
  pub fn use_ssl(mut self, use_ssl: bool) -> Self {  
    self.inner.scheme = if use_ssl {
      rqlite::Scheme::HTTPS
    } else {
      rqlite::Scheme::HTTP
    };
    self.inner.accept_invalid_cert=false;
    self
  }
  pub fn use_insecure_ssl(mut self, use_insecure_ssl: bool) -> Self {
    if use_insecure_ssl {
      self.inner.scheme = rqlite::Scheme::HTTPS;
      self.inner.accept_invalid_cert = true;
    } else {
      self.inner.accept_invalid_cert = false;
      self.inner.scheme = rqlite::Scheme::HTTP;
    }
    self
  }
}

pub mod connect;
mod parse;
