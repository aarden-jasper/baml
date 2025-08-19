use std::collections::HashSet;

use baml_types::{baml_value::TypeLookups, ir_type::TypeRPC};
use ir_hasher::TypeAliasDefinition;

use crate::ir::repr::{Node, TypeAlias};

pub struct TypeAliasAdapter {
    pub type_alias: TypeAlias,
    pub converted_type: TypeRPC,
}

impl TypeAliasAdapter {
    pub fn new(type_alias_node: &Node<TypeAlias>, lookup: &impl TypeLookups) -> Self {
        let converted_type = type_alias_node.elem.r#type.elem.to_rpc_type(lookup);

        Self {
            type_alias: type_alias_node.elem.clone(),
            converted_type,
        }
    }
}

impl TypeAliasDefinition for TypeAliasAdapter {
    fn name(&self) -> &str {
        &self.type_alias.name
    }

    fn r#type(&self) -> &TypeRPC {
        &self.converted_type
    }

    fn dependencies(&self) -> HashSet<String> {
        self.converted_type.immediate_dependencies()
    }
}
