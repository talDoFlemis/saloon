use std::sync::Arc;

use super::{
    node,
    proto::{request_vote_server::RequestVote, RequestVoteRequest, RequestVoteResponse},
};

#[tonic::async_trait]
impl RequestVote for node::Node {
    async fn reply(
        &self,
        req: tonic::Request<RequestVoteRequest>,
    ) -> Result<tonic::Response<RequestVoteResponse>, tonic::Status> {
        let mtx = Arc::clone(&self.state);
        let input = req.get_ref();

        let mut node_state = mtx.lock().await;

        let mut resp = RequestVoteResponse {
            term: node_state.persistent_state.current_term,
            granted: false,
        };

        // (§5.1)
        if input.term < node_state.persistent_state.current_term {
            return Ok(tonic::Response::new(resp));
        }

        if node_state.persistent_state.voted_for.is_none()
            && input.last_log_index >= node_state.volatile_member_state.commit_index
        {
            resp.granted = true;
            node_state.persistent_state.voted_for = Some(input.candidate_id.clone());
        }

        if let Err(e) = self.persist_on_stable_storage().await {
            return Err(tonic::Status::internal(format!(
                "Failed to save state on node {}: {:?}",
                self.id, e
            )));
        }

        Ok(tonic::Response::new(resp))
    }
}
