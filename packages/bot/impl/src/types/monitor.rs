use std::borrow::Cow;
use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use oc_bots_sdk::types::Chat;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, CandidType, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct MonitorId(pub Chat);

impl From<Chat> for MonitorId {
    fn from(
        value: Chat
    ) -> Self {
        Self(value)
    }
}

impl Ord for MonitorId {
    fn cmp(
        &self, 
        other: &Self
    ) -> std::cmp::Ordering {
        match self.0.canister_id().cmp(&other.0.canister_id()) {
            std::cmp::Ordering::Equal => {
                self.0.channel_id().cmp(&other.0.channel_id())
            },
            other => other,
        }
    }
}

impl PartialOrd for MonitorId {
    fn partial_cmp(
        &self, 
        other: &Self
    ) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Storable for MonitorId {
    fn to_bytes(
        &self
    ) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(
        bytes: Cow<[u8]>
    ) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(Serialize, Deserialize, CandidType)]
pub enum MonitorState {
    Idle,
    Running
}

#[derive(Serialize, Deserialize, CandidType)]
pub struct Monitor {
    pub chat: Chat,
    pub state: MonitorState,
    pub canister_id: Principal,
}

impl Storable for Monitor {
    fn to_bytes(
        &self
    ) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(
        bytes: Cow<[u8]>
    ) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}