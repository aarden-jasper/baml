use anyhow::Result;

pub use crate::ast::{
    class::{ClassDefinition, ClassField},
    client::ClientDefinition,
    function::FunctionDefinition,
    r#enum::{EnumDefinition, EnumValue},
    retry_policy::RetryPolicyDefinition,
    type_alias::TypeAliasDefinition,
};

mod shallow_hash;
mod signature;

pub use signature::{Signature, SignatureType};

pub trait CanMakeSignature {
    fn functions(&self) -> Vec<&dyn FunctionDefinition>;
    fn classes(&self) -> Vec<&dyn ClassDefinition>;
    fn enums(&self) -> Vec<&dyn EnumDefinition>;
    fn type_aliases(&self) -> Vec<&dyn TypeAliasDefinition>;
    fn clients(&self) -> Vec<&dyn ClientDefinition>;
    fn retry_policies(&self) -> Vec<&dyn RetryPolicyDefinition>;
}

pub fn generate_signatures(can_make_signature: impl CanMakeSignature) -> Result<Vec<Signature>> {
    let functions = can_make_signature.functions();
    let classes = can_make_signature.classes();
    let enums = can_make_signature.enums();
    let type_aliases = can_make_signature.type_aliases();
    let clients = can_make_signature.clients();
    let retry_policies = can_make_signature.retry_policies();

    let all_tops = shallow_hash::AllTops::new(
        &functions,
        &classes,
        &enums,
        &type_aliases,
        &clients,
        &retry_policies,
    );

    let all_names = functions
        .iter()
        .map(|f| (f.name(), SignatureType::Function))
        .chain(classes.iter().map(|c| (c.name(), SignatureType::Class)))
        .chain(enums.iter().map(|e| (e.name(), SignatureType::Enum)))
        .chain(
            type_aliases
                .iter()
                .map(|t| (t.name(), SignatureType::TypeAlias)),
        )
        .chain(clients.iter().map(|c| (c.name(), SignatureType::Client)))
        .chain(
            retry_policies
                .iter()
                .map(|r| (r.name(), SignatureType::RetryPolicy)),
        );
    let new_signatures = all_names
        .map(|(name, signature_type)| Signature::new(signature_type, name, &all_tops))
        .collect::<Result<Vec<_>>>()?;

    Ok(new_signatures)
}
