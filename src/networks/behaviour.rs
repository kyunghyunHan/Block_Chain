use anyhow::Result;
use libp2p::{
    gossipsub::{Gossipsub, GossipsubConfig, GossipsubEvent, MessageAuthenticity},
    identity::Keypair,
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour,
};
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::Messages;
/*
gossipsub: 메시지 전파를 위해 Gossip 프로토콜을 사용합니다.
mdns: 노드 검색
msg_sender: 다른 노드로부터 요청 메시지를 받은 후 채널로 보냅니다.
*/
#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
pub struct BlockchainBehaviour {
    pub gossipsub: Gossipsub,
    pub mdns: Mdns,
    #[behaviour(ignore)]
    pub msg_sender: mpsc::UnboundedSender<Messages>,
}

impl BlockchainBehaviour {
    pub async fn new(
        key_pair: Keypair,
        config: GossipsubConfig,
        msg_sender: mpsc::UnboundedSender<Messages>,
    ) -> Result<Self> {
        Ok(Self {
            gossipsub: Gossipsub::new(MessageAuthenticity::Signed(key_pair), config).unwrap(),
            mdns: Mdns::new(Default::default()).await?,
            msg_sender,
        })
    }
}

impl NetworkBehaviourEventProcess<GossipsubEvent> for BlockchainBehaviour {
    fn inject_event(&mut self, event: GossipsubEvent) {
        match event {
            GossipsubEvent::Message {
                propagation_source: peer_id,
                message_id: id,
                message,
            } => {
                info!("Got message with id: {} from peer: {:?}", id, peer_id);
                let msg: Messages = serde_json::from_slice(&message.data).unwrap();
                if let Err(e) = self.msg_sender.send(msg) {
                    error!("error sending messages via channel, {}", e);
                }
            }
            GossipsubEvent::Subscribed { peer_id, topic } => {
                info!("Subscribe topic: {} with id: {}", topic, peer_id);
            }
            GossipsubEvent::Unsubscribed { peer_id, topic } => {
                info!("Unsubscribe topic: {} with id: {}", topic, peer_id);
            }
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for BlockchainBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (id, addr) in list {
                    println!("Got peer: {} with addr {}", &id, &addr);
                    self.gossipsub.add_explicit_peer(&id);
                }
            }
            MdnsEvent::Expired(list) => {
                for (id, addr) in list {
                    println!("Removed peer: {} with addr {}", &id, &addr);
                    self.gossipsub.remove_explicit_peer(&id);
                }
            }
        }
    }
}
