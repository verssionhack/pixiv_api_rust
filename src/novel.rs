pub mod novel {
    use chrono::NaiveDateTime;
    use serde::{Serialize, Deserialize};
    use serde_json::Value;

    use crate::{user::user::User, illust::{ImageUrls}, utils::{datetime_deserializer, datetime_serializer}, traits::NextUrl, preload::{PrivacyPolicy, Restrict, XRestrict}};


    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Recommended {
        illusts: Vec<Novel>,
        ranking_illusts: Vec<Novel>,
        contest_exists: bool,
        privacy_policy: PrivacyPolicy,
        next_url: Option<String>,
    }

    impl NextUrl for Recommended {
        type Output = Self;
        fn has_next(&self) -> bool {
            self.next_url.is_some()
        }
        fn next_url(&self) -> Option<Self::Output> {
            unimplemented!()
        }
    }



    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Detail {
        novels: Vec<Novel>,
        user: User,
        next_url: Option<String>,
    }

    impl NextUrl for Detail {
        type Output = Self;
        fn has_next(&self) -> bool {
            self.next_url.is_some()
        }
        fn next_url(&self) -> Option<Self::Output> {
           unimplemented!() 
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Tag {
        name: String,
        translated_name: Option<String>,
        added_by_uploaded_user: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Series {
        id: Option<u64>,
        title: Option<String>,
    }
    
    impl Series {
        pub fn id(&self) -> Option<u64> {
            self.id
        }
        pub fn title(&self) -> Option<&String> {
            self.title.as_ref()
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Novel {
        id: u64,
        title: String,
        caption: String,
        restrict: Restrict,
        x_restrict: XRestrict,
        is_original: bool,
        image_urls: ImageUrls,
        #[serde(deserialize_with = "datetime_deserializer", serialize_with = "datetime_serializer")]
        create_date: NaiveDateTime,
        tags: Vec<Tag>,
        page_count: u64,
        text_length: u64,
        user: User,
        series: Series,
        is_bookmarked: bool,
        total_bookmarks: u64,
        total_view: u64,
        visible: bool,
        total_comments: u64,
        is_muted: bool,
        is_mypixiv_only: bool,
        is_x_restricted: bool,
    }
    impl Novel {
        pub fn is_mypixiv_only(&self) -> bool {
            self.is_mypixiv_only
        }
        pub fn is_x_restricted(&self) -> bool {
            self.is_x_restricted
        }
        pub fn total_comments(&self) -> u64 {
            self.total_comments
        }
        pub fn text_length(&self) -> u64 {
            self.text_length
        }
        pub fn is_original(&self) -> bool {
            self.is_original
        }
        pub fn is_muted(&self) -> bool {
            self.is_muted
        }
        pub fn visible(&self) -> bool {
            self.visible
        }
        pub fn is_bookmarked(&self) -> bool {
            self.is_bookmarked
        }
        pub fn total_bookmarks(&self) -> u64 {
            self.total_bookmarks
        }
        pub fn total_view(&self) -> u64 {
            self.total_view
        }
        pub fn series(&self) -> &Series {
            &self.series
        }
        pub fn x_restrict(&self) -> XRestrict {
            self.x_restrict
        }
        pub fn id(&self) -> u64 {
            self.id
        }
        pub fn title(&self) -> &str {
            &self.title
        }
        pub fn caption(&self) -> &str {
            &self.caption
        }
        pub fn image_urls(&self) -> &ImageUrls {
            &self.image_urls
        }
        pub fn restrict(&self) -> Restrict {
            self.restrict
        }
        pub fn page_count(&self) -> u64 {
            self.page_count
        }
        pub fn tags(&self) -> &Vec<Tag> {
            &self.tags
        }
        pub fn create_date(&self) -> NaiveDateTime {
            self.create_date
        }
        pub fn user(&self) -> &User {
            &self.user
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Text {
        novel_marker: Value,
        novel_text: String,
        series_prev: Novel,
        series_next: Novel,
    }
    impl Text {
        pub fn novel_marker(&self) -> &Value {
            &self.novel_marker
        }
        pub fn novel_text(&self) -> &str {
            &self.novel_text
        }
        pub fn novel_prev(&self) -> &Novel {
            &self.series_prev
        }
        pub fn novel_next(&self) -> &Novel {
            &self.series_next
        }
    }
}
pub mod comments {
    use chrono::NaiveDateTime;
    use serde::{Serialize, Deserialize};

    use crate::{user::ProfileImageUrls, utils::{datetime_deserializer, datetime_serializer}, traits::NextUrl};

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct Detail {
        comments: Vec<Comment>,
        next_url: Option<String>,
    }
    impl Detail {
        pub fn comments(&self) -> &Vec<Comment> {
            &self.comments
        }
    }
    impl NextUrl for Detail {
        type Output = Self;
        fn has_next(&self) -> bool {
            self.next_url.is_some()
        }
        fn next_url(&self) -> Option<Self::Output> {
            unimplemented!()
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct User {
        id: u64,
        name: String,
        account: String,
        profile_image_urls: ProfileImageUrls,
    }
    impl User {
        pub fn id(&self) -> u64 {
            self.id
        }
        pub fn name(&self) -> &str {
            &self.name
        }
        pub fn account(&self) -> &str {
            &self.account
        }
        pub fn profile_image_urls(&self) -> &ProfileImageUrls {
            &self.profile_image_urls
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct Comment {
        id: u64,
        comment: String,
        #[serde(deserialize_with = "datetime_deserializer", serialize_with = "datetime_serializer")]
        date: NaiveDateTime,
        user: User,
        has_replies: bool,
    }

    impl Comment {
        pub fn id(&self) -> u64 {
            self.id
        }
        pub fn comment(&self) -> &str {
            &self.comment
        }
        pub fn date(&self) -> NaiveDateTime {
            self.date
        }
        pub fn user(&self) -> &User {
            &self.user
        }
        pub fn has_replies(&self) -> bool {
            self.has_replies
        }
    }
}

pub mod series {
    use serde::{Serialize, Deserialize};

    use crate::{user::user::User, traits::NextUrl};

    use super::novel::Novel;

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct SeriesNovelResponse {
        novel_series_detail: SeriesDetail,
        novel_series_first_novel: Novel,
        novel_series_latest_novel: Novel,
        novels: Vec<Novel>,
        next_url: Option<String>,
    }
    impl SeriesNovelResponse {
        pub fn novel_series_detail(&self) -> &SeriesDetail {
            &self.novel_series_detail
        }
        pub fn novel_series_first_novel(&self) -> &Novel {
            &self.novel_series_first_novel
        }
        pub fn novel_series_latest_novel(&self) -> &Novel {
            &self.novel_series_latest_novel
        }
        pub fn novels(&self) -> &Vec<Novel> {
            &self.novels
        }
    }
    impl NextUrl for SeriesNovelResponse {
        type Output = Self;
        fn has_next(&self) -> bool {
            self.next_url.is_some()
        }
        fn next_url(&self) -> Option<Self::Output> {
            unimplemented!()
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct SeriesDetail {
        id: u64,
        title: String,
        caption: String,
        is_original: bool,
        is_concluded: bool,
        content_count: u64,
        total_character_count: u64,
        user: User,
        display_text: String,
    }
    impl SeriesDetail {
        pub fn id(&self) -> u64 {
            self.id
        }
        pub fn title(&self) -> &str {
            &self.title
        }
        pub fn caption(&self) -> &str {
            &self.caption
        }
        pub fn is_original(&self) -> bool {
            self.is_original
        }
        pub fn is_concluded(&self) -> bool {
            self.is_concluded
        }
        pub fn content_count(&self) -> u64 {
            self.content_count
        }
        pub fn total_character_count(&self) -> u64 {
            self.total_character_count
        }
        pub fn user(&self) -> &User {
            &self.user
        }
        pub fn display_text(&self) -> &str {
            &self.display_text
        }
    }
}