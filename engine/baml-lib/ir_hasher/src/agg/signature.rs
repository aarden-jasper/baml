enum SignatureType {
    Class,
    Function,
    Enum,
    TypeAlias,
    Client,
    RetryPolicy,
}

#[derive(Clone)]
pub struct Signature {
    r#type: SignatureType,
    display_name: String,
    hash: super::shallow_hash::ShallowHash,
}

impl Signature {
    pub fn new(r#type: SignatureType, name: &str) -> Self {}
}
