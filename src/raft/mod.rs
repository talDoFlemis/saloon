mod election;
mod config;
mod state;
mod request_votes;
mod append_entries;
mod node;

mod proto {
    tonic::include_proto!("raft");
}
