use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(from = "u64")]
pub enum Restrict {
    Public = 0,
    Private,
    MyPixiv,
}

impl From<u64> for Restrict {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Public,
            1 => Self::MyPixiv,
            2 => Self::Private,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Restrict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{:?}", self).to_lowercase())
    }
}

#[derive(Clone, Serialize, Deserialize, Copy, Debug)]
#[serde(rename_all="snake_case")]
#[serde(from = "u64")]
pub enum XRestrict {
    None = 0,
    R18,
    R18g,
}

impl From<u64> for XRestrict {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::None,
            1 => Self::R18,
            2 => Self::R18g,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for XRestrict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{:?}", self).to_lowercase())
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PrivacyPolicy {
    version: Option<String>,
    message: Option<String>,
    url: Option<String>,
}


#[derive(Clone, Serialize, Deserialize, Copy, Debug)]
#[serde(rename_all="snake_case")]
pub enum Filter {
    ForIos,
    ForAndroid
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    Password,
    RefreshToken,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Sort {
    DateDesc,
    DateAsc,
}

impl ToString for Sort {
    fn to_string(&self) -> String {
        match self {
            Self::DateAsc => "date_asc",
            Self::DateDesc => "date_desc",
        }
        .to_string()
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Target {
    PartialMatchForTags,
    ExactMatchForTags,
    TitleAndCaption,
}

impl ToString for Target {
    fn to_string(&self) -> String {
        match self {
            Self::PartialMatchForTags => "partial_match_for_tags",
            Self::ExactMatchForTags => "exact_match_for_tags",
            Self::TitleAndCaption => "title_and_caption",
        }
        .to_string()
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum RankingMode {
    Day,
    Week,
    Month,
    DayMale,
    DayFemale,
    WeekOriginal,
    WeekRookie,
    DayManga,
    WeekRookieManga,
    WeekManga,
}

impl ToString for RankingMode {
    fn to_string(&self) -> String {
        match self {
            Self::Day => "day",
            Self::Week => "week",
            Self::Month => "month",
            Self::DayMale => "day_male",
            Self::DayFemale => "day_female",
            Self::WeekOriginal => "week_original",
            Self::WeekRookie => "week_rookie",
            Self::DayManga => "day_manga",
            Self::WeekRookieManga => "week_rookie_manga",
            Self::WeekManga => "week_manga",
        }
        .to_string()
    }
}

#[derive(Clone, Serialize, Deserialize, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum IllustType {
    Illust,
    Ugoira,
    Manga,
}

impl ToString for IllustType {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_ascii_lowercase()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Publicity {
    Public,
    Private,
}