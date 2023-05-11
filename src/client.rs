const CLIENT_ID: &'static str = "MOBrBDS8blbauoSck0ZfDbtuzpyT";
const CLIENT_SECRET: &'static str = "lsACyCD94FhDUtGTXi3QzcFE2uU1hqtDaKeqrdwj";
pub mod auth {
    use std::collections::HashMap;

    use chrono::{DateTime, NaiveDateTime};
    use reqwest::{
        blocking::Client,
        header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, REFERER, USER_AGENT},
        Proxy,
    };
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    use crate::{error::AuthError, utils::u64_deserializer, preload::GrantType};

    use super::{CLIENT_ID, CLIENT_SECRET};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Response {
        access_token: String,
        expires_in: u64,
        token_type: String,
        scope: String,
        refresh_token: String,
        user: User,
        device_token: Option<String>,
        #[serde(skip)]
        pub proxy: Option<Proxy>,
    }
    impl Response {
        pub fn access_token(&self) -> &str {
            &self.access_token
        }
        pub fn expires_in(&self) -> u64 {
            self.expires_in
        }
        pub fn token_type(&self) -> &str {
            &self.token_type
        }
        pub fn scope(&self) -> &str {
            &self.scope
        }
        pub fn refresh_token(&self) -> &str {
            &self.refresh_token
        }
        pub fn user(&self) -> &User {
            &self.user
        }
        pub fn device_token(&self) -> Option<&String> {
            self.device_token.as_ref()
        }
        pub fn proxy(&self) -> Option<Proxy> {
            self.proxy.clone()
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct User {
        profile_image_urls: ProfileImageUrls,
        #[serde(deserialize_with = "u64_deserializer")]
        id: u64,
        name: String,
        account: String,
        mail_address: String,
        is_premium: bool,
        x_restrict: u64,
        is_mail_authorized: bool,
    }

    impl User {
        pub fn profile_image_urls(&self) -> &ProfileImageUrls {
            &self.profile_image_urls
        }
        pub fn id(&self) -> u64 {
            self.id
        }
        pub fn name(&self) -> &str {
            &self.name
        }
        pub fn account(&self) -> &str {
            &self.account
        }
        pub fn mail_address(&self) -> &str {
            &self.mail_address
        }
        pub fn is_premium(&self) -> bool {
            self.is_premium
        }
        pub fn x_restrict(&self) -> u64 {
            self.x_restrict
        }
        pub fn is_mail_authorized(&self) -> bool {
            self.is_mail_authorized
        }
    }
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ProfileImageUrls {
        px_16x16: String,
        px_50x50: String,
        px_170x170: String,
    }
    impl ProfileImageUrls {
        pub fn px_16x16(&self) -> &str {
            &self.px_16x16
        }
        pub fn px_50x50(&self) -> &str {
            &self.px_50x50
        }
        pub fn px_170x170(&self) -> &str {
            &self.px_170x170
        }
    }


    #[derive(Debug, Clone, Default)]
    pub struct ClientBudiler {
        grant_type: Option<GrantType>,
        username: Option<String>,
        password: Option<String>,
        refresh_token: Option<String>,
        device_token: Option<String>,
        secure_url: Option<bool>,
        include_policy: Option<bool>,
        proxy: Option<Proxy>,
    }

    impl ClientBudiler {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn build(self) -> Result<super::api::Client, AuthError> {
            let mut headers = HeaderMap::new();
            headers.insert(
                REFERER,
                HeaderValue::from_str("https://www.pixiv.net").unwrap(),
            );
            headers.insert(
                USER_AGENT,
                HeaderValue::from_str("PixivAndroidApp/5.0.64 (Android 6.0)").unwrap(),
            );
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
            );
            let mut formbody = HashMap::new();
            formbody.insert("client_id", CLIENT_ID);
            formbody.insert("client_secret", CLIENT_SECRET);
            formbody.insert("grant_type", "refresh_token");
            formbody.insert(
                "refresh_token",
                self.refresh_token.as_ref().unwrap().as_str(),
            );
            let mut client_builder =
                reqwest::blocking::ClientBuilder::new().default_headers(headers);
            if let Some(proxy) = self.proxy.as_ref() {
                client_builder = client_builder.proxy(proxy.clone());
            }
            let client = client_builder.build().unwrap();
            let res = client
                .post("https://oauth.secure.pixiv.net/auth/token")
                .form(&formbody)
                .send().map_err(|e| {
                    panic!("{e:#?}")
                }).unwrap();
            if !res.status().is_success() {
                panic!("Auth error {:#?}", res.headers());
            }

            let res: Value = res.json().map_err(|e| {
                    panic!("{e:#?}")
                }).unwrap();
            if res.get("has_error").is_none() {
                let mut response: Response =
                    serde_json::from_value(res["response"].clone()).unwrap();
                response.proxy = self.proxy;
                Ok(response.into())
            } else {
                Err(serde_json::from_value(res).unwrap())
            }
        }
        pub fn include_policy(&mut self, v: bool) -> &mut Self {
            self.include_policy = Some(v);
            self
        }
        pub fn proxy(&mut self, v: Proxy) -> &mut Self {
            self.proxy = Some(v);
            self
        }
        pub fn secure_url(&mut self, v: bool) -> &mut Self {
            self.secure_url = Some(v);
            self
        }
        pub fn device_token<T>(&mut self, v: T) -> &mut Self
        where
            T: ToString,
        {
            self.device_token = Some(v.to_string());
            self
        }
        pub fn refresh_token<T>(&mut self, v: T) -> &mut Self
        where
            T: ToString,
        {
            self.refresh_token = Some(v.to_string());
            self
        }
        pub fn password<T>(&mut self, v: T) -> &mut Self
        where
            T: ToString,
        {
            self.password = Some(v.to_string());
            self
        }
        pub fn username<T>(&mut self, v: T) -> &mut Self
        where
            T: ToString,
        {
            self.username = Some(v.to_string());
            self
        }
        pub fn grant_type(&mut self, v: GrantType) -> &mut Self {
            self.grant_type = Some(v);
            self
        }
    }
}

