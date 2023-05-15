pub mod illust;
pub mod user;
pub mod novel;
pub mod traits;
pub mod utils;
pub mod error;
pub mod client;
pub mod preload;

pub use preload::*;
pub use client::auth::ClientBudiler;
pub use error::*;

#[cfg(test)]
mod tests {
    use std::fs::write;

    use reqwest::Proxy;

    use crate::{user::user, illust::{comments, self, illust::Illust}, novel, error, client::{auth, api::{Client}}};
    use crate::preload::*;

    #[test]
    fn relogin_test() {
        let token = "OOVwt1_ZrLTjlzKWmirNmbK_WuwOookfSGYTZnDRLcI";
        let mut h = auth::ClientBudiler::new();
        h.refresh_token(token).proxy(Proxy::all("http://localhost:15777").unwrap());
        let client = h.build().unwrap();
        let client_one = client.clone();
        println!("{} {}", client.access_token(), client_one.access_token());
        client.relogin();
        println!("{} {}", client.access_token(), client_one.access_token());
        client_one.relogin();
        println!("{} {}", client.access_token(), client_one.access_token());
    }

    fn api_test() {
        let token = "OOVwt1_ZrLTjlzKWmirNmbK_WuwOookfSGYTZnDRLcI";
        let mut h = auth::ClientBudiler::new();
        h.refresh_token(token).proxy(Proxy::all("http://localhost:15777").unwrap());
        let client = h.build().unwrap();
        println!("User Api Test Start");
        let userid = 1391461;
        println!("user_detail:");
        let v = client.user.detail(userid);
        println!("{:#?}", v);
        println!("user_follower:");
        let v = client.user.follower(userid, 0);
        println!("{:#?}", v);
        println!("user_following:");
        let v = client.user.following(userid, Restrict::Public, 0);
        println!("{:#?}", v);
        println!("user_illusts:");
        let v = client.user.illusts(userid, IllustType::Illust, 0);
        println!("{:#?}", v);
        println!("user_mypixiv:");
        let v = client.user.mypixiv(userid, 0);
        println!("{:#?}", v);
        println!("user_novels:");
        let v = client.user.novels(userid);
        println!("{:#?}", v);

        println!("illust Api Test Start");
        println!("illust_bookmark_detail:\n");
        let v = client.illust.bookmark_detail(107879571);
        println!("{:#?}", v);
        println!("illust_detail:");
        let v = client.illust.detail(107879571);
        println!("{:#?}", v);
        println!("illust_comments:");
        let v = client.illust.comments_v2(107879571);
        println!("{:#?}", v);
        println!("illust_ranking:");
        let v = client.illust.ranking(RankingMode::Day, None);
        println!("{:#?}", v);
        println!("illust_manga_recommended:");
        let v = client.illust.manga_recommended(true);
        println!("{:#?}", v);
        println!("illust_related:");
        let v = client.illust.related(107879571);
        println!("{:#?}", v);
        println!("illust_recommended:");
        let v = client.illust.recommended(true);
        println!("{:#?}", v);

        println!("search Api Test Start");
        println!("search_illust:");
        let v = client.search.illust("kurumin", Sort::DateDesc, Target::TitleAndCaption, true, false, None, None);
        println!("{:#?}", v);
        println!("search_novel:");
        let v = client.search.novel("date a live", Sort::DateDesc, Target::PartialMatchForTags, false, false);
        println!("{:#?}", v);
        println!("search_user:");
        let v = client.search.user("konnyaku");
        println!("{:#?}", v);
        println!("autocomplete:");
        let v = client.search.autocomplete("mune", true);
        println!("{:#?}", v);
    }
}
