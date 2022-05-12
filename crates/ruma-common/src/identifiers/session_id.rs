//! Matrix session ID.

use ruma_macros::IdZst;

/// A session ID.
///
/// Session IDs in Matrix are opaque character sequences of `[0-9a-zA-Z.=_-]`. Their length must
/// must not exceed 255 characters.
#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, IdZst)]
#[ruma_id(validate = ruma_identifiers_validation::session_id::validate)]
pub struct SessionId(str);
