#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Asset {
    Variant0(String),
    Variant1(PlatformAsset),
}
impl From<&Asset> for Asset {
    fn from(value: &Asset) -> Self {
        value.clone()
    }
}
impl From<PlatformAsset> for Asset {
    fn from(value: PlatformAsset) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GitReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(
        rename = "useLatestRelease",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_latest_release: Option<bool>,
}
impl From<&GitReference> for GitReference {
    fn from(value: &GitReference) -> Self {
        value.clone()
    }
}
impl GitReference {
    pub fn builder() -> builder::GitReference {
        builder::GitReference::default()
    }
}
#[doc = "Package definition schema for Hollow Knight mods and modpacks"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HollowKnightPackageDef {
    pub assets: Vec<Asset>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<References>,
    pub description: String,
    #[serde(
        rename = "devDependencies",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dev_dependencies: Option<References>,
    pub name: String,
    pub repository: Link,
}
impl From<&HollowKnightPackageDef> for HollowKnightPackageDef {
    fn from(value: &HollowKnightPackageDef) -> Self {
        value.clone()
    }
}
impl HollowKnightPackageDef {
    pub fn builder() -> builder::HollowKnightPackageDef {
        builder::HollowKnightPackageDef::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Link(pub String);
impl std::ops::Deref for Link {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Link> for String {
    fn from(value: Link) -> Self {
        value.0
    }
}
impl From<&Link> for Link {
    fn from(value: &Link) -> Self {
        value.clone()
    }
}
impl From<String> for Link {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for Link {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for Link {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LinkReference {
    pub link: Link,
}
impl From<&LinkReference> for LinkReference {
    fn from(value: &LinkReference) -> Self {
        value.clone()
    }
}
impl LinkReference {
    pub fn builder() -> builder::LinkReference {
        builder::LinkReference::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ModlinksReference {
    #[serde(
        rename = "useLatestPublished",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_latest_published: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl From<&ModlinksReference> for ModlinksReference {
    fn from(value: &ModlinksReference) -> Self {
        value.clone()
    }
}
impl ModlinksReference {
    pub fn builder() -> builder::ModlinksReference {
        builder::ModlinksReference::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Platform {
    #[serde(rename = "win32")]
    Win32,
    #[serde(rename = "macos")]
    Macos,
    #[serde(rename = "linux")]
    Linux,
}
impl From<&Platform> for Platform {
    fn from(value: &Platform) -> Self {
        value.clone()
    }
}
impl ToString for Platform {
    fn to_string(&self) -> String {
        match *self {
            Self::Win32 => "win32".to_string(),
            Self::Macos => "macos".to_string(),
            Self::Linux => "linux".to_string(),
        }
    }
}
impl std::str::FromStr for Platform {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "win32" => Ok(Self::Win32),
            "macos" => Ok(Self::Macos),
            "linux" => Ok(Self::Linux),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for Platform {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Platform {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Platform {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PlatformAsset {
    pub path: String,
    pub platform: Platform,
}
impl From<&PlatformAsset> for PlatformAsset {
    fn from(value: &PlatformAsset) -> Self {
        value.clone()
    }
}
impl PlatformAsset {
    pub fn builder() -> builder::PlatformAsset {
        builder::PlatformAsset::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceDef {
    #[serde(
        rename = "alternateInstallName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub alternate_install_name: Option<String>,
    #[serde(rename = "fileType", default = "defaults::reference_def_file_type")]
    pub file_type: ReferenceDefFileType,
    #[serde(rename = "ref")]
    pub ref_: ReferenceDefRef,
}
impl From<&ReferenceDef> for ReferenceDef {
    fn from(value: &ReferenceDef) -> Self {
        value.clone()
    }
}
impl ReferenceDef {
    pub fn builder() -> builder::ReferenceDef {
        builder::ReferenceDef::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ReferenceDefFileType {
    #[serde(rename = "zip")]
    Zip,
    #[serde(rename = "dll")]
    Dll,
    #[serde(rename = "infer")]
    Infer,
}
impl From<&ReferenceDefFileType> for ReferenceDefFileType {
    fn from(value: &ReferenceDefFileType) -> Self {
        value.clone()
    }
}
impl ToString for ReferenceDefFileType {
    fn to_string(&self) -> String {
        match *self {
            Self::Zip => "zip".to_string(),
            Self::Dll => "dll".to_string(),
            Self::Infer => "infer".to_string(),
        }
    }
}
impl std::str::FromStr for ReferenceDefFileType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "zip" => Ok(Self::Zip),
            "dll" => Ok(Self::Dll),
            "infer" => Ok(Self::Infer),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ReferenceDefFileType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ReferenceDefFileType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ReferenceDefFileType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for ReferenceDefFileType {
    fn default() -> Self {
        ReferenceDefFileType::Infer
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ReferenceDefRef {
    GitReference(GitReference),
    ModlinksReference(ModlinksReference),
    LinkReference(LinkReference),
}
impl From<&ReferenceDefRef> for ReferenceDefRef {
    fn from(value: &ReferenceDefRef) -> Self {
        value.clone()
    }
}
impl From<GitReference> for ReferenceDefRef {
    fn from(value: GitReference) -> Self {
        Self::GitReference(value)
    }
}
impl From<ModlinksReference> for ReferenceDefRef {
    fn from(value: ModlinksReference) -> Self {
        Self::ModlinksReference(value)
    }
}
impl From<LinkReference> for ReferenceDefRef {
    fn from(value: LinkReference) -> Self {
        Self::LinkReference(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum References {
    Variant0(Vec<String>),
    Variant1 {
        #[serde(flatten)]
        extra: std::collections::HashMap<String, StringVersion>,
    },
    Variant2 {
        #[serde(flatten)]
        extra: std::collections::HashMap<String, ReferenceDef>,
    },
}
impl From<&References> for References {
    fn from(value: &References) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for References {
    fn from(value: Vec<String>) -> Self {
        Self::Variant0(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StringVersion(pub String);
impl std::ops::Deref for StringVersion {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<StringVersion> for String {
    fn from(value: StringVersion) -> Self {
        value.0
    }
}
impl From<&StringVersion> for StringVersion {
    fn from(value: &StringVersion) -> Self {
        value.clone()
    }
}
impl From<String> for StringVersion {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for StringVersion {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ToString for StringVersion {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
mod builder {
    pub struct GitReference {
        asset: Result<Option<String>, String>,
        tag: Result<Option<String>, String>,
        use_latest_release: Result<Option<bool>, String>,
    }
    impl Default for GitReference {
        fn default() -> Self {
            Self {
                asset: Ok(Default::default()),
                tag: Ok(Default::default()),
                use_latest_release: Ok(Default::default()),
            }
        }
    }
    impl GitReference {
        pub fn asset<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.asset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for asset: {}", e));
            self
        }
        pub fn tag<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.tag = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tag: {}", e));
            self
        }
        pub fn use_latest_release<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.use_latest_release = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for use_latest_release: {}",
                    e
                )
            });
            self
        }
    }
    impl std::convert::TryFrom<GitReference> for super::GitReference {
        type Error = String;
        fn try_from(value: GitReference) -> Result<Self, String> {
            Ok(Self {
                asset: value.asset?,
                tag: value.tag?,
                use_latest_release: value.use_latest_release?,
            })
        }
    }
    pub struct HollowKnightPackageDef {
        assets: Result<Vec<super::Asset>, String>,
        author: Result<Option<String>, String>,
        dependencies: Result<Option<super::References>, String>,
        description: Result<String, String>,
        dev_dependencies: Result<Option<super::References>, String>,
        name: Result<String, String>,
        repository: Result<super::Link, String>,
    }
    impl Default for HollowKnightPackageDef {
        fn default() -> Self {
            Self {
                assets: Err("no value supplied for assets".to_string()),
                author: Ok(Default::default()),
                dependencies: Ok(Default::default()),
                description: Err("no value supplied for description".to_string()),
                dev_dependencies: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                repository: Err("no value supplied for repository".to_string()),
            }
        }
    }
    impl HollowKnightPackageDef {
        pub fn assets<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Asset>>,
            T::Error: std::fmt::Display,
        {
            self.assets = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for assets: {}", e));
            self
        }
        pub fn author<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.author = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for author: {}", e));
            self
        }
        pub fn dependencies<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::References>>,
            T::Error: std::fmt::Display,
        {
            self.dependencies = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dependencies: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn dev_dependencies<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::References>>,
            T::Error: std::fmt::Display,
        {
            self.dev_dependencies = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for dev_dependencies: {}",
                    e
                )
            });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn repository<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Link>,
            T::Error: std::fmt::Display,
        {
            self.repository = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for repository: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<HollowKnightPackageDef> for super::HollowKnightPackageDef {
        type Error = String;
        fn try_from(value: HollowKnightPackageDef) -> Result<Self, String> {
            Ok(Self {
                assets: value.assets?,
                author: value.author?,
                dependencies: value.dependencies?,
                description: value.description?,
                dev_dependencies: value.dev_dependencies?,
                name: value.name?,
                repository: value.repository?,
            })
        }
    }
    pub struct LinkReference {
        link: Result<super::Link, String>,
    }
    impl Default for LinkReference {
        fn default() -> Self {
            Self {
                link: Err("no value supplied for link".to_string()),
            }
        }
    }
    impl LinkReference {
        pub fn link<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Link>,
            T::Error: std::fmt::Display,
        {
            self.link = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for link: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<LinkReference> for super::LinkReference {
        type Error = String;
        fn try_from(value: LinkReference) -> Result<Self, String> {
            Ok(Self { link: value.link? })
        }
    }
    pub struct ModlinksReference {
        use_latest_published: Result<Option<bool>, String>,
        version: Result<Option<String>, String>,
    }
    impl Default for ModlinksReference {
        fn default() -> Self {
            Self {
                use_latest_published: Ok(Default::default()),
                version: Ok(Default::default()),
            }
        }
    }
    impl ModlinksReference {
        pub fn use_latest_published<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.use_latest_published = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for use_latest_published: {}",
                    e
                )
            });
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ModlinksReference> for super::ModlinksReference {
        type Error = String;
        fn try_from(value: ModlinksReference) -> Result<Self, String> {
            Ok(Self {
                use_latest_published: value.use_latest_published?,
                version: value.version?,
            })
        }
    }
    pub struct PlatformAsset {
        path: Result<String, String>,
        platform: Result<super::Platform, String>,
    }
    impl Default for PlatformAsset {
        fn default() -> Self {
            Self {
                path: Err("no value supplied for path".to_string()),
                platform: Err("no value supplied for platform".to_string()),
            }
        }
    }
    impl PlatformAsset {
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn platform<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Platform>,
            T::Error: std::fmt::Display,
        {
            self.platform = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for platform: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<PlatformAsset> for super::PlatformAsset {
        type Error = String;
        fn try_from(value: PlatformAsset) -> Result<Self, String> {
            Ok(Self {
                path: value.path?,
                platform: value.platform?,
            })
        }
    }
    pub struct ReferenceDef {
        alternate_install_name: Result<Option<String>, String>,
        file_type: Result<super::ReferenceDefFileType, String>,
        ref_: Result<super::ReferenceDefRef, String>,
    }
    impl Default for ReferenceDef {
        fn default() -> Self {
            Self {
                alternate_install_name: Ok(Default::default()),
                file_type: Ok(super::defaults::reference_def_file_type()),
                ref_: Err("no value supplied for ref_".to_string()),
            }
        }
    }
    impl ReferenceDef {
        pub fn alternate_install_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.alternate_install_name = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for alternate_install_name: {}",
                    e
                )
            });
            self
        }
        pub fn file_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ReferenceDefFileType>,
            T::Error: std::fmt::Display,
        {
            self.file_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_type: {}", e));
            self
        }
        pub fn ref_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ReferenceDefRef>,
            T::Error: std::fmt::Display,
        {
            self.ref_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ref_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ReferenceDef> for super::ReferenceDef {
        type Error = String;
        fn try_from(value: ReferenceDef) -> Result<Self, String> {
            Ok(Self {
                alternate_install_name: value.alternate_install_name?,
                file_type: value.file_type?,
                ref_: value.ref_?,
            })
        }
    }
}
mod defaults {
    pub(super) fn reference_def_file_type() -> super::ReferenceDefFileType {
        super::ReferenceDefFileType::Infer
    }
}
