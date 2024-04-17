use serde::{Deserialize, Serialize};

/// A tag attached to an item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// Tag name
    /// Example: "qiita"
    /// Type: string
    id: String,

    /// Followes count
    /// Example: 100
    /// Type: integer
    followers_count: u64,

    /// Tag Icon URL
    /// Example: "https://s3-ap-northeast-1.amazonaws.com/qiita-tag-image/9de6a11d330f5694820082438f88ccf4a1b289b2/medium.jpg"
    /// Type: null, string
    icon_url: String,

    /// Items count
    /// Example: 200
    /// Type: integer
    items_count: u64,
}
