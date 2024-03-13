//use std::error::Error;
use serde_json::json;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
//use tokio_native_tls::native_tls::TlsConnector;
use crate::cursor::Cursor;
use crate::error::RqliteError;
use hyper::client::conn::{self, SendRequest};
use hyper::{Body, Request};
use serde::Deserialize;
use tokio_rustls::TlsConnector;

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;

use rustls::{/*ClientConfig, server::NoClientAuth,*/ RootCertStore};
use std::sync::Arc;
//use rustls::client::danger::{ServerCertVerifier,ServerCertVerified,};
use crate::types::BoxDynError;
use std::convert::TryFrom;
use std::io;

//use std::iter::FromIterator;
mod danger {
    use rustls::client::danger::HandshakeSignatureValid;
    use rustls::crypto::{verify_tls12_signature, verify_tls13_signature};
    use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
    use rustls::DigitallySignedStruct;

    #[derive(Debug)]
    pub struct NoCertificateVerification {}

    impl rustls::client::danger::ServerCertVerifier for NoCertificateVerification {
        fn verify_server_cert(
            &self,
            _end_entity: &CertificateDer<'_>,
            _intermediates: &[CertificateDer<'_>],
            _server_name: &ServerName<'_>,
            _ocsp: &[u8],
            _now: UnixTime,
        ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
            Ok(rustls::client::danger::ServerCertVerified::assertion())
        }

        fn verify_tls12_signature(
            &self,
            message: &[u8],
            cert: &CertificateDer<'_>,
            dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, rustls::Error> {
            verify_tls12_signature(
                message,
                cert,
                dss,
                &rustls::crypto::ring::default_provider().signature_verification_algorithms,
            )
        }

        fn verify_tls13_signature(
            &self,
            message: &[u8],
            cert: &CertificateDer<'_>,
            dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, rustls::Error> {
            verify_tls13_signature(
                message,
                cert,
                dss,
                &rustls::crypto::ring::default_provider().signature_verification_algorithms,
            )
        }

        fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
            rustls::crypto::ring::default_provider()
                .signature_verification_algorithms
                .supported_schemes()
        }
    }
}

/// Enum to specify connection scheme when creating a connections
#[repr(u8)]
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Scheme {
    #[default]
    HTTP,
    HTTPS,
}

/// Connection builder
#[derive(Clone, Debug, Default)]
pub struct ConnectOptions {
    pub scheme: Scheme,
    pub host: String,
    pub port: u16,
    pub user: Option<String>,
    pub pass: Option<String>,
    //max_redirects: isize,
    pub accept_invalid_cert: bool,
}

trait Socket: Sync + Send + AsyncWrite + AsyncRead + Unpin {}
impl Socket for TcpStream {}
impl<S: Sync + Send + AsyncWrite + AsyncRead + Unpin> Socket
    for tokio_rustls::client::TlsStream<S>
{
}

/// Rqlite connection object
#[derive(Debug)]
pub struct Connection {
    // connection can either be a tcpstream or tokio_native_tls object
    // storing it in heap
    pub(crate) request_sender: SendRequest<Body>,
    pub(crate) settings: ConnectOptions,
}

impl ConnectOptions {
    /// Create a new connection to a rqlite node
    /// ```
    /// use rqlite::ConnectOptions;
    ///
    /// let mut conn = ConnectOptions::new("127.0.0.1", 4001)
    ///		.connect().await?;
    /// ```
    pub fn new(host: &str, port: u16) -> ConnectOptions {
        ConnectOptions {
            scheme: Scheme::HTTP,
            host: host.to_owned(),
            port,
            user: None,
            pass: None,
            //max_redirects: -1,
            accept_invalid_cert: false,
        }
    }

    /// Set scheme for connection (http (default) or https)
    /// ```
    /// let mut conn = ConnectOptions::new("my.node.local", 4001)
    ///		.scheme(Scheme::HTTPS)
    ///		.connect().await?;
    /// ```
    pub fn scheme<'a>(&'a mut self, scheme: Scheme) -> &'a mut ConnectOptions {
        self.scheme = scheme;
        self
    }

