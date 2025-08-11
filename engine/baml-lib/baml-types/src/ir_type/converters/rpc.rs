use crate::{
    baml_value::TypeLookups,
    ir_type::{ArrowGeneric, TypeRPC, UnionTypeGeneric},
    type_meta, StreamingMode, TypeIR, TypeValue,
};

pub fn from_type_ir(r#type: &TypeIR, _lookup: &impl TypeLookups) -> TypeRPC {
    // This inner worker function goes from `FieldType` to `FieldType` to be
    // suitable for recursive use. We only wrap the outermost `FieldType` in
    // `StreamingType`.

    // A copy of the metadata to use in the new type.
    let meta = type_meta::RPC::new(&r#type.meta().constraints);

    match r#type {
        TypeIR::Primitive(type_value, _) => match type_value {
            TypeValue::Null => TypeRPC::Primitive(TypeValue::Null, meta),
            TypeValue::Int => TypeRPC::Primitive(TypeValue::Int, meta),
            TypeValue::Float => TypeRPC::Primitive(TypeValue::Float, meta),
            TypeValue::Bool => TypeRPC::Primitive(TypeValue::Bool, meta),
            TypeValue::String => TypeRPC::Primitive(TypeValue::String, meta),
            TypeValue::Media(media_type) => TypeRPC::Primitive(TypeValue::Media(*media_type), meta),
        },
        TypeIR::Enum { name, dynamic, .. } => TypeRPC::Enum {
            name: name.clone(),
            dynamic: *dynamic,
            meta: meta.clone(),
        },
        TypeIR::Literal(literal_value, _) => TypeRPC::Literal(literal_value.clone(), meta),
        TypeIR::Class { name, dynamic, .. } => TypeRPC::Class {
            name: name.clone(),
            mode: StreamingMode::NonStreaming,
            dynamic: *dynamic,
            meta: meta.clone(),
        },
        TypeIR::List(item_type, _) => {
            TypeRPC::List(Box::new(from_type_ir(item_type, _lookup)), meta)
        }
        TypeIR::Map(key_type, item_type, _) => TypeRPC::Map(
            {
                // Keys cannot be null in maps
                let mut clone = key_type.clone();
                clone.meta_mut().streaming_behavior.needed = true;
                Box::new(from_type_ir(&clone, _lookup))
            },
            Box::new(from_type_ir(item_type, _lookup)),
            meta,
        ),
        TypeIR::RecursiveTypeAlias { name, .. } => TypeRPC::RecursiveTypeAlias {
            name: name.clone(),
            mode: StreamingMode::NonStreaming,
            meta: meta.clone(),
        },
        TypeIR::Tuple(field_types, _) => TypeRPC::Tuple(
            field_types
                .iter()
                .map(|t| from_type_ir(t, _lookup))
                .collect(),
            meta,
        ),
        TypeIR::Arrow(arrow, _) => TypeRPC::Arrow(
            Box::new(ArrowGeneric {
                param_types: arrow
                    .param_types
                    .iter()
                    .map(|t| from_type_ir(t, _lookup))
                    .collect(),
                return_type: from_type_ir(&arrow.return_type, _lookup),
            }),
            meta,
        ),
        TypeIR::Union(union_type, _) => {
            let variants = union_type.iter_include_null();
            let variants = variants
                .into_iter()
                .cloned()
                .map(|t| from_type_ir(&t, _lookup));

            TypeRPC::Union(
                unsafe { UnionTypeGeneric::new_unsafe(variants.collect()) },
                meta,
            )
        }
    }
}
