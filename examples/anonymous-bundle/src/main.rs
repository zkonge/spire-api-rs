mod rustls_dangerous_client_verifier;

use std::{env::args, net::SocketAddr};

use chrono::{DateTime, Utc};
use futures_util::TryFutureExt;
use hyper_util::rt::TokioIo;
use pem_rfc7468::{self as pem, LineEnding};
use spire_api::spire::api::server::bundle::v1::{GetBundleRequest, bundle_client::BundleClient};
use tokio::net::TcpStream;
use tokio_rustls::{TlsConnector, rustls::pki_types::ServerName};
use tonic::transport::{Channel, Uri};
use tower::service_fn;

use crate::rustls_dangerous_client_verifier::make_unsafe_rustls_client_config;

async fn new_channel(server_addr: SocketAddr) -> Channel {
    let connector = TlsConnector::from(make_unsafe_rustls_client_config());
    let server_name = ServerName::IpAddress(server_addr.ip().into()).to_owned();
    let connector = service_fn(move |_: Uri| {
        let connector = connector.clone();
        let server_name = server_name.clone();

        TcpStream::connect(server_addr)
            .and_then(move |s| connector.connect(server_name, s))
            .map_ok(TokioIo::new)
    });

    Channel::from_static("http://[::1]")
        .connect_with_connector(connector)
        .await
        .unwrap()
}

fn format_cert(der: &[u8]) -> String {
    pem::encode_string("CERTIFICATE", LineEnding::LF, der).unwrap()
}

fn format_spki(der: &[u8]) -> String {
    pem::encode_string("PUBLIC KEY", LineEnding::LF, der).unwrap()
}

async fn amain() {
    // cargo run -- [::1]:8081
    let server_addr: SocketAddr = args()
        .skip(1)
        .next()
        .expect("specify a server addr first")
        .parse()
        .unwrap();

    let channel = new_channel(server_addr).await;
    let mut client = BundleClient::new(channel);

    let bundle = client
        .get_bundle(GetBundleRequest::default())
        .await
        .unwrap()
        .into_inner();

    println!("trust domain: {}", bundle.trust_domain);
    println!("refresh hint: {}", bundle.refresh_hint);
    println!("sequence number: {}", bundle.sequence_number);

    // 1. X.509 authorities
    println!("==== X.509 authorities ====");
    for cert in bundle.x509_authorities.into_iter() {
        println!("tainted: {}", cert.tainted);
        println!("certificate:\n{}", format_cert(&cert.asn1));
    }

    // 2. print JWT authorities
    println!("==== JWT authorities ====");
    for jwt in bundle.jwt_authorities.into_iter() {
        println!("tainted: {}", jwt.tainted);
        println!("key id: {}", jwt.key_id);
        println!(
            "expires at: {}",
            DateTime::<Utc>::from_timestamp(jwt.expires_at, 0).unwrap()
        );
        println!("public key:\n{}", format_spki(&jwt.public_key));
    }
}

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(amain());
}
