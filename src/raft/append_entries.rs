use std::sync::Arc;

use super::{
    node::Node,
    proto::{append_entries_server::AppendEntries, AppendEntriesRequest, AppendEntriesResponse},
};

#[tonic::async_trait]
impl AppendEntries for Node {
    async fn reply(
        &self,
        req: tonic::Request<AppendEntriesRequest>,
    ) -> Result<tonic::Response<AppendEntriesResponse>, tonic::Status> {
        let mtx = Arc::clone(&self.state);
        let log = req.get_ref();

        let mut node_state = mtx.lock().await;

        let mut resp = AppendEntriesResponse {
            term: node_state.persistent_state.current_term,
            success: false,
        };

        // 1. Reply false if term < currentTerm (§5.1)
        if log.term < node_state.persistent_state.current_term {
            return Ok(tonic::Response::new(resp));
        }
        /* 2. Reply false if log doesn’t contain an entry at prevLogIndex
        whose term matches prevLogTerm (§5.3) */

        /* 3. If an existing entry conflicts with a new one (same index
        but different terms), delete the existing entry and all that
        follow it (§5.3) */

        // 4. Append any new entries not already in the log

        /* 5. If leaderCommit > commitIndex, set commitIndex =
        min(leaderCommit, index of last new entry) */
        if log.leader_commit_index > node_state.volatile_member_state.commit_index {
            node_state.volatile_member_state.commit_index =
                std::cmp::min(log.leader_commit_index, 0);
        }

        Ok(tonic::Response::new(resp))
    }
}
