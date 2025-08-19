use std::collections::HashSet;

use crate::interfaces::{self, LLMRenderable, ShallowSignature, TypedObject};

/// Find some way to hash the enum

pub trait EnumDefinition: TypedObject + LLMRenderable {
    fn values_sorted_by_order<'a>(&'a self) -> Vec<&'a dyn EnumValue>;
    fn values_sorted_by_name<'a>(&'a self) -> Vec<&'a dyn EnumValue> {
        let mut values = self.values_sorted_by_order();
        values.sort_by_key(|v| v.name());
        values
    }
}

pub trait EnumValue: TypedObject + LLMRenderable {}

pub struct TopEnumDefinition<'a>(&'a dyn EnumDefinition);

impl<'a> TopEnumDefinition<'a> {
    pub fn new(definition: &'a dyn EnumDefinition) -> Self {
        Self(definition)
    }
}

impl ShallowSignature for TopEnumDefinition<'_> {
    fn shallow_hash_prefix(&self) -> &'static str {
        "enum"
    }

    fn shallow_interface_hash(&self) -> impl std::hash::Hash {
        EnumInterface(self.0)
    }

    fn unsorted_interface_dependencies(&self) -> HashSet<String> {
        HashSet::new()
    }

    fn shallow_implementation_hash(&self) -> impl std::hash::Hash {
        EnumImplementation(self.0)
    }
}

struct EnumValueInterface<'a, T: EnumValue + ?Sized>(pub(crate) &'a T);

impl<'a, T: EnumValue + ?Sized> std::hash::Hash for EnumValueInterface<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        interfaces::TypedObjectInterface(self.0).hash(state);
        interfaces::LLMRenderableInterface(self.0).hash(state);
    }
}

struct EnumInterface<'a, T: EnumDefinition + ?Sized>(pub(crate) &'a T);

impl<'a, T: EnumDefinition + ?Sized> std::hash::Hash for EnumInterface<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        interfaces::TypedObjectInterface(self.0).hash(state);
        interfaces::LLMRenderableInterface(self.0).hash(state);
        // Important to sort by name!
        // ordering is only relevant for the interface, so
        // we want to make the interface hash deterministic
        for value in self.0.values_sorted_by_name() {
            EnumValueInterface(value).hash(state);
        }
    }
}

struct EnumValueImplementation<'a, T: EnumValue + ?Sized>(pub(crate) &'a T);

impl<'a, T: EnumValue + ?Sized> std::hash::Hash for EnumValueImplementation<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        interfaces::TypedObjectImplementation(self.0).hash(state);
        interfaces::LLMRenderableImplementation(self.0).hash(state);
    }
}

struct EnumImplementation<'a, T: EnumDefinition + ?Sized>(pub(crate) &'a T);

impl<'a, T: EnumDefinition + ?Sized> std::hash::Hash for EnumImplementation<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        interfaces::TypedObjectImplementation(self.0).hash(state);
        interfaces::LLMRenderableImplementation(self.0).hash(state);
        for value in self.0.values_sorted_by_order() {
            EnumValueImplementation(value).hash(state);
        }
    }
}
