use anyhow::Result;
use baml_rpc::{AstNodeId, HashPart, NodeHash};

#[derive(Clone, Debug)]
pub enum SignatureType {
    Class,
    Function,
    Enum,
    TypeAlias,
    Client,
    RetryPolicy,
    TemplateString,
}

#[derive(Clone, Debug)]
pub struct Signature {
    pub r#type: SignatureType,
    pub display_name: String,
    pub hashes: NodeHash,
    pub dependencies: Vec<String>,
}

impl Signature {
    pub fn new(
        r#type: SignatureType,
        name: &str,
        all_tops: &super::shallow_hash::AllTops,
    ) -> Result<Self> {
        let implementation_hash = all_tops.hash_implementation(name)?;
        let interface_hash = all_tops.hash_interface(name)?;
        let dependencies = all_tops.get_dependencies(name)?;

        Ok(Self {
            r#type,
            display_name: name.to_string(),
            hashes: NodeHash::CompileTimeOnly(HashPart::new(interface_hash, implementation_hash)),
            dependencies,
        })
    }

    pub fn to_ast_node(&self) -> AstNodeId {
        match self.r#type {
            SignatureType::Class => AstNodeId::new_class(self.display_name.clone(), self.hashes),
            SignatureType::Function => {
                AstNodeId::new_function(self.display_name.clone(), self.hashes)
            }
            SignatureType::Enum => AstNodeId::new_enum(self.display_name.clone(), self.hashes),
            SignatureType::TypeAlias => {
                AstNodeId::new_type_alias(self.display_name.clone(), self.hashes)
            }
            SignatureType::Client => AstNodeId::new_client(self.display_name.clone(), self.hashes),
            SignatureType::RetryPolicy => {
                AstNodeId::new_retry_policy(self.display_name.clone(), self.hashes)
            }
            SignatureType::TemplateString => todo!(),
        }
    }

    pub fn dependency_names(&self) -> &Vec<String> {
        &self.dependencies
    }
}