pub mod api {

    use lazy_static::lazy_static;
    use reqwest::{
        blocking::{Request, RequestBuilder},
        header::{HeaderMap, REFERER, USER_AGENT},
        Method, Proxy, Url,
    };
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    type ApiResult<T> = Result<T, ApiError>;
    type RequestResult<T> = Result<ApiResult<T>, dyn Error>;

    use crate::{
        error::ApiError,
        illust::{self},
        novel::{self},
        user::user,
    };

    lazy_static! {
        static ref BASE_URL: Url = Url::from_str("https://app-api.pixiv.net").unwrap();
    }

    use self::{
        api_illust::IllustApi, api_novel::NovelApi, api_search::SearchApi, api_user::UserApi,
    };

    use super::{
        auth::{Response, User},
        Response::{BookmarkTags, Illusts},
    };
    use std::{collections::HashMap, rc::Rc, str::FromStr, error::Error};

    #[macro_export]
    macro_rules! params {
        ($($k: expr => $v: expr,)*) => ({
            let mut params = std::collections::HashMap::new();
            $(
                params.insert($k.to_string(), $v.to_string());
            )*
            params
        });
        ($($k: expr => $v: expr),*) => ({
            let mut params = std::collections::HashMap::new();
            $(
                params.insert($k.to_string(), $v.to_string());
            )*
            params
        });
    }

    pub mod api_search {

        use std::rc::Rc;

        use reqwest::Method;
        use serde::{Deserialize, Serialize};

        use crate::{client::Response::{self, SearchAutoCompleteKeywords}, novel, user, preload::{Sort, Target}};

        use super::{ApiResult, Client};


