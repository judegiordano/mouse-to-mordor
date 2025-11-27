use anyhow::Result;
use webrtc::{
    api::{media_engine::MediaEngine, APIBuilder},
    ice_transport::ice_server::RTCIceServer,
    interceptor::registry::Registry,
    peer_connection::{
        configuration::RTCConfiguration, peer_connection_state::RTCPeerConnectionState,
        sdp::session_description::RTCSessionDescription, RTCPeerConnection,
    },
};

const GOOGLE_STUN_SERVER: &str = "stun:stun.l.google.com:19302";
const CLOUDFLARE_STUN_SERVER: &str = "stun:stun.cloudflare.com:3478";

pub struct Rtc {
    pub peer_connection: RTCPeerConnection,
}

impl Rtc {
    pub async fn new() -> Result<Self> {
        let registry = Registry::new();
        let media_engine = MediaEngine::default();

        let api = APIBuilder::new()
            .with_interceptor_registry(registry)
            .with_media_engine(media_engine)
            .build();

        let config = RTCConfiguration {
            ice_servers: vec![RTCIceServer {
                urls: vec![
                    GOOGLE_STUN_SERVER.to_string(),
                    CLOUDFLARE_STUN_SERVER.to_string(),
                ],
                ..Default::default()
            }],
            ..Default::default()
        };

        let peer_connection = api.new_peer_connection(config).await?;

        peer_connection.on_peer_connection_state_change(Box::new(|s: RTCPeerConnectionState| {
            Box::pin(async move {
                println!("PeerConnection State has changed: {}", s);
            })
        }));

        Ok(Self { peer_connection })
    }
}

pub async fn build_local_offer(
    peer_connection: &RTCPeerConnection,
) -> Result<Option<RTCSessionDescription>> {
    let offer = peer_connection.create_offer(None).await?;
    peer_connection.set_local_description(offer.clone()).await?;
    //
    let mut gather_complete = peer_connection.gathering_complete_promise().await;
    gather_complete.recv().await;

    Ok(peer_connection.local_description().await)
}

pub async fn accept_remote_offer(
    peer_connection: &RTCPeerConnection,
    remote_description: RTCSessionDescription,
) -> Result<Option<RTCSessionDescription>> {
    peer_connection
        .set_remote_description(remote_description)
        .await?;
    let answer = peer_connection.create_answer(None).await?;
    peer_connection
        .set_local_description(answer.clone())
        .await?;

    let mut gather_complete = peer_connection.gathering_complete_promise().await;
    gather_complete.recv().await;

    Ok(peer_connection.local_description().await)
}
