#[derive(Default)]
pub struct PersistentState {
    pub current_term: u64,
    pub voted_for: Option<String>,
    pub log: Vec<Vec<bytes::Bytes>>,
}

#[derive(Default)]
pub struct VolatileMemberState {
    pub commit_index: u64,
    pub last_applied: u64,
}

#[derive(Default)]
pub struct VolatileLeaderState {
    pub members_next_index: Vec<u64>,
    pub match_index: Vec<u64>,
}

pub struct NodeState {
    pub persistent_state: PersistentState,
    pub volatile_member_state: VolatileMemberState,
    pub volatile_leader_state: Option<VolatileLeaderState>,
}
