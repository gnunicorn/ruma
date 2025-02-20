//! Modules for events in the `m.call` namespace.
//!
//! This module also contains types shared by events in its child namespaces.

use serde::{Deserialize, Serialize};

use crate::{serde::StringEnum, PrivOwnedStr};

pub mod answer;
pub mod candidates;
pub mod hangup;
pub mod invite;

/// A VoIP session description.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct SessionDescription {
    /// The type of session description.
    #[serde(rename = "type")]
    pub session_type: SessionDescriptionType,

    /// The SDP text of the session description.
    pub sdp: String,
}

impl SessionDescription {
    /// Creates a new `SessionDescription` with the given session type and SDP text.
    pub fn new(session_type: SessionDescriptionType, sdp: String) -> Self {
        Self { session_type, sdp }
    }
}

/// The type of VoIP session description.
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/doc/string_enum.md"))]
#[derive(Clone, Debug, PartialEq, Eq, StringEnum)]
#[ruma_enum(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SessionDescriptionType {
    /// An answer.
    Answer,

    /// An offer.
    Offer,

    #[doc(hidden)]
    _Custom(PrivOwnedStr),
}

impl SessionDescriptionType {
    /// Creates a string slice from this `SessionDescriptionType`.
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}
