use serde::{Deserialize, Serialize};

use crate::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Tag {
    /// {"name"=>"Ruby", "versions"=>["0.0.1"]}
    name: String,
    versions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Group {
    created_at: String,
    description: String,
    name: String,
    private: bool,
    updated_at: String,
    url_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TeamMemberShip {
    name: String,
}

/// Represents an item posted from a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// Item body in HTML
    /// Example: "<h1>Example</h1>"
    /// Type: string
    rendered_body: String,

    /// Item body in Markdown
    /// Example: "# Example"
    /// Type: string
    body: String,

    /// A flag whether this item is co-edit mode (only available on Qiita Team)
    /// Example: false
    /// Type: boolean
    coediting: bool,

    /// Comments count
    /// Example: 100
    /// Type: integer
    comments_count: u64,

    // Date-time when this data was created
    // Example: "2000-01-01T00:00:00+00:00"
    // Type: string
    // Format: date-time
    created_at: String,

    /// Represents a group on Qiita Team
    group: Option<Group>,

    /// An unique item ID
    /// Example: "c686397e4a0f4f11683d"
    /// Type: string
    /// Pattern: /^[0-9a-f]{20}$/
    id: String,

    /// Likes count (only available on Qiita)
    /// Example: 100
    /// Type: integer
    likes_count: u64,

    /// A flag whether this item is private (only available on Qiita)
    /// Example: false
    /// Type: boolean
    private: bool,

    /// Emoji reactions count (only availabble on Qiita Team)
    /// Example: 100
    /// Type: integer
    reactions_count: u64,

    /// Stocks count
    /// Example: 100
    /// Type: integer
    stocks_count: u64,

    /// A list of tags
    /// Example: [{"name"=>"Ruby", "versions"=>["0.0.1"]}]
    /// Type: array
    tags: Vec<Tag>,

    /// The title of this item
    /// Example: "Example title"
    /// Type: string
    title: String,

    /// Date-time when this data was updated
    /// Example: "2000-01-01T00:00:00+00:00"
    /// Type: string
    /// Format: date-time
    updated_at: String,

    /// The URL of this item
    /// Example: "https://qiita.com/Qiita/items/c686397e4a0f4f11683d"
    /// Type: string
    url: String,

    /// A Qiita user (a.k.a. account)
    user: User,

    // The number of views.
    // Example: 100
    // Type: null, integer
    page_views_count: Option<u64>,

    /// A Qiita Team member.  
    team_membership: Option<TeamMemberShip>,

    /// Represents url_name of Organization of this item.
    /// Example: "qiita-inc"
    /// Type: string, null     
    organization_url_name: Option<String>,

    /// A flag whether this item has slide mode enabled
    /// Example: false
    /// Type: boolean
    slide: bool,
}
