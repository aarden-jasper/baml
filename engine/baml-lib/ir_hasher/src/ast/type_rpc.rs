use std::collections::HashSet;

use baml_types::{ir_type::TypeRPC, type_meta, TypeValue};

use crate::interfaces::ShallowSignature;

impl ShallowSignature for TypeRPC {
    fn shallow_hash_prefix(&self) -> &'static str {
        "type"
    }

    fn shallow_interface_hash(&self) -> impl std::hash::Hash {
        TypeRPCInterface(self)
    }

    fn unsorted_interface_dependencies(&self) -> HashSet<String> {
        self.immediate_dependencies()
    }

    fn shallow_implementation_hash(&self) -> Option<impl std::hash::Hash> {
        // If union order ends up being important, we can hash the union type
        // then this will need to be changed
        Some(TypeRPCImplementation(self))
    }
}

struct TypeRPCInterface<'a>(&'a TypeRPC);

impl<'a> std::hash::Hash for TypeRPCInterface<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.0 {
            TypeRPC::Primitive(type_value, _) => {
                TypeValueInterface(type_value).hash(state);
            }
            TypeRPC::Enum { name, dynamic, .. } => {
                "enum".hash(state);
                name.hash(state);
                dynamic.hash(state);
            }
            TypeRPC::Literal(literal_value, _) => {
                "literal".hash(state);
                match literal_value {
                    baml_types::LiteralValue::String(v) => v.hash(state),
                    baml_types::LiteralValue::Int(v) => v.hash(state),
                    baml_types::LiteralValue::Bool(v) => v.hash(state),
                }
            }
            TypeRPC::Class { name, dynamic, .. } => {
                "class".hash(state);
                name.hash(state);
                dynamic.hash(state);
            }
            TypeRPC::List(type_generic, _) => {
                "list".hash(state);
                type_generic.hash(state);
            }
            TypeRPC::Map(key_type, value_type, _) => {
                "map".hash(state);
                key_type.hash(state);
                value_type.hash(state);
            }
            TypeRPC::Tuple(type_generics, _) => {
                "tuple".hash(state);
                type_generics.hash(state);
            }
            TypeRPC::Arrow(arrow_generic, _) => {
                "arrow".hash(state);
                arrow_generic.hash(state);
            }
            TypeRPC::Union(union_type_generic, _) => {
                "union".hash(state);
                union_type_generic
                    .iter_include_null()
                    .into_iter()
                    .for_each(|t| t.hash(state));
            }
            TypeRPC::RecursiveTypeAlias { name, .. } => {
                "recursive_type_alias".hash(state);
                name.hash(state);
            }
        }
        RpcMetaInterface(self.0.meta()).hash(state);
    }
}

struct RpcMetaInterface<'a>(&'a type_meta::RPC);

impl<'a> std::hash::Hash for RpcMetaInterface<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for constraint in self.0.constraints.iter() {
            constraint.label.hash(state);
            constraint.expression.hash(state);
        }
    }
}

struct TypeValueInterface<'a>(&'a TypeValue);

impl<'a> std::hash::Hash for TypeValueInterface<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.0 {
            TypeValue::String => "string".hash(state),
            TypeValue::Int => "i64".hash(state),
            TypeValue::Float => "f64".hash(state),
            TypeValue::Bool => "bool".hash(state),
            TypeValue::Null => "null".hash(state),
            TypeValue::Media(baml_media_type) => match baml_media_type {
                baml_types::BamlMediaType::Image => "media/image".hash(state),
                baml_types::BamlMediaType::Audio => "media/audio".hash(state),
                baml_types::BamlMediaType::Pdf => "media/pdf".hash(state),
                baml_types::BamlMediaType::Video => "media/video".hash(state),
            },
        }
    }
}

struct TypeRPCImplementation<'a>(&'a TypeRPC);

impl<'a> std::hash::Hash for TypeRPCImplementation<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // If union order ends up being important,
        // we can hash the union type then this will need
        // to be changed to account for that.
        //
        // class / enum / recursive type alias specific implementations
        // are not handled here as they are handled in their respective
        // definitions.
        TypeRPCInterface(self.0).hash(state);
    }
}
