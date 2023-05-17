use serde::{Deserialize, Serialize};

const CLIENT_ID: &'static str = "MOBrBDS8blbauoSck0ZfDbtuzpyT";
const CLIENT_SECRET: &'static str = "lsACyCD94FhDUtGTXi3QzcFE2uU1hqtDaKeqrdwj";


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiNone {}
pub mod auth {
    use std::{collections::HashMap, time::Duration, sync::Arc};

    use chrono::{DateTime, NaiveDateTime};
    use reqwest::{
        blocking::Client,
        header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, REFERER, USER_AGENT},
        Proxy,
    };
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    use crate::{error::AuthError, utils::u64_deserializer, preload::GrantType};

    use super::{CLIENT_ID, CLIENT_SECRET, api};

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
        pub(crate) proxy: Option<Proxy>,
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
        pub fn device_token(&self) -> Option<&str> {
            self.device_token.as_ref().map(|v| v.as_str())
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
        pub fn build(&self) -> Result<Arc<super::api::Client>, AuthError> {
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
                reqwest::blocking::ClientBuilder::new().timeout(Duration::from_secs(10)).default_headers(headers);
            if let Some(proxy) = self.proxy.as_ref() {
                client_builder = client_builder.proxy(proxy.clone());
            }
            let client = client_builder.build().unwrap();
            let mut request = client
                .post("https://oauth.secure.pixiv.net/auth/token")
                .form(&formbody);

            let res = api::Client::ensure_json_response(request).unwrap();

            if res.get("has_error").is_none() {
                let mut response: Response =
                    serde_json::from_value(res["response"].clone()).unwrap();
                response.proxy = self.proxy.clone();
                Ok(response.build_client())
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
        Method, Proxy, Url, StatusCode,
    };
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    pub type ApiResult<T> = Result<T, ApiError>;
    pub type RequestResult<T> = Result<ApiResult<T>, dyn Error>;

    use crate::{
        error::ApiError,
        illust::{self},
        novel::{self},
        user::user, ClientBudiler,
    };

    lazy_static! {
        static ref BASE_URL: Url = Url::from_str("https://app-api.pixiv.net").unwrap();
    }

    use self::{
        api_illust::IllustApi, api_novel::NovelApi, api_search::SearchApi, api_user::UserApi,
    };

    use super::{
        auth::{Response, User},
        response::{BookmarkTags, Illusts},
    };
    use std::{collections::HashMap, sync::{Arc, Mutex}, str::FromStr, error::Error, cell::RefCell, time::Duration};

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

        use std::sync::{Arc, RwLock};

        use reqwest::Method;
        use serde::{Deserialize, Serialize};

        use crate::{client::response::{self, SearchAutoCompleteKeywords}, novel, user::{self, user::Preview}, preload::{Sort, Target}};

        use super::{ApiResult, Client};


        #[derive(Clone, Default, Debug)]
        pub struct SearchApi {
            pub(crate) client: Arc<RwLock<Option<Arc<Client>>>>,
        }
        impl SearchApi {
            pub fn autocomplete<T>(&self, word: T, merge_plain_keyword_results: bool) -> ApiResult<SearchAutoCompleteKeywords>
            where
                T: ToString,
            {
                let mut req = self
                    .client.read().unwrap()
                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/search/autocomplete")
                    .query(&[("word", word.to_string())])
                    .query(&[("merge_plain_keyword_results", merge_plain_keyword_results)]);
                Client::response(req)
            }
            pub fn autocomplete_v2<T>(&self, word: T, merge_plain_keyword_results: bool) -> ApiResult<response::Tags>
            where
                T: ToString,
            {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
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
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/search/user")
                    .query(&[("word", word.to_string())]);
                let mut ret: ApiResult<user::user::Previews> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }
            pub fn novel_popular_preview<T>(
                &self,
                word: T,
                sort: Sort,
                search_target: Target,
                include_translated_tag_results: bool,
                merge_plain_keyword_results: bool,
            ) -> ApiResult<response::Novels>
            where
                T: ToString,
            {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
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
            ) -> ApiResult<response::Novels>
            where
                T: ToString,
            {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
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
            ) -> ApiResult<response::Illusts>
            where
                T: ToString,
            {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
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
            ) -> ApiResult<response::Illusts>
            where
                T: ToString,
            {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
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
        use std::sync::{Arc, RwLock};

        use reqwest::Method;

        use crate::{client::{response, ApiNone}, illust, novel, user, preload::{Restrict, IllustType}};

        use super::{ApiResult, Client};

        #[derive(Clone, Default, Debug)]
        pub struct UserApi {
            pub(crate) client: Arc<RwLock<Option<Arc<Client>>>>,
        }
        impl UserApi {
            pub fn detail(&self, user_id: u64) -> ApiResult<user::user::Detail> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/user/detail")
                    .query(&[("user_id", user_id)]);
                Client::response(req)
            }

            pub fn mypixiv(&self, user_id: u64, offset: usize) -> ApiResult<user::user::Previews> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/user/mypixiv")
                    .query(&[
                        ("user_id", user_id.to_string()),
                        ("offset", offset.to_string()),
                    ]);
                let mut ret: ApiResult<user::user::Previews> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }

            pub fn illusts(
                &self,
                user_id: u64,
                tp: IllustType,
                offset: usize,
            ) -> ApiResult<response::Illusts> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/user/illusts")
                    .query(&[
                        ("user_id", user_id.to_string()),
                        ("type", tp.to_string()),
                        ("offset", offset.to_string()),
                    ]);
                let mut ret: ApiResult<response::Illusts> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }

            pub fn novels(&self, user_id: u64) -> ApiResult<novel::novel::Detail> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/user/novels")
                    .query(&[("user_id", user_id.to_string())]);
                let mut ret: ApiResult<novel::novel::Detail> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }
            pub fn bookmarks_novel(
                &self,
                user_id: u64,
                restrict: Restrict,
            ) -> ApiResult<novel::novel::Detail> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/user/bookmarks/novel")
                    .query(&[
                        ("user_id", user_id.to_string()),
                        ("restrict", restrict.to_string()),
                    ]);
                let mut ret: ApiResult<novel::novel::Detail> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }
            pub fn bookmark_illust_tags(
                &self,
                user_id: u64,
                restrict: Restrict,
            ) -> ApiResult<response::BookmarkTags> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
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
            ) -> ApiResult<response::BookmarkTags> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
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
            ) -> ApiResult<response::Illusts> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
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
                let mut ret: ApiResult<response::Illusts> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }
            pub fn follow_add(&self, user_id: u64, restrict: Restrict) -> ApiResult<ApiNone> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::POST, "/v1/user/follow/add")
                    .form(&params! {
                    "user_id" => user_id,
                    "restrict" => restrict,
                    });
                Client::response(req)
            }
            pub fn follow_delete(&self, user_id: u64) -> ApiResult<ApiNone> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
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
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/user/following")
                    .query(&[
                        ("user_id", user_id.to_string()),
                        ("restrict", restrict.to_string()),
                        ("offset", offset.to_string()),
                    ]);
                let mut ret: ApiResult<user::user::Previews> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }

            pub fn follower(&self, user_id: u64, offset: usize) -> ApiResult<user::user::Previews> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/user/follower")
                    .query(&[
                        ("user_id", user_id.to_string()),
                        ("offset", offset.to_string()),
                    ]);
                let mut ret: ApiResult<user::user::Previews> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }
        }
    }
    pub mod api_illust {

        use std::sync::{Arc, RwLock};

        use reqwest::Method;

        use crate::{client::{response, ApiNone}, illust, preload::{Restrict, RankingMode}};

        use super::{ApiResult, Client};

        #[derive(Clone, Default, Debug)]
        pub struct IllustApi {
            pub(crate) client: Arc<RwLock<Option<Arc<Client>>>>,
        }
        impl IllustApi {
            pub fn detail(&self, illust_id: u64) -> ApiResult<illust::illust::Detail> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/illust/detail")
                    .query(&[("illust_id", illust_id)]);
                Client::response(req)
            }
            pub fn bookmark_add(&self, illust_id: u64, restrict: Restrict) -> ApiResult<ApiNone> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::POST, "/v2/illust/bookmark/add")
                    .query(&[("illust_id", illust_id)])
                    .query(&[("restrict", restrict.to_string())]);
                Client::response(req)
            }
            pub fn bookmark_delete(&self, illust_id: u64) -> ApiResult<ApiNone> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
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
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v2/illust/bookmark/detail")
                    .query(&[("illust_id", illust_id)]);
                Client::response(req)
            }
            pub fn related(&self, illust_id: u64) -> ApiResult<response::Illusts> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v2/illust/related")
                    .query(&[("illust_id", illust_id)]);
                let mut ret: ApiResult<response::Illusts> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }
            pub fn ranking(&self, mode: RankingMode, date: Option<String>) -> ApiResult<response::Illusts> {
                let mut req = self.client.read().unwrap().as_ref().unwrap()
                    .request(Method::GET, "/v1/illust/ranking")
                    .query(&[("mode", mode.to_string())]);
                if let Some(date) = date {
                    req = req.query(&[("date", date)]);
                }
                let mut ret: ApiResult<response::Illusts> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }
            pub fn recommended(
                &self,
                include_ranking_illusts: bool,
            ) -> ApiResult<illust::illust::Recommended> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/illust/recommended")
                    .query(&[("include_ranking_illusts", include_ranking_illusts)]);
                let mut ret: ApiResult<illust::illust::Recommended> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }
            pub fn comments(
                &self,
                illust_id: u64,
                include_total_comments: bool,
            ) -> ApiResult<illust::comments::Detail> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/illust/comments")
                    .query(&[("illust_id", illust_id)])
                    .query(&[("include_total_comments", include_total_comments)]);
                let mut ret: ApiResult<illust::comments::Detail> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }

            pub fn comments_v2(&self, illust_id: u64) -> ApiResult<illust::comments::Detail> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v2/illust/comments")
                    .query(&[("illust_id", illust_id)]);
                let mut ret: ApiResult<illust::comments::Detail> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }

            pub fn comment_replies(&self, comment_id: u64) -> ApiResult<illust::comments::Detail> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/illust/comment/replies")
                    .query(&[("comment_id", comment_id)]);
                let mut ret: ApiResult<illust::comments::Detail> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }
            pub fn manga_recommended(
                &self,
                include_ranking_label: bool,
            ) -> ApiResult<illust::illust::Recommended> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/manga/recommended")
                    .query(&[("include_ranking_label", include_ranking_label)]);
                let mut ret: ApiResult<illust::illust::Recommended> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }

            pub fn ugoira_metadata(&self, illust_id: &str) -> ApiResult<illust::ugoira::Detail> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/ugoira/metadata")
                    .query(&[("illust_id", illust_id)]);
                Client::response(req)
            }
        }
    }
    pub mod api_novel {

        use std::sync::{Arc, RwLock};

        use reqwest::Method;

        use crate::{illust, novel, preload::{Restrict, RankingMode}, client::ApiNone};

        use super::{ApiResult, Client};

        #[derive(Clone, Default, Debug)]
        pub struct NovelApi {
            pub(crate) client: Arc<RwLock<Option<Arc<Client>>>>,
        }
        impl NovelApi {
            pub fn detail(&self, novel_id: u64) -> ApiResult<novel::novel::Detail> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v2/novel/detail")
                    .query(&[("novel_id", novel_id)]);
                let mut ret: ApiResult<novel::novel::Detail> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }
            pub fn bookmark_add(&self, novel_id: u64, restrict: Restrict) -> ApiResult<ApiNone> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::POST, "/v2/novel/bookmark/add")
                    .query(&[("novel_id", novel_id)])
                    .query(&[("restrict", restrict.to_string())]);
                Client::response(req)
            }
            pub fn bookmark_delete(&self, novel_id: u64) -> ApiResult<ApiNone> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
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
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v2/illust/bookmark/detail")
                    .query(&[("illust_id", illust_id)]);
                Client::response(req)
            }
            pub fn text(&self, novel_id: u64) -> ApiResult<novel::novel::Text> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/novel/text")
                    .query(&[("novel_id", novel_id)]);
                Client::response(req)
            }
            pub fn ranking(&self, mode: RankingMode, date: Option<String>) -> ApiResult<novel::novel::Detail> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/novel/ranking")
                    .query(&[("mode", mode.to_string())]);
                if let Some(date) = date {
                    req = req.query(&[("date", date)]);
                }
                let mut ret: ApiResult<novel::novel::Detail> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }
            pub fn recommended(
                &self,
                include_ranking_novels: bool,
            ) -> ApiResult<novel::novel::Recommended> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/novel/recommended")
                    .query(&[("include_ranking_novels", include_ranking_novels)]);
                let mut ret: ApiResult<novel::novel::Recommended> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }
            pub fn comments(
                &self,
                novel_id: u64,
                include_total_comments: bool,
            ) -> ApiResult<novel::novel::Detail> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/novel/comments")
                    .query(&[("novel_id", novel_id)])
                    .query(&[("include_total_comments", include_total_comments)]);
                let mut ret: ApiResult<novel::novel::Detail> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }

            pub fn comments_v2(&self, novel_id: u64) -> ApiResult<novel::comments::Detail> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v2/novel/comments")
                    .query(&[("novel_id", novel_id)]);
                let mut ret: ApiResult<novel::comments::Detail> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }

            pub fn comment_replies(&self, comment_id: u64) -> ApiResult<novel::comments::Detail> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/novel/comment/replies")
                    .query(&[("comment_id", comment_id)]);
                let mut ret: ApiResult<novel::comments::Detail> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }
            pub fn series(&self, series_id: u64) -> ApiResult<novel::series::SeriesNovelResponse> {
                let mut req = self
                    .client.read().unwrap()                    .as_ref()
                    .unwrap()
                    .request(Method::GET, "/v1/novel/series")
                    .query(&[("series_id", series_id)]);
                let mut ret: ApiResult<novel::series::SeriesNovelResponse> = Client::response(req);
                ret.map(|v| {
                    let mut v = v;
                    v.client = self.client.read().unwrap().clone();
                    v
                })
            }
        }
    }


    #[derive(Debug)]
    pub struct Client {
        access_token: Arc<Mutex<String>>,
        expires_in: u64,
        token_type: String,
        scope: String,
        refresh_token: String,
        userinfo: User,
        proxy: Option<Proxy>,
        pub client: Arc<reqwest::blocking::Client>,
        default_headers: HeaderMap,
        pub search: SearchApi,
        pub user: UserApi,
        pub novel: NovelApi,
        pub illust: IllustApi,
    }
    impl Response {
        pub fn build_client(self) -> Arc<Client> {
            let mut default_headers = HeaderMap::new();
            default_headers.insert(REFERER, "https://www.pixiv.net".parse().unwrap());
            default_headers.insert(USER_AGENT, "pixivdownloader/".parse().unwrap());
            let mut client_builder =
                reqwest::blocking::ClientBuilder::new().timeout(Duration::from_secs(10)).default_headers(default_headers.clone());
            if let Some(proxy) = self.proxy() {
                client_builder =
                    client_builder.proxy(Proxy::all("http://localhost:15777").unwrap());
            }
            let mut sf_arc = Arc::new(Client {
                client: Arc::new(client_builder.build().unwrap()),
                default_headers: default_headers,
                access_token: Arc::new(Mutex::new(self.access_token().to_string())),
                token_type: self.token_type().to_string(),
                expires_in: self.expires_in(),
                scope: self.scope().to_string(),
                refresh_token: self.refresh_token().to_string(),
                userinfo: self.user().clone(),
                proxy: self.proxy(),
                search: SearchApi::default(),
                user: UserApi::default(),
                novel: NovelApi::default(),
                illust: IllustApi::default(),
            });
            *sf_arc.search.client.write().unwrap() = Some(sf_arc.clone());
            *sf_arc.user.client.write().unwrap() = Some(sf_arc.clone());
            *sf_arc.novel.client.write().unwrap() = Some(sf_arc.clone());
            *sf_arc.illust.client.write().unwrap() = Some(sf_arc.clone());
            sf_arc
        }
    }
    impl Client {
        pub fn access_token(&self) -> String {
            self.access_token.lock().unwrap().clone()
        }
        fn expires_in(&self) -> u64 {
            self.expires_in
        }
        pub fn relogin(&self) -> bool {
            let mut builder = ClientBudiler::new();
            if let Some(v) = self.proxy.clone() {
                builder.proxy(v);
            }
            builder.refresh_token(self.refresh_token());
            if let Ok(res) = builder.build() {
                *self.access_token.lock().unwrap() = res.access_token().to_string();
                true
            } else {
                false
            }
        }
        pub(crate) fn request<'a>(&self, method: Method, path: &'a str) -> RequestBuilder {
            self.client.request(method, BASE_URL.join(path).unwrap())
                .bearer_auth(self.access_token())
        }
        pub(crate) fn no_auth_request<'a>(&self, method: Method, path: &'a str) -> RequestBuilder {
            self.client.request(method, BASE_URL.join(path).unwrap())
        }
        fn token_type(&self) -> &str {
            &self.token_type
        }
        fn scope(&self) -> &str {
            &self.scope
        }
        pub fn refresh_token(&self) -> &str {
            &self.refresh_token
        }
        pub fn userinfo(&self) -> &User {
            &self.userinfo
        }
        pub(crate) fn ensure_success_response(request: RequestBuilder) -> Result<reqwest::blocking::Response, reqwest::Error> {
            match request.try_clone().unwrap().send() {
                Ok(res) => {
                    Ok(res)
                    /*
                    match res.status() {
                        StatusCode::OK | StatusCode::FORBIDDEN | StatusCode:: => Ok(res),
                        code => panic!("UnexceptedStatuCode({}) for {}\n{:#?}", code, res.url(), res.headers()),
                    }
                    */
                }
                Err(err) => {
                    if err.is_connect() | err.is_timeout() | err.is_request() {
                        Self::ensure_success_response(request)
                    } else {
                        Err(err)
                    }
                }
            }
        }
        pub(crate) fn ensure_json_response(request: RequestBuilder) -> Result<Value, reqwest::Error> {
            let request_clone = request.try_clone().unwrap();
            let res_json: Value = match Self::ensure_success_response(request) {
                Ok(res) => match res.json() {
                    Ok(json_res) => {json_res},
                    Err(err) => {
                        if err.is_connect() | err.is_timeout() | err.is_request() {
                            Self::ensure_json_response(request_clone)?
                        } else {
                            return Err(err);
                        }
                    }
                },
                Err(err) => panic!("UnexceptedRequestError: {:#?}", err),
            };
            //println!("{:#?}", res_json);
            Ok(serde_json::from_value(res_json).unwrap())
        }
        pub(crate) fn response<T>(request: RequestBuilder) -> ApiResult<T>
        where
            T: for<'de> Deserialize<'de>,
        {
            let request_clone = request.try_clone().unwrap();
            let res_json: Value = match Self::ensure_success_response(request) {
                Ok(res) => match res.json() {
                    Ok(json_res) => {json_res},
                    Err(err) => {
                        return Self::response(request_clone);
                    }
                },
                Err(err) => panic!("UnexceptedRequestError: {:#?}", err),
            };
            //println!("{:#?}", res_json);
            if res_json.get("error").is_none() {
                Ok(serde_json::from_value(res_json).unwrap())
            } else {
                Err(serde_json::from_value(res_json).unwrap())
            }
        }
    }
}

