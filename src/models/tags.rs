use super::tag::Tag;
use octocrab::{Octocrab, Result};

#[async_trait::async_trait]
pub trait Tags {
    /// Gets all tags for a given repo
    async fn get_tags(&self, owner: String, repo: String) -> Result<Vec<Tag>>;
}

#[async_trait::async_trait]
impl Tags for Octocrab {
    /// Implements trait: Gets all tags for a given repo
    async fn get_tags(&self, owner: String, repo: String) -> Result<Vec<Tag>> {
        let url = format!("/repos/{0}/{1}/tags", owner, repo);

        self.get(url, None::<&()>).await
    }
}
