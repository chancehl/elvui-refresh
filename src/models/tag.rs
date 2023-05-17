use serde::{Deserialize, Serialize};

use super::commit::Commit;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub name: String,
    pub zipball_url: String,
    pub tarball_url: String,
    pub node_id: String,
    pub commit: Commit,
}
