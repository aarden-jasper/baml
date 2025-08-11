use std::collections::HashSet;

use crate::interfaces::{self, LLMRenderable, ShallowSignature, TypedObject};

/// Find some way to hash the class

// Shallow hashes are based on the thigns that define the class
// (unordered list of [field name, field type], name of the class, if its dynamic)

pub trait ClassField: TypedObject + LLMRenderable {}

pub trait ClassDefinition: TypedObject + LLMRenderable {
    // ordered
    fn fields_sorted_by_order<'a>(&'a self) -> Vec<&'a dyn ClassField>;
    fn fields_sorted_by_name<'a>(&'a self) -> Vec<&'a dyn ClassField> {
        let mut fields = self.fields_sorted_by_order();
        fields.sort_by_key(|f| f.name());
        fields
    }
}

pub struct TopClassDefinition<'a>(&'a dyn ClassDefinition);

impl<'a> TopClassDefinition<'a> {
    pub fn new(definition: &'a dyn ClassDefinition) -> Self {
        Self(definition)
    }
}

impl ShallowSignature for TopClassDefinition<'_> {
    fn shallow_hash_prefix(&self) -> &'static str {
        "class"
    }

    fn shallow_interface_hash(&self) -> impl std::hash::Hash {
        ClassInterface(self.0)
    }

    fn unsorted_interface_dependencies(&self) -> HashSet<String> {
        self.0
            .fields_sorted_by_order()
            .iter()
            .flat_map(|f| f.r#type().immediate_dependencies())
            .collect()
    }

    fn shallow_implementation_hash(&self) -> Option<impl std::hash::Hash> {
        Some(ClassImplementation(self.0))
    }
}

struct ClassFieldInterface<'a, T: ClassField + ?Sized>(pub(crate) &'a T);

impl<'a, T: ClassField + ?Sized> std::hash::Hash for ClassFieldInterface<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        interfaces::TypedObjectInterface(self.0).hash(state);
        interfaces::LLMRenderableInterface(self.0).hash(state);
    }
}

struct ClassInterface<'a, T: ClassDefinition + ?Sized>(pub(crate) &'a T);

impl<'a, T: ClassDefinition + ?Sized> std::hash::Hash for ClassInterface<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        interfaces::TypedObjectInterface(self.0).hash(state);
        interfaces::LLMRenderableInterface(self.0).hash(state);
        // Important to sort by name!
        // ordering is only relevant for the interface, so
        // we want to make the interface hash deterministic
        for field in self.0.fields_sorted_by_name() {
            ClassFieldInterface(field).hash(state);
        }
    }
}

struct ClassFieldImplementation<'a, T: ClassField + ?Sized>(pub(crate) &'a T);

impl<'a, T: ClassField + ?Sized> std::hash::Hash for ClassFieldImplementation<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        interfaces::TypedObjectImplementation(self.0).hash(state);
        interfaces::LLMRenderableImplementation(self.0).hash(state);
    }
}

struct ClassImplementation<'a, T: ClassDefinition + ?Sized>(pub(crate) &'a T);

impl<'a, T: ClassDefinition + ?Sized> std::hash::Hash for ClassImplementation<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        interfaces::TypedObjectImplementation(self.0).hash(state);
        interfaces::LLMRenderableImplementation(self.0).hash(state);
        for field in self.0.fields_sorted_by_order() {
            ClassFieldImplementation(field).hash(state);
        }
    }
}