    /// set user for basic authentification
    /// ```
    /// let mut conn = ConnectOptions::new("my.node.local", 4001)
    ///		.scheme(Scheme::HTTPS)
    ///		.user("root")
    ///		.connect().await?;
    /// ```
    pub fn user<'a>(&'a mut self, user: &str) -> &'a mut ConnectOptions {
        self.user = Some(user.to_owned());
        self
    }

    /// set user for basic authentification
    /// ```
    /// let mut conn = ConnectOptions::new("my.node.local", 4001)
    ///		.scheme(Scheme::HTTPS)
    ///		.user("root")
    ///		.pass("root")
    ///		.connect().await?;
    /// ```

    pub fn pass<'a>(&'a mut self, pass: &str) -> &'a mut ConnectOptions {
        self.pass = Some(pass.to_owned());
        self
    }

    /*/// Set max redirects in connection
    pub fn max_redirects<'a>(&'a mut self, redirects: usize) -> &'a mut ConnectOptions {
        self.max_redirects = redirects as isize;
        self
    }

    /// Allow infinite number of redirects
    pub fn infinite_redirects<'a>(&'a mut self, inf: bool) -> &'a mut ConnectOptions {
        if inf {
            self.max_redirects = -1;
        }
        self
    }*/

    /// Accept invalid TLS certificates.
    /// ```
    /// let mut conn = ConnectOptions::new("my.node.local", 4001)
    ///		.scheme(Scheme::HTTPS)
    ///		.user("root")
    ///		.accept_invalid_cert(true)
    ///		.connect().await?;
    /// ```
    pub fn accept_invalid_cert<'a>(&'a mut self, accept: bool) -> &'a mut ConnectOptions {
        self.accept_invalid_cert = accept;
        self
    }

    /// Establish connection to rqlite node
    /// ```
    /// let mut conn = ConnectOptions::new("my.node.local", 4001)
    ///		.scheme(Scheme::HTTPS)
    ///		.user("root")
    ///		.accept_invalid_cert(true)
    ///		.connect().await?;
    /// ```
    ///
    /// Returns Error on unsuccessful connection or error creating Tls context
    pub async fn connect<'a>(&'a self) -> Result<Connection, BoxDynError> {
        let sock = TcpStream::connect(format!("{}:{}", self.host, self.port)).await?;
        let socket;

        match self.scheme {
            Scheme::HTTPS => {
                /*let mut config_builder = rustls::ClientConfig::builder().with_no_client_auth()with_safe_default_protocol_versions()
                /*
                .danger_accept_invalid_certs(if self.accept_invalid_cert { true } else { false })
                .danger_accept_invalid_hostnames(if self.accept_invalid_cert { true } else { false })

                */
                .unwrap();
                */

                /*let builder = TlsConnector::builder()
                                        .danger_accept_invalid_certs(if self.accept_invalid_cert { true } else { false })
                                        .danger_accept_invalid_hostnames(if self.accept_invalid_cert { true } else { false })
                                        .build().unwrap();
                */
                let mut root_store = RootCertStore::empty();
                root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
                let mut config = rustls::ClientConfig::builder()
                    .with_root_certificates(root_store)
                    .with_no_client_auth();

                if self.accept_invalid_cert {
                    config
                        .dangerous()
                        .set_certificate_verifier(Arc::new(danger::NoCertificateVerification {}));
                }

                let cx = TlsConnector::from(Arc::new(config));
                let domain = rustls::pki_types::ServerName::try_from(self.host.as_str())
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid dnsname"))?
                    .to_owned();
                socket = Box::new(cx.connect(domain, sock).await?) as Box<dyn Socket>;
            }
            _ => socket = Box::new(sock) as Box<dyn Socket>,
        };

        let (req, con) = conn::handshake(socket).await?;
        tokio::spawn(async move {
            con.await.ok();
        });

        Ok(Connection {
            request_sender: req,
            settings: self.clone(),
        })
    }
}

/// Node information
#[derive(Debug)]
pub struct Node {
    /// Node id
    pub id: String,
    /// Node ip:port
    pub api_addr: String,
    /// Node raft ip:port
    pub addr: String,
    /// Is node active
    pub reachable: bool,
    /// Is node the current leader
    pub leader: bool,
    /// Latency in communication
    pub time: f32,
}

