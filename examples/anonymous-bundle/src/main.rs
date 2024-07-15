mod rustls_dangerous_client_verifier;

use base64ct::{Base64, Encoding};
use chrono::{DateTime, Utc};
use hyper_util::rt::TokioIo;
use pem_rfc7468::LineEnding;
use rustls_dangerous_client_verifier::make_unsafe_rustls_client_config;
use spire_api::spire::api::server::bundle::v1::{bundle_client::BundleClient, GetBundleRequest};
use tokio::net::TcpStream;
use tokio_rustls::{rustls::pki_types::ServerName, TlsConnector};
use tonic::transport::{Channel, Uri};
use tower::service_fn;

async fn amain() {
    let connector = TlsConnector::from(make_unsafe_rustls_client_config());
    let connection = Channel::from_static("https://localhost:8081")
        .connect_with_connector(service_fn(move |uri: Uri| {
            let connector = connector.clone();
            async move {
                connector
                    .connect(
                        ServerName::DnsName(uri.host().unwrap().to_owned().try_into().unwrap()),
                        TcpStream::connect(uri.authority().unwrap().to_string()).await?,
                    )
                    .await
                    .map(TokioIo::new)
            }
        }))
        .await
        .unwrap();

    let mut client = BundleClient::new(connection);

    let bundle = client
        .get_bundle(GetBundleRequest { output_mask: None })
        .await
        .unwrap()
        .into_inner();

    println!("trust domain: {}", bundle.trust_domain);
    println!("refresh hint: {}", bundle.refresh_hint);
    println!("sequence number: {}", bundle.sequence_number);

    // 1. X.509 authorities
    println!("\n==== X.509 authorities ====");
    for cert in bundle.x509_authorities.into_iter() {
        let cert_pem =
            pem_rfc7468::encode_string("CERTIFICATE", LineEnding::LF, &cert.asn1).unwrap();
        println!("tainted: {}", cert.tainted);
        println!("certificate:\n  {}", cert_pem.replace('\n', "\n  "));
    }

    // 2. print JWT authorities
    println!("==== JWT authorities ====");
    for jwt in bundle.jwt_authorities.into_iter() {
        println!("tainted: {}", jwt.tainted);
        println!("key id: {}", jwt.key_id);
        println!(
            "expired at: {}",
            DateTime::<Utc>::from_timestamp(jwt.expires_at, 0).unwrap()
        );
        println!(
            "public key:\n  {}\n",
            Base64::encode_string(&jwt.public_key)
        );
    }
}

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(amain());
}