        #[derive(Clone, Default, Debug)]
        pub struct SearchApi {
            pub client: Option<Rc<Client>>,
        }
        impl SearchApi {
            pub fn autocomplete<T>(&self, word: T, merge_plain_keyword_results: bool) -> ApiResult<SearchAutoCompleteKeywords>
            where
                T: ToString,
            {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/search/autocomplete")
                    .query(&[("word", word.to_string())])
                    .query(&[("merge_plain_keyword_results", merge_plain_keyword_results)]);
                Client::response(req)
            }
            pub fn autocomplete_v2<T>(&self, word: T, merge_plain_keyword_results: bool) -> ApiResult<Response::Tags>
            where
                T: ToString,
            {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v2/search/autocomplete")
                    .query(&[("word", word.to_string())])
                    .query(&[("merge_plain_keyword_results", merge_plain_keyword_results)]);
                Client::response(req)
            }
            pub fn user<T>(&self, word: T) -> ApiResult<user::user::Previews>
            where
                T: ToString,
            {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/search/user")
                    .query(&[("word", word.to_string())]);
                Client::response(req)
            }
            pub fn novel_popular_preview<T>(
                &self,
                word: T,
                sort: Sort,
                search_target: Target,
                include_translated_tag_results: bool,
                merge_plain_keyword_results: bool,
            ) -> ApiResult<Response::Novels>
            where
                T: ToString,
            {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/search/popular-preview/novel")
                    .query(&[
                        ("word", word.to_string()),
                        ("sort", sort.to_string()),
                        ("search_target", search_target.to_string()),
                    ])
                    .query(&[
                        (
                            "include_translated_tag_results",
                            include_translated_tag_results,
                        ),
                        ("merge_plain_keyword_results", merge_plain_keyword_results),
                    ]);
                Client::response(req)
            }
            pub fn novel<T>(
                &self,
                word: T,
                sort: Sort,
                search_target: Target,
                include_translated_tag_results: bool,
                merge_plain_keyword_results: bool,
            ) -> ApiResult<Response::Novels>
            where
                T: ToString,
            {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/search/novel")
                    .query(&[
                        ("word", word.to_string()),
                        ("sort", sort.to_string()),
                        ("search_target", search_target.to_string()),
                    ])
                    .query(&[
                        (
                            "include_translated_tag_results",
                            include_translated_tag_results,
                        ),
                        ("merge_plain_keyword_results", merge_plain_keyword_results),
                    ]);
                Client::response(req)
            }
            pub fn illust_popular_preview<T>(
                &self,
                word: T,
                sort: Sort,
                search_target: Target,
                include_translated_tag_results: bool,
                merge_plain_keyword_results: bool,
                start_date: Option<u64>,
                end_date: Option<u64>,
            ) -> ApiResult<Response::Illusts>
            where
                T: ToString,
            {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/search/popular-preview/illust")
                    .query(&[
                        ("word", word.to_string()),
                        ("sort", sort.to_string()),
                        ("search_target", search_target.to_string()),
                    ])
                    .query(&[
                        (
                            "include_translated_tag_results",
                            include_translated_tag_results,
                        ),
                        ("merge_plain_keyword_results", merge_plain_keyword_results),
                    ]);
                if let Some(date) = start_date {
                    req = req.query(&[("start_date", date)]);
                }
                if let Some(date) = end_date {
                    req = req.query(&[("end_date", date)]);
                }
                Client::response(req)
            }
            pub fn illust<T>(
                &self,
                word: T,
                sort: Sort,
                search_target: Target,
                include_translated_tag_results: bool,
                merge_plain_keyword_results: bool,
                start_date: Option<u64>,
                end_date: Option<u64>,
            ) -> ApiResult<Response::Illusts>
            where
                T: ToString,
            {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/search/illust")
                    .query(&[
                        ("word", word.to_string()),
                        ("sort", sort.to_string()),
                        ("search_target", search_target.to_string()),
                    ])
                    .query(&[
                        (
                            "include_translated_tag_results",
                            include_translated_tag_results,
                        ),
                        ("merge_plain_keyword_results", merge_plain_keyword_results),
                    ]);
                if let Some(date) = start_date {
                    req = req.query(&[("start_date", date)]);
                }
                if let Some(date) = end_date {
                    req = req.query(&[("end_date", date)]);
                }
                Client::response(req)
            }
        }
    }
    pub mod api_user {
        use std::rc::Rc;

