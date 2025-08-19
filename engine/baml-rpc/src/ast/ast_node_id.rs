use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(
    Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Clone, TS, strum::Display, strum::EnumString,
)]
#[ts(export)]
#[serde(rename_all = "snake_case")]
// To ensure that from_str works correctly, we need to use snake_case for the enum values
#[strum(serialize_all = "snake_case")]
pub enum AstNodeIdType {
    Ast,
    Function,
    TypeAlias,
    Enum,
    Class,
    Client,
    RetryPolicy,
    TemplateString,
}

// u64 are serde'd as Strings in this type, because we use this type directly in
// clickhouse, and clickhouse u64 behavior is _weird_. See
// output_format_json_quote_64bit_integers,
// https://github.com/ClickHouse/ClickHouse/issues/114 and
// https://gloo-global.slack.com/archives/C085SCFUETC/p1748989355944309
// #[serde_as]
#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Clone, TS)]
#[ts(export)]
pub struct AstNodeId {
    type_name: AstNodeIdType,
    name: String,
    hash: NodeHash,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Clone, TS, Copy)]
#[ts(export)]
pub enum NodeHash {
    CompileTimeOnly(HashPart),
    CompileTimeAndRuntime(HashPart, HashPart),
    RuntimeOnly(HashPart),
}

impl NodeHash {
    fn with_splitter(&self, splitter: &str) -> String {
        format!(
            "{}{splitter}{}",
            self.compile_time().with_splitter(splitter),
            self.runtime().with_splitter(splitter)
        )
    }

    fn compile_time(&self) -> &HashPart {
        match self {
            NodeHash::CompileTimeOnly(hash_part) => hash_part,
            NodeHash::CompileTimeAndRuntime(hash_part, _) => hash_part,
            NodeHash::RuntimeOnly(_) => &ZERO_HASH,
        }
    }

    fn runtime(&self) -> &HashPart {
        match self {
            NodeHash::CompileTimeOnly(_) => &ZERO_HASH,
            NodeHash::CompileTimeAndRuntime(_, hash_part) => hash_part,
            NodeHash::RuntimeOnly(hash_part) => hash_part,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Clone, TS, Copy)]
#[ts(export)]
pub struct HashPart {
    #[ts(type = "string")]
    #[serde(
        serialize_with = "serialize_u64_to_string",
        deserialize_with = "deserialize_string_to_u64"
    )]
    interface_hash: u64,
    #[ts(type = "string")]
    #[serde(
        serialize_with = "serialize_u64_to_string",
        deserialize_with = "deserialize_string_to_u64"
    )]
    impl_hash: u64,
}

const ZERO_HASH: HashPart = HashPart::zero();

impl HashPart {
    pub fn new(interface_hash: u64, impl_hash: u64) -> Self {
        Self {
            interface_hash,
            impl_hash,
        }
    }

    const fn zero() -> Self {
        Self {
            interface_hash: 0,
            impl_hash: 0,
        }
    }

    fn with_splitter(&self, splitter: &str) -> String {
        format!("{}{splitter}{}", self.interface_hash, self.impl_hash)
    }

    pub fn interface_hash(&self) -> u64 {
        self.interface_hash
    }

    pub fn impl_hash(&self) -> u64 {
        self.impl_hash
    }
}

