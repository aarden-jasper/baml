use std::collections::HashSet;

use baml_types::ir_type::TypeRPC;

use crate::interfaces::ShallowSignature;

/// Find some way to hash the type alias

pub trait TypeAliasDefinition {
    fn name(&self) -> &str;
    fn r#type(&self) -> &TypeRPC;
    fn dependencies(&self) -> HashSet<String>;
}

pub struct TopTypeAliasDefinition<'a>(&'a dyn TypeAliasDefinition);

impl<'a> TopTypeAliasDefinition<'a> {
    pub fn new(definition: &'a dyn TypeAliasDefinition) -> Self {
        Self(definition)
    }
}

impl ShallowSignature for TopTypeAliasDefinition<'_> {
    fn shallow_hash_prefix(&self) -> &'static str {
        "type_alias"
    }

    fn shallow_interface_hash(&self) -> impl std::hash::Hash {
        TypeAliasInterface(self.0)
    }

    fn unsorted_interface_dependencies(&self) -> HashSet<String> {
        self.0.dependencies()
    }

    fn shallow_implementation_hash(&self) -> Option<impl std::hash::Hash> {
        Some(TypeAliasImplementation(self.0))
    }
}

struct TypeAliasInterface<'a, T: TypeAliasDefinition + ?Sized>(pub(crate) &'a T);

impl<'a, T: TypeAliasDefinition + ?Sized> std::hash::Hash for TypeAliasInterface<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.name().hash(state);
        self.0.r#type().shallow_interface_hash().hash(state);
    }
}

struct TypeAliasImplementation<'a, T: TypeAliasDefinition + ?Sized>(pub(crate) &'a T);

impl<'a, T: TypeAliasDefinition + ?Sized> std::hash::Hash for TypeAliasImplementation<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        TypeAliasInterface(self.0).hash(state);
        self.0.r#type().shallow_implementation_hash().hash(state);
    }
}