        use reqwest::Method;

        use crate::{client::Response, illust, novel, user, preload::{Restrict, IllustType}};

        use super::{ApiResult, Client};

        #[derive(Clone, Default, Debug)]
        pub struct UserApi {
            pub client: Option<Rc<Client>>,
        }
        impl UserApi {
            pub fn detail(&self, user_id: u64) -> ApiResult<user::user::Detail> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/user/detail")
                    .query(&[("user_id", user_id)]);
                Client::response(req)
            }

            pub fn mypixiv(&self, user_id: u64, offset: usize) -> ApiResult<user::user::Previews> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/user/mypixiv")
                    .query(&[
                        ("user_id", user_id.to_string()),
                        ("offset", offset.to_string()),
                    ]);
                Client::response(req)
            }

            pub fn illusts(
                &self,
                user_id: u64,
                tp: IllustType,
                offset: usize,
            ) -> ApiResult<Response::Illusts> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/user/illusts")
                    .query(&[
                        ("user_id", user_id.to_string()),
                        ("type", tp.to_string()),
                        ("offset", offset.to_string()),
                    ]);
                Client::response(req)
            }

            pub fn novels(&self, user_id: u64) -> ApiResult<novel::novel::Detail> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/user/novels")
                    .query(&[("user_id", user_id.to_string())]);
                Client::response(req)
            }
            pub fn bookmarks_novel(
                &self,
                user_id: u64,
                restrict: Restrict,
            ) -> ApiResult<novel::novel::Detail> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/user/bookmarks/novel")
                    .query(&[
                        ("user_id", user_id.to_string()),
                        ("restrict", restrict.to_string()),
                    ]);
                Client::response(req)
            }
            pub fn bookmark_illust_tags(
                &self,
                user_id: u64,
                restrict: Restrict,
            ) -> ApiResult<Response::BookmarkTags> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/user/bookmark-tags/illust")
                    .query(&[
                        ("user_id", user_id.to_string()),
                        ("restrict", restrict.to_string()),
                    ]);
                Client::response(req)
            }

            pub fn bookmark_novel_tags(
                &self,
                user_id: u64,
                restrict: Restrict,
            ) -> ApiResult<Response::BookmarkTags> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/user/bookmark-tags/novel")
                    .query(&[
                        ("user_id", user_id.to_string()),
                        ("restrict", restrict.to_string()),
                    ]);
                Client::response(req)
            }

            pub fn bookmarks_illust(
                &self,
                user_id: u64,
                restrict: Restrict,
                max_bookmark_id: Option<u64>,
                tag: Option<String>,
            ) -> ApiResult<Response::Illusts> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/user/bookmarks/illust")
                    .query(&[
                        ("user_id", user_id.to_string()),
                        ("restrict", restrict.to_string()),
                    ]);
                if let Some(v) = max_bookmark_id {
                    req = req.query(&[("max_bookmark_id", v)]);
                }
                if let Some(v) = tag {
                    req = req.query(&[("tag", v)]);
                }
                Client::response(req)
            }
            pub fn follow_add(&self, user_id: u64, restrict: Restrict) -> ApiResult<()> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::POST, "/v1/user/follow/add")
                    .form(&params! {
                    "user_id" => user_id,
                    "restrict" => restrict,
                    });
                Client::response(req)
            }
            pub fn follow_delete(&self, user_id: u64) -> ApiResult<()> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::POST, "/v1/user/follow/delete")
                    .form(&params! {
                    "user_id" => user_id,
                    });
                Client::response(req)
            }

            pub fn following(
                &self,
                user_id: u64,
                restrict: Restrict,
                offset: usize,
            ) -> ApiResult<user::user::Previews> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/user/following")
                    .query(&[
                        ("user_id", user_id.to_string()),
                        ("restrict", restrict.to_string()),
                        ("offset", offset.to_string()),
                    ]);
                Client::response(req)
            }

            pub fn follower(&self, user_id: u64, offset: usize) -> ApiResult<user::user::Previews> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/user/follower")
                    .query(&[
                        ("user_id", user_id.to_string()),
                        ("offset", offset.to_string()),
                    ]);
                Client::response(req)
            }
        }
    }
    pub mod api_illust {

        use std::rc::Rc;

        use reqwest::Method;

        use crate::{client::Response, illust, preload::{Restrict, RankingMode}};

        use super::{ApiResult, Client};

        #[derive(Clone, Default, Debug)]
        pub struct IllustApi {
            pub client: Option<Rc<Client>>,
        }
        impl IllustApi {
            pub fn detail(&self, illust_id: u64) -> ApiResult<illust::illust::Detail> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/illust/detail")
                    .query(&[("illust_id", illust_id)]);
                Client::response(req)
            }
            pub fn bookmark_add(&self, illust_id: u64, restrict: Restrict) -> ApiResult<()> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::POST, "/v2/illust/bookmark/add")
                    .query(&[("illust_id", illust_id)])
                    .query(&[("restrict", restrict.to_string())]);
                Client::response(req)
            }
            pub fn bookmark_delete(&self, illust_id: u64) -> ApiResult<()> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::POST, "/v2/illust/bookmark/delete")
                    .query(&[("illust_id", illust_id)]);
                Client::response(req)
            }
            pub fn bookmark_detail(
                &self,
                illust_id: u64,
            ) -> ApiResult<illust::illust::BookmarkTagsResponse> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v2/illust/bookmark/detail")
                    .query(&[("illust_id", illust_id)]);
                Client::response(req)
            }
            pub fn related(&self, illust_id: u64) -> ApiResult<Response::Illusts> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v2/illust/related")
                    .query(&[("illust_id", illust_id)]);
                Client::response(req)
            }
            pub fn ranking(&self, mode: RankingMode, date: Option<String>) -> ApiResult<Response::Illusts> {
                let mut req = self.client.as_ref().unwrap()
                    .request(Method::GET, "/v1/illust/ranking")
                    .query(&[("mode", mode.to_string())]);
                if let Some(date) = date {
                    req = req.query(&[("date", date)]);
                }
                Client::response(req)
            }
            pub fn recommended(
                &self,
                include_ranking_illusts: bool,
            ) -> ApiResult<illust::illust::Recommended> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/illust/recommended")
                    .query(&[("include_ranking_illusts", include_ranking_illusts)]);
                Client::response(req)
            }
            pub fn comments(
                &self,
                illust_id: u64,
                include_total_comments: bool,
            ) -> ApiResult<illust::comments::Detail> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/illust/comments")
                    .query(&[("illust_id", illust_id)])
                    .query(&[("include_total_comments", include_total_comments)]);
                Client::response(req)
            }

            pub fn comments_v2(&self, illust_id: u64) -> ApiResult<illust::comments::Detail> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v2/illust/comments")
                    .query(&[("illust_id", illust_id)]);
                Client::response(req)
            }

            pub fn comment_replies(&self, comment_id: u64) -> ApiResult<illust::comments::Detail> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/illust/comment/replies")
                    .query(&[("comment_id", comment_id)]);
                Client::response(req)
            }
            pub fn manga_recommended(
                &self,
                include_ranking_label: bool,
            ) -> ApiResult<illust::illust::Recommended> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/manga/recommended")
                    .query(&[("include_ranking_label", include_ranking_label)]);
                Client::response(req)
            }

            pub fn ugoira_metadata(&self, illust_id: &str) -> ApiResult<illust::ugoira::Detail> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/ugoira/metadata")
                    .query(&[("illust_id", illust_id)]);
                Client::response(req)
            }
        }
    }
    pub mod api_novel {

        use std::rc::Rc;

        use reqwest::Method;

        use crate::{illust, novel, preload::{Restrict, RankingMode}};

        use super::{ApiResult, Client};

        #[derive(Clone, Default, Debug)]
        pub struct NovelApi {
            pub client: Option<Rc<Client>>,
        }
        impl NovelApi {
            pub fn detail(&self, novel_id: u64) -> ApiResult<novel::novel::Detail> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v2/novel/detail")
                    .query(&[("novel_id", novel_id)]);
                Client::response(req)
            }
            pub fn bookmark_add(&self, novel_id: u64, restrict: Restrict) -> ApiResult<()> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::POST, "/v2/novel/bookmark/add")
                    .query(&[("novel_id", novel_id)])
                    .query(&[("restrict", restrict.to_string())]);
                Client::response(req)
            }
            pub fn bookmark_delete(&self, novel_id: u64) -> ApiResult<()> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::POST, "/v2/novel/bookmark/delete")
                    .query(&[("novel_id", novel_id)]);
                Client::response(req)
            }
            pub fn bookmark_detail(
                &self,
                illust_id: u64,
            ) -> ApiResult<illust::illust::BookmarkTagsResponse> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v2/illust/bookmark/detail")
                    .query(&[("illust_id", illust_id)]);
                Client::response(req)
            }
            pub fn text(&self, novel_id: u64) -> ApiResult<novel::novel::Text> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/novel/text")
                    .query(&[("novel_id", novel_id)]);
                Client::response(req)
            }
            pub fn ranking(&self, mode: RankingMode, date: Option<String>) -> ApiResult<novel::novel::Detail> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/novel/ranking")
                    .query(&[("mode", mode.to_string())]);
                if let Some(date) = date {
                    req = req.query(&[("date", date)]);
                }
                Client::response(req)
            }
            pub fn recommended(
                &self,
                include_ranking_novels: bool,
            ) -> ApiResult<novel::novel::Recommended> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/novel/recommended")
                    .query(&[("include_ranking_novels", include_ranking_novels)]);
                Client::response(req)
            }
            pub fn comments(
                &self,
                novel_id: u64,
                include_total_comments: bool,
            ) -> ApiResult<novel::novel::Detail> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/novel/comments")
                    .query(&[("novel_id", novel_id)])
                    .query(&[("include_total_comments", include_total_comments)]);
                Client::response(req)
            }

            pub fn comments_v2(&self, novel_id: u64) -> ApiResult<novel::comments::Detail> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v2/novel/comments")
                    .query(&[("novel_id", novel_id)]);
                Client::response(req)
            }

            pub fn comment_replies(&self, comment_id: u64) -> ApiResult<novel::comments::Detail> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/novel/comment/replies")
                    .query(&[("comment_id", comment_id)]);
                Client::response(req)
            }
            pub fn series(&self, series_id: u64) -> ApiResult<novel::series::SeriesNovelResponse> {
                let mut req = self
                    .client
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/novel/series")
                    .query(&[("series_id", series_id)]);
                Client::response(req)
            }
        }
    }


    #[derive(Debug, Clone)]
    pub struct Client {
        access_token: String,
        expires_in: u64,
        token_type: String,
        scope: String,
        refresh_token: String,
        userinfo: User,
        pub client: reqwest::blocking::Client,
        default_headers: HeaderMap,
        pub search: SearchApi,
        pub user: UserApi,
        pub novel: NovelApi,
        pub illust: IllustApi,
    }
    impl From<Response> for Client {
        fn from(value: Response) -> Self {
            let mut default_headers = HeaderMap::new();
            default_headers.insert(REFERER, "https://www.pixiv.net".parse().unwrap());
            default_headers.insert(USER_AGENT, "pixivdownloader/".parse().unwrap());
            let mut client_builder =
                reqwest::blocking::ClientBuilder::new().default_headers(default_headers.clone());
            if let Some(proxy) = value.proxy() {
                client_builder =
                    client_builder.proxy(Proxy::all("http://localhost:15777").unwrap());
            }
            let mut sf = Self {
                client: client_builder.build().unwrap(),
                default_headers: default_headers,
                access_token: value.access_token().to_string(),
                token_type: value.token_type().to_string(),
                expires_in: value.expires_in(),
                scope: value.scope().to_string(),
                refresh_token: value.refresh_token().to_string(),
                userinfo: value.user().clone(),
                search: SearchApi::default(),
                user: UserApi::default(),
                novel: NovelApi::default(),
                illust: IllustApi::default(),
            };
            let sf_rc = Rc::new(sf.clone());
            sf.search.client = Some(sf_rc.clone());
            sf.user.client = Some(sf_rc.clone());
            sf.novel.client = Some(sf_rc.clone());
            sf.illust.client = Some(sf_rc.clone());
            sf
        }
    }
    impl Client {
        fn access_token(&self) -> &str {
            &self.access_token
        }
        fn expires_in(&self) -> u64 {
            self.expires_in
        }
        fn request<'a>(&self, method: Method, path: &'a str) -> RequestBuilder {
            self.client
                .request(method, BASE_URL.join(path).unwrap())
                .bearer_auth(self.access_token())
        }
        fn no_auth_request<'a>(&self, method: Method, path: &'a str) -> RequestBuilder {
            self.client.request(method, BASE_URL.join(path).unwrap())
        }
        fn token_type(&self) -> &str {
            &self.token_type
        }
        fn scope(&self) -> &str {
            &self.scope
        }
        fn refresh_token(&self) -> &str {
            &self.refresh_token
        }
        pub fn userinfo(&self) -> &User {
            &self.userinfo
        }
        fn response<T>(request: RequestBuilder) -> ApiResult<T>
        where
            T: for<'de> Deserialize<'de>,
        {
            let res_json: Value = request.send().unwrap().json().unwrap();
            //println!("{:#?}", res_json);
            if res_json.get("error").is_none() {
                Ok(serde_json::from_value(res_json).unwrap())
            } else {
                Err(serde_json::from_value(res_json).unwrap())
            }
        }
    }
}

