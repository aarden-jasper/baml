use std::collections::HashSet;

use baml_types::{ir_type::TypeRPC, StringOr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefinitionSource {
    // Defined purely at compile time
    CompileTime,
    // Defined purely at runtime
    Runtime,
    // Defined at compile time, but overridden at runtime
    OverriddenAtRuntime,
}

pub trait ShallowSignature {
    fn shallow_hash_prefix(&self) -> &'static str;
    fn shallow_interface_hash(&self) -> impl std::hash::Hash;
    fn unsorted_interface_dependencies(&self) -> HashSet<String>;
    fn interface_dependencies(&self) -> Vec<String> {
        let mut deps: Vec<String> = self.unsorted_interface_dependencies().into_iter().collect();
        deps.sort();
        deps
    }
    fn shallow_implementation_hash(&self) -> impl std::hash::Hash;
    fn unsorted_implementation_dependencies(&self) -> HashSet<String> {
        self.unsorted_interface_dependencies()
    }
    fn implementation_dependencies(&self) -> Vec<String> {
        let mut deps: Vec<String> = self
            .unsorted_implementation_dependencies()
            .into_iter()
            .collect();
        deps.sort();
        deps
    }
}

pub(crate) trait TypedObject {
    fn name(&self) -> &str;
    fn r#type(&self) -> &TypeRPC;
    fn definition_source(&self) -> DefinitionSource;
}

pub(crate) trait LLMRenderable {
    fn alias(&self) -> Option<&StringOr>;
    fn description(&self) -> Option<&StringOr>;
    fn skip(&self) -> bool;
}

pub(crate) struct TypedObjectInterface<'a, T: TypedObject + ?Sized>(pub(crate) &'a T);

impl<'a, T: TypedObject + ?Sized> std::hash::Hash for TypedObjectInterface<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.name().hash(state);
        self.0.r#type().hash(state);
        match self.0.definition_source() {
            DefinitionSource::CompileTime => "compile_time".hash(state),
            DefinitionSource::Runtime => "runtime".hash(state),
            DefinitionSource::OverriddenAtRuntime => "overridden_at_runtime".hash(state),
        }
    }
}

pub(crate) struct TypedObjectImplementation<'a, T: TypedObject + ?Sized>(pub(crate) &'a T);

impl<'a, T: TypedObject + ?Sized> std::hash::Hash for TypedObjectImplementation<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Nothing special to hash for the implementation
        TypedObjectInterface(self.0).hash(state);
    }
}

pub(crate) struct LLMRenderableInterface<'a, T: LLMRenderable + TypedObject + ?Sized>(
    pub(crate) &'a T,
);

impl<'a, T: LLMRenderable + TypedObject + ?Sized> std::hash::Hash
    for LLMRenderableInterface<'a, T>
{
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
        // to the make the compiler happy we do a no-op
        self.0;
        // But this function is intentionally empty
    }
}

pub(crate) struct LLMRenderableImplementation<'a, T: LLMRenderable + TypedObject + ?Sized>(
    pub(crate) &'a T,
);

impl<'a, T: LLMRenderable + TypedObject + ?Sized> std::hash::Hash
    for LLMRenderableImplementation<'a, T>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Nothing special to hash for the implementation
        LLMRenderableInterface(self.0).hash(state);
        let alias = self.0.alias().and_then(|alias| {
            if let StringOr::Value(val) = alias {
                if val.as_str() == self.0.name() || val.as_str() == "" {
                    None
                } else {
                    Some(alias)
                }
            } else {
                Some(alias)
            }
        });
        alias.hash(state);

        self.0.description().hash(state);
        self.0.skip().hash(state);
    }
}
