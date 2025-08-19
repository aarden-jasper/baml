use anyhow::Result;
use internal_baml_core::ir::repr::{make_test_ir_and_diagnostics, IntermediateRepr};
use ir_hasher::{generate_signatures, Signature};

pub fn parse_baml(content: &str) -> Result<IntermediateRepr> {
    let (ir, diagnostics) = make_test_ir_and_diagnostics(content)?;

    if diagnostics.has_errors() {
        return Err(anyhow::anyhow!(
            "Validation errors: {}",
            diagnostics.to_pretty_string()
        ));
    }

    Ok(ir)
}

pub fn parse_fixture(path: &str) -> Result<IntermediateRepr> {
    let full_path = format!(
        "/Users/vbv/repos/baml/engine/baml-lib/ir_hasher/tests/fixtures/{}",
        path
    );
    let content = std::fs::read_to_string(full_path)?;
    parse_baml(&content)
}

pub fn get_signatures(ir: &IntermediateRepr) -> Result<Vec<Signature>> {
    let adapters = ir.as_signature_adapters();
    generate_signatures(adapters)
}

pub fn get_signature_by_name(ir: &IntermediateRepr, name: &str) -> Result<Signature> {
    let signatures = get_signatures(ir)?;
    signatures
        .into_iter()
        .find(|s| s.display_name == name)
        .ok_or_else(|| anyhow::anyhow!("Signature not found for {}", name))
}

pub fn get_interface_hash(ir: &IntermediateRepr, name: &str) -> Result<u64> {
    let signature = get_signature_by_name(ir, name)?;
    match signature.hashes {
        baml_rpc::NodeHash::CompileTimeOnly(part) => Ok(part.interface_hash()),
        _ => Err(anyhow::anyhow!("Unexpected hash type")),
    }
}

pub fn get_implementation_hash(ir: &IntermediateRepr, name: &str) -> Result<u64> {
    let signature = get_signature_by_name(ir, name)?;
    match signature.hashes {
        baml_rpc::NodeHash::CompileTimeOnly(part) => Ok(part.impl_hash()),
        _ => Err(anyhow::anyhow!("Unexpected hash type")),
    }
}

pub fn assert_interface_hash_equal(baml1: &str, baml2: &str, type_name: &str) -> Result<()> {
    let ir1 = parse_baml(baml1)?;
    let ir2 = parse_baml(baml2)?;

    let hash1 = get_interface_hash(&ir1, type_name)?;
    let hash2 = get_interface_hash(&ir2, type_name)?;

    assert_eq!(
        hash1, hash2,
        "Interface hash mismatch for type {}. Hash1: {}, Hash2: {}",
        type_name, hash1, hash2
    );
    Ok(())
}

pub fn assert_interface_hash_different(baml1: &str, baml2: &str, type_name: &str) -> Result<()> {
    let ir1 = parse_baml(baml1)?;
    let ir2 = parse_baml(baml2)?;

    let hash1 = get_interface_hash(&ir1, type_name)?;
    let hash2 = get_interface_hash(&ir2, type_name)?;

    assert_ne!(
        hash1, hash2,
        "Expected interface hashes to differ for type {}. Both have hash: {}",
        type_name, hash1
    );
    Ok(())
}

pub fn assert_implementation_hash_different(
    baml1: &str,
    baml2: &str,
    type_name: &str,
) -> Result<()> {
    let ir1 = parse_baml(baml1)?;
    let ir2 = parse_baml(baml2)?;

    let hash1 = get_implementation_hash(&ir1, type_name)?;
    let hash2 = get_implementation_hash(&ir2, type_name)?;

    assert_ne!(
        hash1, hash2,
        "Expected implementation hashes to differ for type {}. Both have hash: {}",
        type_name, hash1
    );
    Ok(())
}

pub fn assert_implementation_hash_equal(baml1: &str, baml2: &str, type_name: &str) -> Result<()> {
    let ir1 = parse_baml(baml1)?;
    let ir2 = parse_baml(baml2)?;

    let hash1 = get_implementation_hash(&ir1, type_name)?;
    let hash2 = get_implementation_hash(&ir2, type_name)?;

    assert_eq!(
        hash1, hash2,
        "Implementation hash mismatch for type {}. Hash1: {}, Hash2: {}",
        type_name, hash1, hash2
    );
    Ok(())
}

pub fn assert_hash_stable(ir: &IntermediateRepr) -> Result<()> {
    let hash1 = get_signatures(ir)?;
    let hash2 = get_signatures(ir)?;
    let hash3 = get_signatures(ir)?;

    for i in 0..hash1.len() {
        let h1 = &hash1[i];
        let h2 = &hash2[i];
        let h3 = &hash3[i];

        assert_eq!(
            h1.display_name, h2.display_name,
            "Names don't match between runs"
        );
        assert_eq!(
            h2.display_name, h3.display_name,
            "Names don't match between runs"
        );

        let (h1_interface, h1_impl) = match h1.hashes {
            baml_rpc::NodeHash::CompileTimeOnly(part) => (part.interface_hash(), part.impl_hash()),
            _ => panic!("Unexpected hash type"),
        };

        let (h2_interface, h2_impl) = match h2.hashes {
            baml_rpc::NodeHash::CompileTimeOnly(part) => (part.interface_hash(), part.impl_hash()),
            _ => panic!("Unexpected hash type"),
        };

        let (h3_interface, h3_impl) = match h3.hashes {
            baml_rpc::NodeHash::CompileTimeOnly(part) => (part.interface_hash(), part.impl_hash()),
            _ => panic!("Unexpected hash type"),
        };

        assert_eq!(
            h1_interface, h2_interface,
            "Interface hash not stable for {} between runs 1 and 2",
            h1.display_name
        );
        assert_eq!(
            h2_interface, h3_interface,
            "Interface hash not stable for {} between runs 2 and 3",
            h1.display_name
        );
        assert_eq!(
            h1_impl, h2_impl,
            "Implementation hash not stable for {} between runs 1 and 2",
            h1.display_name
        );
        assert_eq!(
            h2_impl, h3_impl,
            "Implementation hash not stable for {} between runs 2 and 3",
            h1.display_name
        );
    }

    Ok(())
}
