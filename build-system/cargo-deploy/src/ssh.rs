use std::net::IpAddr;

use anyhow::{bail, Result};
use async_trait::async_trait;
use russh::{
    client::{connect, Handle, Handler},
    ChannelMsg,
};
use russh_keys::key;
use russh_sftp::client::SftpSession;
use tokio::io::AsyncWriteExt;

struct Client {}

#[async_trait]
impl Handler for Client {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &key::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

pub struct Session {
    session: Handle<Client>,
}

impl Session {
    pub async fn connect(addr: IpAddr) -> Result<Self> {
        let mut session = connect(Default::default(), (addr, 22), Client {}).await?;

        if !session.authenticate_none("admin").await? {
            bail!("Could not authenticate");
        }

        Ok(Self { session })
    }

    pub async fn call(&self, command: impl Into<Vec<u8>>) -> Result<Vec<u8>> {
        let mut channel = self.session.channel_open_session().await?;

        channel.exec(true, command).await?;

        let mut result = vec![];

        loop {
            match channel.wait().await {
                None => break,
                Some(ChannelMsg::Data { ref data }) => {
                    result.write_all(data).await?;
                }
                Some(ChannelMsg::ExitStatus { exit_status }) => {
                    if exit_status != 0 {
                        bail!("Error with stats: {}", exit_status);
                    }
                }
                _ => {}
            }
        }

        Ok(result)
    }

    pub async fn sftp(&self) -> Result<SftpSession> {
        let channel = self.session.channel_open_session().await?;
        channel.request_subsystem(true, "sftp").await?;
        Ok(SftpSession::new(channel.into_stream()).await?)
    }
}
