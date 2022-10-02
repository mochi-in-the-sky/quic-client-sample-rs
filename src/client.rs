use anyhow::Result;
use quinn::{ClientConfig, Endpoint, RecvStream, SendStream};
use tracing::*;

use super::setting::Setting;

pub struct Client {
    setting: Setting,
}

impl Client {
    pub fn new(setting: Setting) -> Self {
        Self { setting }
    }

    pub async fn listen(&self) -> Result<()> {
        let mut roots = rustls::RootCertStore::empty();
        for cert in self.setting.cert.clone() {
            roots.add(&cert)?;
        }
        let client_config = ClientConfig::with_root_certificates(roots);

        let mut endpoint = Endpoint::client("0.0.0.0:0".parse().unwrap())?;
        endpoint.set_default_client_config(client_config);

        let (sender, reciever) = endpoint
            .connect(self.setting.addr, "target")?
            .await?
            .connection
            .open_bi()
            .await?;

        let (ret_recv, ret_send) = tokio::join!(Self::recv(reciever), Self::send(sender));
        if let Err(e) = ret_recv {
            error!("catcher failed: {:?}", e);
        }
        if let Err(e) = ret_send {
            error!("pitcher failed: {:?}", e);
        }

        Ok(())
    }

    async fn recv(reciever: RecvStream) -> Result<()> {
        let received = reciever.read_to_end(10).await?;
        info!("catch: {:?}", received);
        Ok(())
    }

    async fn send(mut sender: SendStream) -> Result<()> {
        sender.write_all(b"Hello World").await?;
        sender.finish().await?;
        Ok(())
    }
}
