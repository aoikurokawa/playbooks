use serde::{Deserialize, Serialize};

/// A Qiita user (a.k.a. account)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// self-description
    /// Example: "Hello, world."
    /// Type: null, string
    description: Option<String>,

    /// Facebook ID
    /// Example: "qiita"
    /// Type: null, string
    facebook_id: Option<String>,

    /// Followees count
    /// Example: 100
    /// Type: integer
    followees_count: u64,

    /// Followers count
    /// Example: 200
    /// Type: integer
    followers_count: u64,

    /// GitHub ID
    /// Example: "qiitan"
    /// Type: null, string
    github_login_name: Option<String>,

    /// User ID
    /// Example: "qiita"
    /// Type: string
    id: String,

    /// How many items a user posted on qiita.com (Items on Qiita Team are not included)
    /// Example: 300
    /// Type: integer
    items_count: u64,

    /// LinkedIn ID
    /// Example: "qiita"
    /// Type: null, string
    linkedin_id: Option<String>,

    /// Location
    /// Example: "Tokyo, Japan"
    /// Type: null, string
    location: Option<String>,

    /// Customized user name
    /// Example: "Qiita キータ"
    /// Type: null, string
    name: Option<String>,

    /// Organization which a user belongs to
    /// Example: "Qiita Inc."
    /// Type: null, string
    organization: Option<String>,

    /// Unique integer ID
    /// Example: 1
    /// Type: integer
    permanent_id: u64,

    /// Profile image URL
    /// Example: "https://s3-ap-northeast-1.amazonaws.com/qiita-image-store/0/88/ccf90b557a406157dbb9d2d7e543dae384dbb561/large.png?1575443439"
    /// Type: string
    profile_image_url: String,

    /// A flag whether this user is configured as team-only
    /// Example: false
    /// Type: boolean
    team_only: bool,

    /// Twitter screen name
    /// Example: "qiita"
    /// Type: null, string
    twitter_screen_name: Option<String>,

    /// Website URL
    /// Example: "https://qiita.com"
    /// Type: null, string
    website_url: Option<String>,
}
