use std::{error::Error, fs::read, path::Path, time::Duration};

use reqwest::{Certificate, Client, Identity, IntoUrl, StatusCode};

use crate::{endpoints::ack::AckRequest, types::ErrorResponse};

#[derive(Clone)]
pub struct Etsi020Client {
    client: Client,
}

impl Etsi020Client {
    pub fn new(
        root_path: impl AsRef<Path>,
        public_cert_path: impl AsRef<Path>,
        private_key_path: impl AsRef<Path>,
    ) -> Result<Self, Box<dyn Error>> {
        let root_cert = Certificate::from_pem(&read(&root_path)?)?;
        let client_cert = read(&public_cert_path)?;
        let client_key = read(&private_key_path)?;
        let identity = Identity::from_pem(&[client_cert, client_key].concat())?;

        let client = reqwest::Client::builder()
            .tls_certs_only([root_cert])
            .identity(identity)
            .timeout(Duration::from_secs(10))
            .build()?;

        Ok(Self { client })
    }

    pub async fn ack(&self, url: impl IntoUrl, body: AckRequest) -> Result<(), Box<dyn Error>> {
        let ack_url_response = self
            .client
            .post(url)
            .json(&body)
            .send()
            .await
            .inspect_err(|e| println!("Failed to send 'ack' request. Error: {:?}", e))?;

        let status = ack_url_response.status();
        if status != StatusCode::OK {
            println!("ACK url call ERR. Status: {:?}", ack_url_response.status());

            let err_body = ack_url_response
                .json::<ErrorResponse>()
                .await
                .inspect_err(|e| println!("Could not parse ACK response. Error: {}, {:?}", e, e))?;

            println!("ACK response body:\n\t{:#?}", err_body);
        } else {
            println!("ACK url call OK.");
        }

        Ok(())
    }
}
