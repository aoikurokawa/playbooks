pub mod item;
pub mod tag;
pub mod user;

pub use item::Item;
pub use tag::*;
pub use user::*;

pub const BASE_URL: &str = "https://qiita.com/api/v2";

pub struct QiitaClient {}

impl QiitaClient {
    // GET /api/v2/authenticated_user/items
    //
    // List the authenticated user's items in newest order
    //
    //     page
    //         Page number (from 1 to 100)
    //         Example: 1
    //         Type: string
    //         Pattern: /^[0-9]+$/
    //     per_page
    //         Records count per page (from 1 to 100)
    //         Example: 20
    //         Type: string
    //         Pattern: /^[0-9]+$/
    pub async fn authenticated_user_items(page: u8, per_page: u8) -> reqwest::Result<Vec<Item>> {
        let url = format!("{BASE_URL}/mkt_hanada/items?page={page}?per_page={per_page}");
        // let res = reqwest::get(url).await?.json::<Vec<Item>>().await?;
        let res = reqwest::get(url).await?;

        let json = res.text().await?;

        println!("{:?}", json);

        Ok(Vec::new())
    }

    // GET /api/v2/items
    //
    // List items.
    //
    //     page
    //         Page number (from 1 to 100)
    //         Example: 1
    //         Type: string
    //         Pattern: /^[0-9]+$/
    //     per_page
    //         Records count per page (from 1 to 100)
    //         Example: 20
    //         Type: string
    //         Pattern: /^[0-9]+$/
    //     query
    //         Search query
    //         Example: "qiita user:Qiita"
    //         Type: string
    pub async fn items(page: u8, per_page: u8) -> reqwest::Result<Vec<Item>> {
        let url = format!("{BASE_URL}/items");
        let res = reqwest::get(url).await?.json::<Vec<Item>>().await?;
        // let res = reqwest::get(url).await?;

        // let json = res.text().await?;

        // println!("{:?}", json);

        Ok(res)
    }

    // GET /api/v2/items/:item_id
    //
    // Get an item.
    pub async fn items_by_item_id(item_id: &str) -> reqwest::Result<Item> {
        let url = format!("{BASE_URL}/items/{item_id}");
        let res = reqwest::get(url).await?.json::<Item>().await?;
        // let res = reqwest::get(url).await?;

        // let json = res.text().await?;

        // println!("{:?}", json);

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::QiitaClient;

    #[tokio::test]
    async fn test_authenticated_user_items() {
        let items = QiitaClient::authenticated_user_items(1, 1)
            .await
            .expect("get items for authenticated user");

        println!("{items:?}");
    }

    #[tokio::test]
    async fn test_items() {
        let items = QiitaClient::items(1, 20).await.expect("get items");

        println!("{items:?}");
    }

    #[tokio::test]
    async fn test_items_by_item_id() {
        let item_id = "6a121a6027788cd97725";
        let items = QiitaClient::items_by_item_id(item_id)
            .await
            .expect("get items");

        println!("{items:?}");
    }
}