// struct to help for deserializing
#[derive(Deserialize)]
struct _Node {
    api_addr: String,
    addr: String,
    reachable: bool,
    leader: bool,
    time: f32,
}

/// Rqlite connection
impl Connection {
    /// Get a cursor, and use it to do sql queries
    /// ```
    /// let mut conn = ConnectOptions::new("127.0.0.1", 4001)
    ///		.connect().await?;
    /// let cur = conn.cursor();
    /// cur.execute("SELECT * FROM foo", par!())?;
    /// ```
    pub fn cursor<'a>(&'a mut self) -> Cursor<'a> {
        Cursor::new(self)
    }

    /// Execute a sql query
    /// ```
    /// let mut conn = ConnectOptions::new("127.0.0.1", 4001)
    ///		.connect().await?;
    /// if conn.execute("INSERT INTO foo(name) VALUES (?)", par!("fiona")).await?.rows_affected() == 1 {
    ///     println!("fiona is now a member of foo");
    /// };
    /// ```
    ///
    /// Returns RqliteError on error to handle exception explicitly
    pub async fn execute<'a>(
        &'a mut self,
        query: &str,
        params: Vec<serde_json::Value>,
    ) -> Result<Cursor<'a>, Box<RqliteError>> {
        let mut cursor = self.cursor();
        cursor.execute(query, params).await?;
        Ok(cursor)
    }

    pub(crate) async fn request(
        &mut self,
        req_builder: hyper::http::request::Builder,
        body: Option<&serde_json::Value>,
    ) -> Result<hyper::Response<Body>, Box<RqliteError>> {
        let req = match req_builder.body(if body.is_some() {
            Body::from(match serde_json::to_string(body.unwrap()) {
                Ok(v) => v,
                Err(e) => {
                    println!("{}", e);
                    return Err(Box::new(RqliteError::DataSer(e.to_string())));
                }
            })
        } else {
            Body::empty()
        }) {
            Ok(v) => v,
            Err(e) => return Err(Box::new(RqliteError::DataSer(e.to_string()))),
        };
        let resp = match self.request_sender.send_request(req).await {
            Ok(v) => v,
            Err(e) => return Err(Box::new(RqliteError::Connection(e.to_string()))),
        };
        self.check_auth(resp.status().as_u16())?;
        Ok(resp)
    }

    pub(crate) async fn read_body(
        &self,
        resp: hyper::Response<Body>,
    ) -> Result<bytes::Bytes, Box<RqliteError>> {
        match hyper::body::to_bytes(resp.into_body()).await {
            Ok(v) => Ok(v),
            Err(e) => return Err(Box::new(RqliteError::Connection(e.to_string()))),
        }
    }

    pub(crate) async fn body<'a, T: serde::de::Deserialize<'a>>(
        &self,
        slice: &'a [u8],
    ) -> Result<T, Box<RqliteError>> {
        Ok(match serde_json::from_slice(slice) {
            Ok(v) => v,
            Err(e) => return Err(Box::new(RqliteError::DataSer(e.to_string()))),
        })
    }

    pub(crate) fn base_headers(
        &self,
        req_builder: hyper::http::request::Builder,
    ) -> hyper::http::request::Builder {
        req_builder
            .header(
                "Host",
                format!("{}:{}", self.settings.host, self.settings.port),
            )
            .header("Content-Type", "application/json")
    }

    pub(crate) fn check_auth(&self, status_code: u16) -> Result<(), Box<RqliteError>> {
        if status_code == 401 {
            return Err(Box::new(RqliteError::AuthError));
        }
        Ok(())
    }

    pub(crate) fn auth(
        &self,
        mut req_builder: hyper::http::request::Builder,
    ) -> hyper::http::request::Builder {
        if self.settings.user.is_some() && self.settings.pass.is_some() {
            req_builder = req_builder.header(
                "Authorization",
                format!(
                    "Basic {}",
                    URL_SAFE_NO_PAD.encode(format!(
                        "{}:{}",
                        self.settings.user.clone().unwrap(),
                        self.settings.pass.clone().unwrap()
                    ))
                ),
            );
        }
        req_builder
    }

    /// List all node in cluster.
    ///
    /// bool show_nonvoters to show non voting nodes too
    /// ```
    /// use rqlite::{ConnectOptions, Node};
    ///
    /// let mut conn = ConnectOptions::new("127.0.0.1", 4001)
    ///		.connect().await?;
    /// let nodes: Vec<Node> = conn.nodes(false).await?;
    /// println("{:?}", nodes);
    /// // [Node { id: "1", api_addr: "http://127.0.0.1:4001", addr: "127.0.0.1:4002",
    /// // reachable: true, leader: true, time: 0.000037026 },
    /// // Node { id: "2", api_addr: "http://127.0.0.1:4003", addr: "127.0.0.1:4004",
    /// // reachable: true, leader: false, time: 0.000073143 },
    /// // Node { id: "3", api_addr: "http://127.0.0.1:4005", addr: "127.0.0.1:4006",
    /// // reachable: true, leader: false, time: 0.000043848 }]
    /// ```
    ///
    /// Returns RqliteError on error to handle exception explicitly
    pub async fn nodes(&mut self, show_nonvoters: bool) -> Result<Vec<Node>, Box<RqliteError>> {
        let mut req_builder = Request::builder().method("GET").uri(if show_nonvoters {
            "/nodes?nonvoters"
        } else {
            "/nodes"
        });
        req_builder = self.auth(self.base_headers(req_builder));
        let resp = self.request(req_builder, None).await?;

        let body = self.read_body(resp).await?;
        let json: serde_json::Value = self.body(&body).await?;
        if json.is_object() {
            let json_map = json.as_object();
            let mut node_vec = Vec::new();
            let mut iter = json_map.iter();
            let mut opt_node;
            let mut de_node: _Node;
            loop {
                opt_node = iter.next();
                if opt_node.is_none() {
                    break;
                }
                let key_val: Vec<(&String, &serde_json::Value)> =
                    opt_node.unwrap().iter().collect();
                for (key, val) in key_val {
                    de_node = match serde_json::from_value(val.to_owned()) {
                        Ok(v) => v,
                        Err(e) => return Err(Box::new(RqliteError::DataSer(e.to_string()))),
                    };
                    node_vec.push(Node {
                        id: key.to_owned(),
                        api_addr: de_node.api_addr,
                        addr: de_node.addr,
                        reachable: de_node.reachable,
                        leader: de_node.leader,
                        time: de_node.time,
                    });
                }
            }
            return Ok(node_vec);
        }
        return Err(Box::new(RqliteError::SqlError(
            "Error deserializing json body".to_owned(),
        )));
    }

    /// Check if node is ready to respond to database requests and cluster management operations
    /// ```
    /// let mut conn = ConnectOptions::new("127.0.0.1", 4001)
    ///		.connect().await?;
    /// if conn.ready().await? {
    ///     println!("127.0.0.1:4001 is ready to receive requests");
    /// }
    /// ```
    ///
    /// Returns RqliteError on error to handle exception explicitly
    pub async fn ready(&mut self) -> Result<bool, Box<RqliteError>> {
        let mut req_builder = Request::builder().method("GET").uri("/readyz");
        req_builder = self.auth(self.base_headers(req_builder));
        let resp = self.request(req_builder, None).await?;
        if resp.status().as_u16() != 200 {
            return Ok(false);
        }
        Ok(true)
    }

    /// Remove node from cluster.
    ///
    /// Cluster must still be functional (can be verified using [`Connection::ready()`]).
    ///
    /// This can cause a cluster failure if removed node is the last tolerated failure.
    /// ```
    /// let mut conn = ConnectOptions::new("127.0.0.1", 4001)
    ///		.connect().await?;
    /// if conn.ready().await? {
    ///     if conn.remove("num5").await? {
    ///         println("num5 removed from cluster");
    ///     }
    /// }
    /// ```
    ///
    /// Returns RqliteError on error to handle exception explicitly
    pub async fn remove(&mut self, id: &str) -> Result<bool, Box<RqliteError>> {
        let mut req_builder = Request::builder().method("DELETE").uri("/remove");
        req_builder = self.auth(self.base_headers(req_builder));
        let resp = self.request(req_builder, Some(&json!({"id": id}))).await?;
        if resp.status().as_u16() != 200 {
            return Ok(false);
        }
        Ok(true)
    }
}
