use std::io;
use std::net::SocketAddr;

use crypto::crypto_box::PublicKey;
use tezos_messages::p2p::encoding::ack::NackMotive;
use tezos_messages::p2p::encoding::version::NetworkVersion;

use crate::Port;

#[derive(Debug, Clone)]
pub struct PeerConnectionMessageReadInitAction {
    pub address: SocketAddr,
}

#[derive(Debug, Clone)]
pub struct PeerConnectionMessagePartReadAction {
    pub address: SocketAddr,
    // TODO: can be optimized by using `Cow<'a, [u8]>` instead to avoid allocation.
    /// Chunk bytes including chunk length bytes at the beginning.
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct PeerConnectionMessageReadErrorAction {
    pub address: SocketAddr,
    pub error: io::ErrorKind,
}

#[derive(Debug, Clone)]
pub struct PeerConnectionMessageReadSuccessAction {
    pub address: SocketAddr,
    pub port: Port,
    pub compatible_version: Result<NetworkVersion, NackMotive>,
    pub public_key: PublicKey,
}