pub mod response {
    use std::sync::Arc;

    use reqwest::Method;
    use serde::{Deserialize, Serialize};

    use crate::{
        illust::illust::{self, Tag},
        novel,
        traits::NextUrl,
    };

    use super::api::{Client, ApiResult};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Novels {
        novels: Vec<novel::novel::Novel>,
        next_url: Option<String>,
        search_span_limit: Option<u64>,
        #[serde(skip)]
        pub(crate) client: Option<Arc<Client>>,
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
        fn next_url(&self) -> Option<super::api::ApiResult<Self::Output>> {
            let mut ret: ApiResult<Self::Output> = Client::response(self.client.as_ref()?.request(Method::GET, self.next_url.as_ref()?));
            Some(ret.map(|v| {
                let mut v = v;
                v.client = self.client.clone();
                v
            }))
        }
    }
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Illusts {
        illusts: Vec<illust::Illust>,
        next_url: Option<String>,
        search_span_limit: Option<u64>,
        #[serde(skip)]
        pub(crate) client: Option<Arc<Client>>,
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
        fn next_url(&self) -> Option<super::api::ApiResult<Self::Output>> {
            let mut ret: ApiResult<Self::Output> = Client::response(self.client.as_ref()?.request(Method::GET, self.next_url.as_ref()?));
            Some(ret.map(|v| {
                let mut v = v;
                v.client = self.client.clone();
                v
            }))
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
        #[serde(skip)]
        pub(crate) client: Option<Arc<Client>>,
    }
    impl NextUrl for BookmarkTags {
        type Output = Self;
        fn has_next(&self) -> bool {
            self.next_url.is_some()
        }
        fn next_url(&self) -> Option<super::api::ApiResult<Self::Output>> {
            let mut ret: ApiResult<Self::Output> = Client::response(self.client.as_ref()?.request(Method::GET, self.next_url.as_ref()?));
            Some(ret.map(|v| {
                let mut v = v;
                v.client = self.client.clone();
                v
            }))
        }
    }
    impl BookmarkTags {
        pub fn bookmark_tags(&self) -> &Vec<Tag> {
            &self.bookmark_tags
        }
    }
}
