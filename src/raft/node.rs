use std::{net::IpAddr, sync::Arc};
use tokio::sync::Mutex;

use super::state::NodeState;
use anyhow::Result;

pub struct Node {
    pub id: uuid::Uuid,
    pub ip: IpAddr,
    pub state: Arc<Mutex<NodeState>>,
}

impl Node {
    pub async fn persist_on_stable_storage(&self) -> Result<()> {
        Ok(())
    }
}