impl AstNodeId {
    pub fn type_name(&self) -> &str {
        match self.type_name {
            AstNodeIdType::Ast => "ast",
            AstNodeIdType::Function => "function",
            AstNodeIdType::TypeAlias => "type_alias",
            AstNodeIdType::Enum => "enum",
            AstNodeIdType::Class => "class",
            AstNodeIdType::Client => "client",
            AstNodeIdType::RetryPolicy => "retry_policy",
            AstNodeIdType::TemplateString => "template_string",
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn compile_time(&self) -> &HashPart {
        self.hash.compile_time()
    }
    pub fn runtime(&self) -> &HashPart {
        self.hash.runtime()
    }

    pub fn new_ast(hash: NodeHash) -> Self {
        Self {
            type_name: AstNodeIdType::Ast,
            name: "root".to_string(),
            hash,
        }
    }

    pub fn new_type_alias(name: String, hash: NodeHash) -> Self {
        Self {
            type_name: AstNodeIdType::TypeAlias,
            name,
            hash,
        }
    }

    pub fn new_function(name: String, hash: NodeHash) -> Self {
        Self {
            type_name: AstNodeIdType::Function,
            name,
            hash,
        }
    }

    pub fn new_enum(name: String, hash: NodeHash) -> Self {
        Self {
            type_name: AstNodeIdType::Enum,
            name,
            hash,
        }
    }

    pub fn new_class(name: String, hash: NodeHash) -> Self {
        Self {
            type_name: AstNodeIdType::Class,
            name,
            hash,
        }
    }

    pub fn new_client(name: String, hash: NodeHash) -> Self {
        Self {
            type_name: AstNodeIdType::Client,
            name,
            hash,
        }
    }

    pub fn new_retry_policy(name: String, hash: NodeHash) -> Self {
        Self {
            type_name: AstNodeIdType::RetryPolicy,
            name,
            hash,
        }
    }

    pub fn new_template_string(name: String, hash: NodeHash) -> Self {
        Self {
            type_name: AstNodeIdType::TemplateString,
            name,
            hash,
        }
    }
}

impl std::fmt::Display for AstNodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}##{}##{}",
            self.type_name,
            self.name,
            self.hash.with_splitter("##")
        )
    }
}

impl std::str::FromStr for AstNodeId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("##").collect::<Vec<_>>();

        let comp_time_interface_hash = match parts[2].parse() {
            Ok(interface_hash) => interface_hash,
            Err(_) => return Err(anyhow::anyhow!("Invalid unique id: {}", s)),
        };
        let comp_time_impl_hash = match parts[3].parse() {
            Ok(impl_hash) => impl_hash,
            Err(_) => return Err(anyhow::anyhow!("Invalid unique id: {}", s)),
        };
        let runtime_interface_hash = match parts[4].parse() {
            Ok(interface_hash) => interface_hash,
            Err(_) => return Err(anyhow::anyhow!("Invalid unique id: {}", s)),
        };
        let runtime_impl_hash = match parts[5].parse() {
            Ok(impl_hash) => impl_hash,
            Err(_) => return Err(anyhow::anyhow!("Invalid unique id: {}", s)),
        };

        fn match_hash_parts(interface_hash: u64, impl_hash: u64) -> Option<HashPart> {
            if interface_hash == 0 && impl_hash == 0 {
                None
            } else {
                Some(HashPart {
                    interface_hash,
                    impl_hash,
                })
            }
        }

        let compile_time = match_hash_parts(comp_time_interface_hash, comp_time_impl_hash);
        let runtime = match_hash_parts(runtime_interface_hash, runtime_impl_hash);

        let hash = match (compile_time, runtime) {
            (Some(c), Some(r)) => NodeHash::CompileTimeAndRuntime(c, r),
            (Some(c), None) => NodeHash::CompileTimeOnly(c),
            (None, Some(r)) => NodeHash::RuntimeOnly(r),
            (None, None) => anyhow::bail!("Invalid unique id: {}", s),
        };

        Ok(AstNodeId {
            type_name: parts[0].parse::<AstNodeIdType>()?,
            name: parts[1].to_string(),
            hash,
        })
    }
}

// Helper function to deserialize string to u64
fn deserialize_string_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNum {
        String(String),
        Num(u64),
    }

    match StringOrNum::deserialize(deserializer)? {
        StringOrNum::String(s) => s.parse::<u64>().map_err(serde::de::Error::custom),
        StringOrNum::Num(i) => Ok(i),
    }
}

// Helper function to serialize u64 to string
fn serialize_u64_to_string<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}
