use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AnyDispatch
{
    Direct(String),
    Conditional(ConditionalDispatch),
    Protocol(ProtocolDispatch),
    Mime(MimeDispatch),
    Extension(ExtensionDispatch),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum OneOrManyDispatch
{
    One(AnyDispatch),
    Many(Vec<AnyDispatch>),
}

#[derive(Debug, Deserialize)]
pub struct ConditionalDispatch
{
    #[serde(rename = "use")]
    pub use_elem: String,
    pub contains: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ProtocolDispatch
{
    pub protocol: BTreeMap<String, OneOrManyDispatch>,
}

#[derive(Debug, Deserialize)]
pub struct MimeDispatch
{
    pub mime: BTreeMap<String, OneOrManyDispatch>,
}

#[derive(Debug, Deserialize)]
pub struct ExtensionDispatch
{
    pub extension: BTreeMap<String, OneOrManyDispatch>,
}
