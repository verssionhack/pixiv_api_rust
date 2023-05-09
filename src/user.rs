use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfileImageUrls {
    medium: Option<String>,
}


pub mod user {
    use serde::{Deserialize, Serialize};

    use crate::{
        illust::{illust::Illust, ImageUrls},
        novel::novel::Novel,
        traits::NextUrl, preload::Publicity,
    };

    use super::{ProfileImageUrls};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Previews {
        user_previews: Vec<Preview>,
        next_url: Option<String>,
    }

    impl Previews {
        pub fn user_previews(&self) -> &Vec<Preview> {
            &self.user_previews
        }
    }
    impl NextUrl for Previews {
        type Output = Self;
        fn has_next(&self) -> bool {
            self.next_url.is_some()
        }
        fn next_url(&self) -> Option<Self::Output> {
            unimplemented!()
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Preview {
        user: User,
        illusts: Vec<Illust>,
        novels: Vec<Novel>,
        is_muted: bool,
    }
    impl Preview {
        pub fn user(&self) -> &User {
            &self.user
        }
        pub fn illusts(&self) -> &Vec<Illust> {
            &self.illusts
        }
        pub fn novels(&self) -> &Vec<Novel> {
            &self.novels
        }
        pub fn is_muted(&self) -> bool {
            self.is_muted
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Detail {
        user: User,
        profile: Profile,
        profile_publicity: ProfilePublicity,
        workspace: Workspace,
    }
    impl Detail {
        pub fn user(&self) -> &User {
            &self.user
        }
        pub fn profile(&self) -> &Profile {
            &self.profile
        }
        pub fn profile_publicity(&self) -> &ProfilePublicity {
            &self.profile_publicity
        }
        pub fn workspace(&self) -> &Workspace {
            &self.workspace
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct UserDetail {
        id: u64,
        name: String,
        account: String,
        profile_image_urls: ProfileImageUrls,
        comment: Option<String>,
    }
    impl UserDetail {
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
        pub fn comment(&self) -> Option<&String> {
            self.comment.as_ref()
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct User {
        id: u64,
        name: String,
        account: String,
        profile_image_urls: ProfileImageUrls,
        comment: Option<String>,
        #[serde(default)]
        is_followed: bool,
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
        pub fn comment(&self) -> Option<&String> {
            self.comment.as_ref()
        }
        pub fn is_followed(&self) -> bool {
            self.is_followed
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Profile {
        webpage: Option<String>,
        gender: String,
        birth: String,
        birth_day: String,
        birth_year: u64,
        region: String,
        address_id: u64,
        country_code: String,
        job: String,
        job_id: u64,
        total_follow_users: u64,
        total_mypixiv_users: u64,
        total_illusts: u64,
        total_manga: u64,
        total_novels: u64,
        total_illust_bookmarks_public: u64,
        total_illust_series: u64,
        total_novel_series: u64,
        background_image_url: Option<String>,
        twitter_account: String,
        twitter_url: Option<String>,
        pawoo_url: Option<String>,
        is_premium: bool,
        is_using_custom_profile_image: bool,
    }

    impl Profile {
        pub fn webpage(&self) -> Option<&String> {
            self.webpage.as_ref()
        }
        pub fn gender(&self) -> &str {
            &self.gender
        }
        pub fn birth(&self) -> &str {
            &self.birth
        }
        pub fn birth_day(&self) -> &str {
            &self.birth_day
        }
        pub fn birth_year(&self) -> u64 {
            self.birth_year
        }
        pub fn region(&self) -> &str {
            &self.region
        }
        pub fn address_id(&self) -> u64 {
            self.address_id
        }
        pub fn country_code(&self) -> &str {
            &self.country_code
        }
        pub fn job(&self) -> &str {
            &self.job
        }
        pub fn job_id(&self) -> u64 {
            self.job_id
        }
        pub fn total_follow_users(&self) -> u64 {
            self.total_follow_users
        }
        pub fn total_mypixiv_users(&self) -> u64 {
            self.total_mypixiv_users
        }
        pub fn total_illusts(&self) -> u64 {
            self.total_illusts
        }
        pub fn total_manga(&self) -> u64 {
            self.total_manga
        }
        pub fn total_novels(&self) -> u64 {
            self.total_novels
        }
        pub fn total_illust_bookmarks_public(&self) -> u64 {
            self.total_illust_bookmarks_public
        }
        pub fn total_illust_series(&self) -> u64 {
            self.total_illust_series
        }
        pub fn total_novel_series(&self) -> u64 {
            self.total_novel_series
        }
        pub fn background_image_url(&self) -> Option<&String> {
            self.background_image_url.as_ref()
        }
        pub fn twitter_account(&self) -> &str {
            &self.twitter_account
        }
        pub fn twitter_url(&self) -> Option<&String> {
            self.twitter_url.as_ref()
        }
        pub fn pawoo_url(&self) -> Option<&String> {
            self.pawoo_url.as_ref()
        }
        pub fn is_premium(&self) -> bool {
            self.is_premium
        }
        pub fn is_using_custom_profile_image(&self) -> bool {
            self.is_using_custom_profile_image
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ProfilePublicity {
        gender: Publicity,
        region: Publicity,
        birth_day: Publicity,
        birth_year: Publicity,
        job: Publicity,
        pawoo: bool,
    }

    impl ProfilePublicity {
        pub fn gender(&self) -> Publicity {
            self.gender
        }
        pub fn region(&self) -> Publicity {
            self.region
        }
        pub fn birth_day(&self) -> Publicity {
            self.birth_day
        }
        pub fn birth_year(&self) -> Publicity {
            self.birth_year
        }
        pub fn job(&self) -> Publicity {
            self.job
        }
        pub fn pawoo(&self) -> bool {
            self.pawoo
        }
    }
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Workspace {
        pc: String,
        monitor: String,
        tool: String,
        scanner: String,
        tablet: String,
        mouse: String,
        printer: String,
        desktop: String,
        music: String,
        desk: String,
        chair: String,
        comment: String,
        workspace_image_url: Option<String>,
    }
    impl Workspace {
        pub fn pc(&self) -> &str {
            &self.pc
        }
        pub fn monitor(&self) -> &str {
            &self.monitor
        }
        pub fn tool(&self) -> &str {
            &self.tool
        }
        pub fn scanner(&self) -> &str {
            &self.scanner
        }
        pub fn tablet(&self) -> &str {
            &self.tablet
        }
        pub fn mouse(&self) -> &str {
            &self.mouse
        }
        pub fn printer(&self) -> &str {
            &self.printer
        }
        pub fn desktop(&self) -> &str {
            &self.desktop
        }
        pub fn music(&self) -> &str {
            &self.music
        }
        pub fn desk(&self) -> &str {
            &self.desk
        }
        pub fn chair(&self) -> &str {
            &self.chair
        }
        pub fn comment(&self) -> &str {
            &self.comment
        }
        pub fn workspace_image_url(&self) -> Option<&String> {
            self.workspace_image_url.as_ref()
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ProfilePresets {
        addresses: Vec<Address>,
        countries: Vec<Country>,
        jobs: Vec<Job>,
        default_profile_image_urls: ImageUrls,
    }
    impl ProfilePresets {
        pub fn addresses(&self) -> &Vec<Address> {
            &self.addresses
        }
        pub fn countries(&self) -> &Vec<Country> {
            &self.countries
        }
        pub fn jobs(&self) -> &Vec<Job> {
            &self.jobs
        }
        pub fn default_profile_image_urls(&self) -> &ImageUrls {
            &self.default_profile_image_urls
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Address {
        id: u64,
        name: String,
        is_global: bool,
    }
    impl Address {
        pub fn id(&self) -> &u64 {
            &self.id
        }
        pub fn name(&self) -> &String {
            &self.name
        }
        pub fn is_global(&self) -> &bool {
            &self.is_global
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Country {
        code: String,
        name: String,
    }
    impl Country {
        pub fn code(&self) -> &String {
            &self.code
        }
        pub fn name(&self) -> &String {
            &self.name
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Job {
        id: u64,
        name: String,
    }
    impl Job {
        pub fn id(&self) -> &u64 {
            &self.id
        }
        pub fn name(&self) -> &String {
            &self.name
        }
    }
}
