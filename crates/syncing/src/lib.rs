mod broker;
mod config;
mod context;
mod discovery;
mod error;
mod lrnodepool;
mod message;
mod transfer;

pub const MAX_CONNECTED_NODES: usize = 8;
pub mod bootstrap;

use crate::bootstrap::node_bootstrap_syncing_context_start;

pub async fn node_syncing() {
    let offset_localstate_file_as_param: u64 = 100;

    tokio::spawn(async move {
        node_bootstrap_syncing_context_start(offset_localstate_file_as_param).await;
    });
}