pub mod Response {
    use serde::{Deserialize, Serialize};

    use crate::{
        illust::illust::{self, Tag},
        novel,
        traits::NextUrl,
    };

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Novels {
        novels: Vec<novel::novel::Novel>,
        next_url: Option<String>,
        search_span_limit: Option<u64>,
    }
    impl Novels {
        pub fn novels(&self) -> &Vec<novel::novel::Novel> {
            &self.novels
        }
        pub fn search_span_limit(&self) -> Option<u64> {
            self.search_span_limit
        }
    }
    impl NextUrl for Novels {
        type Output = Self;
        fn has_next(&self) -> bool {
            self.next_url.is_some()
        }
        fn next_url(&self) -> Option<Self::Output> {
            unimplemented!()
        }
    }
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Illusts {
        illusts: Vec<illust::Illust>,
        next_url: Option<String>,
        search_span_limit: Option<u64>,
    }
    impl Illusts {
        pub fn illusts(&self) -> &Vec<illust::Illust> {
            &self.illusts
        }
        pub fn search_span_limit(&self) -> Option<u64> {
            self.search_span_limit
        }
    }
    impl NextUrl for Illusts {
        type Output = Self;
        fn has_next(&self) -> bool {
            self.next_url.is_some()
        }
        fn next_url(&self) -> Option<Self::Output> {
            unimplemented!()
        }
    }
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct SearchAutoCompleteKeywords {
        search_auto_complete_keywords: Vec<String>
    }
    impl SearchAutoCompleteKeywords {
        pub fn search_auto_complete_keywords(&self) -> &Vec<String> {
            &self.search_auto_complete_keywords
        }
    }
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Tags {
        tags: Vec<illust::Tag>
    }
    impl Tags {
        pub fn tags(&self) -> &Vec<illust::Tag> {
            &self.tags
        }
    }
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct BookmarkTags {
        bookmark_tags: Vec<Tag>,
        next_url: Option<String>,
    }
    impl NextUrl for BookmarkTags {
        type Output = Self;
        fn has_next(&self) -> bool {
            self.next_url.is_some()
        }
        fn next_url(&self) -> Option<Self::Output> {
            unimplemented!()
        }
    }
    impl BookmarkTags {
        pub fn bookmark_tags(&self) -> &Vec<Tag> {
            &self.bookmark_tags
        }
    }
}
