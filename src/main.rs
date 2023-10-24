#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Rustls with native certs
    // let tls = {
    //     use rustls::{Certificate, RootCertStore};

    //     let mut root_store = RootCertStore::empty();
    //     for cert in rustls_native_certs::load_native_certs()? {
    //         root_store.add(&Certificate(cert.0))?;
    //     }
    //     let config = rustls::ClientConfig::builder()
    //         .with_safe_defaults()
    //         .with_root_certificates(rustls::RootCertStore::empty())
    //         .with_no_client_auth();

    //     tokio_postgres_rustls::MakeRustlsConnect::new(config)
    // };

    // Openssl
    let tls = {
        use openssl::ssl::{SslConnector, SslMethod};
        use postgres_openssl::MakeTlsConnector;

        let builder = SslConnector::builder(SslMethod::tls_client())?;
        MakeTlsConnector::new(builder.build())
    };

    let (client, connection) =
        tokio_postgres::connect("postgresql://postgres:../railway", tls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client.query("SELECT 1", &[]).await?;

    println!("rows: {:#?}", rows);

    Ok(())
}
