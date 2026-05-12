use std::{error::Error, fs::read, path::Path, time::Duration};

use reqwest::{Certificate, Client, Identity, IntoUrl, Response, StatusCode};

use crate::types::{ack::AckRequest, ext_keys::ExtKeysRequest, ErrorResponse};

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

    pub async fn ext_keys_async(
        &self,
        url: impl IntoUrl,
        body: ExtKeysRequest,
    ) -> Result<(), Box<dyn Error>> {
        let ext_keys_url_response = self
            .client
            .post(url)
            .json(&body)
            .send()
            .await
            .inspect_err(|e| println!("Failed to send 'ext_keys' request. Error: {:?}", e))?;

        Self::process_response_no_body(ext_keys_url_response, StatusCode::ACCEPTED)
            .await
            .inspect(|_| println!("EXT_KEYS url call OK."))
            .inspect_err(|e| println!("EXT_KEYS error processing response. Error: {e}"))
    }

    pub async fn ack(&self, url: impl IntoUrl, body: AckRequest) -> Result<(), Box<dyn Error>> {
        let ack_url_response = self
            .client
            .post(url)
            .json(&body)
            .send()
            .await
            .inspect_err(|e| println!("Failed to send 'ack' request. Error: {:?}", e))?;

        Self::process_response_no_body(ack_url_response, StatusCode::OK)
            .await
            .inspect(|_| println!("ACK url call OK."))
            .inspect_err(|e| println!("ACK error processing response. Error: {e}"))
    }

    async fn process_response_no_body(
        response: Response,
        expected_status: StatusCode,
    ) -> Result<(), Box<dyn Error>> {
        let status = response.status();

        if status != expected_status {
            println!("Response status error. Expected {expected_status}, received {status}");

            let err_body = response.json::<ErrorResponse>().await.inspect_err(|e| {
                println!("Could not parse response JSON body. Error: {} ({:?})", e, e)
            })?;

            Err(Box::new(err_body))
        } else {
            Ok(())
        }
    }
}
