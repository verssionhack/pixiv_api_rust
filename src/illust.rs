use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct ImageUrls {
    square_medium: Option<String>,
    medium: Option<String>,
    large: Option<String>,
    original: Option<String>,
}

impl ImageUrls {
    pub fn square_medium(&self) -> Option<&String> {
        self.square_medium.as_ref()
    }
    pub fn medium(&self) -> Option<&String> {
        self.medium.as_ref()
    }
    pub fn large(&self) -> Option<&String> {
        self.large.as_ref()
    }
    pub fn original(&self) -> Option<&String> {
        self.original.as_ref()
    }
}

pub mod series {
    use std::rc::Rc;

    use chrono::NaiveDateTime;
    use reqwest::Method;
    use serde::{Deserialize, Serialize};

    use crate::{
        traits::{NextUrl, Pagible},
        user::user::User,
        utils::{datetime_deserializer, datetime_serializer}, client::api::{Client, ApiResult},
    };

    use super::illust::Illust;

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct SeriesResponse {
        illust_series_detail: SeriesDetail,
        illust_series_first_illust: Illust,
        illusts: Vec<Illust>,
        next_url: Option<String>,
        #[serde(skip)]
        pub(crate) client: Option<Rc<Client>>,
    }
    impl SeriesResponse {
        pub fn illust_series_detail(&self) -> &SeriesDetail {
            &self.illust_series_detail
        }
        pub fn illust_series_first_illust(&self) -> &Illust {
            &self.illust_series_first_illust
        }
        pub fn illusts(&self) -> &Vec<Illust> {
            self.illusts.as_ref()
        }
        pub fn next_url(&self) -> Option<&String> {
            self.next_url.as_ref()
        }
    }
    impl NextUrl for SeriesResponse {
        type Output = Self;
        fn has_next(&self) -> bool {
            self.next_url().is_some()
        }
        fn next_url(&self) -> Option<ApiResult<Self::Output>> {
            let mut ret: ApiResult<Self::Output> = Client::response(self.client.as_ref()?.request(Method::GET, self.next_url.as_ref()?));
            Some(ret.map(|v| {
                let mut v = v;
                v.client = self.client.clone();
                v
            }))
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct SeriesIllustResponse {
        illust_series_detail: SeriesDetail,
        illust_series_context: SeriesContext,
    }
    impl SeriesIllustResponse {
        pub fn illust_series_detail(&self) -> &SeriesDetail {
            &self.illust_series_detail
        }
        pub fn illust_series_context(&self) -> &SeriesContext {
            &self.illust_series_context
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct CoverImageUrls {
        medium: String,
    }
    impl CoverImageUrls {
        pub fn medium(&self) -> &str {
            &self.medium
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct SeriesDetail {
        id: u64,
        title: String,
        caption: String,
        cover_image_urls: CoverImageUrls,
        series_work_count: u64,
        #[serde(
            deserialize_with = "datetime_deserializer",
            serialize_with = "datetime_serializer"
        )]
        create_date: NaiveDateTime,
        width: u64,
        height: u64,
        user: User,
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
        pub fn cover_image_urls(&self) -> &CoverImageUrls {
            &self.cover_image_urls
        }
        pub fn series_work_count(&self) -> u64 {
            self.series_work_count
        }
        pub fn create_date(&self) -> NaiveDateTime {
            self.create_date
        }
        pub fn width(&self) -> u64 {
            self.width
        }
        pub fn height(&self) -> u64 {
            self.height
        }
        pub fn user(&self) -> &User {
            &self.user
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct SeriesContext {
        content_order: u64,
        prev: Option<Illust>,
        next: Option<Illust>,
        #[serde(skip)]
        pub(crate) client: Option<Rc<Client>>,
    }

    impl SeriesContext {
        pub fn content_order(&self) -> u64 {
            self.content_order
        }
    }

    impl Pagible for SeriesContext {
        type Output = Illust;
        fn next(&self) -> Option<Self::Output> {
            self.next.clone()
        }
        fn prev(&self) -> Option<Self::Output> {
            self.prev.clone()
        }
    }
}

pub mod illust {
    use std::rc::Rc;

    use chrono::NaiveDateTime;
    use reqwest::Method;
    use serde::{Deserialize, Serialize};

    use crate::{
        novel::novel::Series,
        preload::{PrivacyPolicy, Restrict, XRestrict, IllustType},
        traits::NextUrl,
        user::user::{User, UserDetail},
        utils::{datetime_deserializer, datetime_serializer}, client::api::{Client, ApiResult},
    };

    use super::{ImageUrls};

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct Recommended {
        illusts: Vec<Illust>,
        ranking_illusts: Vec<Illust>,
        #[serde(default)]
        contest_exists: bool,
        privacy_policy: PrivacyPolicy,
        next_url: Option<String>,
        #[serde(skip)]
        pub(crate) client: Option<Rc<Client>>,
    }
    impl Recommended {
        pub fn illusts(&self) -> &Vec<Illust> {
            &self.illusts
        }
        pub fn ranking_illusts(&self) -> &Vec<Illust> {
            &self.ranking_illusts
        }
        pub fn contest_exists(&self) -> bool {
            self.contest_exists
        }
        pub fn privacy_policy(&self) -> PrivacyPolicy {
            self.privacy_policy.clone()
        }
    }

    impl NextUrl for Recommended {
        type Output = Self;
        fn has_next(&self) -> bool {
            self.next_url.is_some()
        }
        fn next_url(&self) -> Option<ApiResult<Self::Output>> {
            let mut ret: ApiResult<Self::Output> = Client::response(self.client.as_ref()?.request(Method::GET, self.next_url.as_ref()?));
            Some(ret.map(|v| {
                let mut v = v;
                v.client = self.client.clone();
                v
            }))
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct Tag {
        name: String,
        translated_name: Option<String>,
    }
    impl Tag {
        pub fn name(&self) -> &str {
            &self.name
        }
        pub fn translated_name(&self) -> Option<&String> {
            self.translated_name.as_ref()
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct MetaSinglePage {
        description: Option<String>,
        original_image_url: Option<String>,
    }

    impl MetaSinglePage {
        pub fn description(&self) -> Option<&String> {
            self.description.as_ref()
        }
        pub fn original_image_url(&self) -> Option<&String> {
            self.original_image_url.as_ref()
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct Detail {
        illust: Illust,
    }

    impl Detail {
        pub fn illust(&self) -> &Illust {
            &self.illust
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct MetaPage {
        description: Option<String>,
        image_urls: ImageUrls,
    }

    impl MetaPage {
        pub fn description(&self) -> Option<&String> {
            self.description.as_ref()
        }
        pub fn image_urls(&self) -> &ImageUrls {
            &self.image_urls
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct Illust {
        id: u64,
        title: String,
        r#type: IllustType,
        image_urls: ImageUrls,
        caption: String,
        illust_ai_type: u64,
        illust_book_style: u64,
        restrict: Restrict,
        user: User,
        tags: Vec<Tag>,
        tools: Vec<String>,
        #[serde(
            deserialize_with = "datetime_deserializer",
            serialize_with = "datetime_serializer"
        )]
        create_date: NaiveDateTime,
        page_count: u64,
        width: u64,
        height: u64,
        sanity_level: u64,
        x_restrict: XRestrict,
        series: Option<Series>,
        meta_single_page: MetaSinglePage,
        meta_pages: Vec<MetaPage>,
        total_view: u64,
        total_bookmarks: u64,
        is_bookmarked: bool,
        visible: bool,
        is_muted: bool,
        #[serde(default)]
        total_comments: u64,
    }
    impl Illust {
        pub fn total_comments(&self) -> u64 {
            self.total_comments
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
        pub fn illust_book_style(&self) -> u64 {
            self.illust_book_style
        }
        pub fn illust_ai_type(&self) -> u64 {
            self.illust_ai_type
        }
        pub fn total_bookmarks(&self) -> u64 {
            self.total_bookmarks
        }
        pub fn total_view(&self) -> u64 {
            self.total_view
        }
        pub fn meta_pages(&self) -> &Vec<MetaPage> {
            &self.meta_pages
        }
        pub fn meta_single_page(&self) -> &MetaSinglePage {
            &self.meta_single_page
        }
        pub fn series(&self) -> &Option<Series> {
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
        pub fn r#type(&self) -> IllustType {
            self.r#type
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
        pub fn sanity_level(&self) -> u64 {
            self.sanity_level
        }
        pub fn tools(&self) -> &Vec<String> {
            &self.tools
        }
        pub fn tags(&self) -> &Vec<Tag> {
            &self.tags
        }
        pub fn create_date(&self) -> NaiveDateTime {
            self.create_date
        }
        pub fn width(&self) -> u64 {
            self.width
        }
        pub fn height(&self) -> u64 {
            self.height
        }
        pub fn user(&self) -> &User {
            &self.user
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct BookmarkTags {
        is_bookmarked: bool,
        restrict: Restrict,
        tags: Vec<Tag>,
    }
    impl BookmarkTags {
        pub fn is_bookmarked(&self) -> bool {
            self.is_bookmarked
        }
        pub fn restrict(&self) -> Restrict {
            self.restrict
        }
        pub fn tags(&self) -> &Vec<Tag> {
            &self.tags
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct BookmarkTagsResponse {
        bookmark_detail: BookmarkTags,
    }
    impl BookmarkTagsResponse {
        pub fn bookmark_detail(&self) -> &BookmarkTags {
            &self.bookmark_detail
        }
    }
}

pub mod comments {
    use std::rc::Rc;

    use crate::{
        traits::NextUrl,
        user::ProfileImageUrls,
        utils::{datetime_deserializer, datetime_serializer}, client::api::{Client, ApiResult},
    };
    use chrono::NaiveDateTime;
    use reqwest::Method;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct Detail {
        comments: Vec<Comment>,
        next_url: Option<String>,
        #[serde(skip)]
        pub(crate) client: Option<Rc<Client>>,
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
        fn next_url(&self) -> Option<ApiResult<Self::Output>> {
            let mut ret: ApiResult<Self::Output> = Client::response(self.client.as_ref()?.request(Method::GET, self.next_url.as_ref()?));
            Some(ret.map(|v| {
                let mut v = v;
                v.client = self.client.clone();
                v
            }))
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
        #[serde(
            deserialize_with = "datetime_deserializer",
            serialize_with = "datetime_serializer"
        )]
        date: NaiveDateTime,
        user: User,
        #[serde(default = "bool::default")]
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

pub mod ugoira {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct Frame {
        delay: u64,
        file: String,
    }

    impl Frame {
        pub fn delay(&self) -> u64 {
            self.delay
        }
        pub fn file(&self) -> &str {
            &self.file
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct ZipUrls {
        medium: Option<String>,
    }
    impl ZipUrls {
        pub fn medium(&self) -> Option<&String> {
            self.medium.as_ref()
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct Metadata {
        frames: Vec<Frame>,
        zip_urls: ZipUrls,
    }
    impl Metadata {
        pub fn frames(&self) -> &Vec<Frame> {
            &self.frames
        }
        pub fn zip_urls(&self) -> &ZipUrls {
            &self.zip_urls
        }
    }
    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct Detail {
        ugoira_metadata: Metadata,
    }

    impl Detail {
        pub fn ugoira_metadata(&self) -> &Metadata {
            &self.ugoira_metadata
        }
    }
}
