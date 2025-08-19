use anyhow::Result;

use crate::helpers::*;

#[test]
fn test_simple_type_alias() -> Result<()> {
    let baml = r#"
type Name = string
"#;
    let ir = parse_baml(baml)?;
    let hash = get_interface_hash(&ir, "Name")?;
    assert_ne!(hash, 0, "Simple type alias should have non-zero hash");
    Ok(())
}

#[test]
fn test_type_alias_underlying_type_change_affects_interface() -> Result<()> {
    let baml1 = r#"
type Age = int
"#;
    let baml2 = r#"
type Age = string
"#;
    assert_interface_hash_different(baml1, baml2, "Age")?;
    Ok(())
}

#[test]
fn test_optional_type_alias() -> Result<()> {
    let baml = r#"
type MaybeName = string?
"#;
    let ir = parse_baml(baml)?;
    let hash = get_interface_hash(&ir, "MaybeName")?;
    assert_ne!(hash, 0, "Optional type alias should have non-zero hash");
    Ok(())
}

#[test]
fn test_list_type_alias() -> Result<()> {
    let baml = r#"
type Names = string[]
"#;
    let ir = parse_baml(baml)?;
    let hash = get_interface_hash(&ir, "Names")?;
    assert_ne!(hash, 0, "List type alias should have non-zero hash");
    Ok(())
}

#[test]
fn test_map_type_alias() -> Result<()> {
    let baml = r#"
type Dictionary = map<string, string>
"#;
    let ir = parse_baml(baml)?;
    let hash = get_interface_hash(&ir, "Dictionary")?;
    assert_ne!(hash, 0, "Map type alias should have non-zero hash");
    Ok(())
}

#[test]
fn test_union_type_alias() -> Result<()> {
    let baml = r#"
type StringOrInt = string | int
"#;
    let ir = parse_baml(baml)?;
    let hash = get_interface_hash(&ir, "StringOrInt")?;
    assert_ne!(hash, 0, "Union type alias should have non-zero hash");
    Ok(())
}

#[test]
fn test_complex_union_type_alias() -> Result<()> {
    let baml = r#"
type Complex = string | int[]? | map<string, bool>
"#;
    let ir = parse_baml(baml)?;
    let hash = get_interface_hash(&ir, "Complex")?;
    assert_ne!(
        hash, 0,
        "Complex union type alias should have non-zero hash"
    );
    Ok(())
}

#[test]
fn test_type_alias_modifier_change_affects_interface() -> Result<()> {
    let baml1 = r#"
type Name = string
"#;
    let baml2 = r#"
type Name = string?
"#;
    assert_interface_hash_different(baml1, baml2, "Name")?;
    Ok(())
}

#[test]
fn test_type_alias_list_change_affects_interface() -> Result<()> {
    let baml1 = r#"
type Data = string
"#;
    let baml2 = r#"
type Data = string[]
"#;
    assert_interface_hash_different(baml1, baml2, "Data")?;
    Ok(())
}

#[test]
fn test_type_alias_union_variant_addition_affects_interface() -> Result<()> {
    let baml1 = r#"
type Value = string | int
"#;
    let baml2 = r#"
type Value = string | int | bool
"#;
    assert_interface_hash_different(baml1, baml2, "Value")?;
    Ok(())
}

#[test]
fn test_type_alias_with_check_constraints_does_not_affect_hashes() -> Result<()> {
    let baml1 = r#"
type Age = int
"#;
    let baml2 = r#"
type Age = int @check(positive, {{ this > 0 }})
"#;
    // @check constraints don't affect any hash - they're runtime-only validation
    assert_interface_hash_equal(baml1, baml2, "Age")?;
    assert_implementation_hash_equal(baml1, baml2, "Age")?;
    Ok(())
}

#[test]
fn test_type_alias_check_constraint_expression_change_does_not_affect_hashes() -> Result<()> {
    let baml1 = r#"
type Age = int @check(positive, {{ this > 0 }})
"#;
    let baml2 = r#"
type Age = int @check(positive, {{ this >= 0 }})
"#;
    // @check expression changes don't affect any hash - they're runtime-only validation
    assert_interface_hash_equal(baml1, baml2, "Age")?;
    assert_implementation_hash_equal(baml1, baml2, "Age")?;
    Ok(())
}

#[test]
fn test_nested_type_alias() -> Result<()> {
    let baml = r#"
type Inner = string
type Outer = Inner[]
"#;
    let ir = parse_baml(baml)?;
    let hash = get_interface_hash(&ir, "Outer")?;
    assert_ne!(hash, 0, "Nested type alias should have non-zero hash");
    Ok(())
}

#[test]
fn test_type_alias_hash_stability() -> Result<()> {
    let baml = r#"
type Complex = string | int[] | map<string, bool>
"#;
    let ir = parse_baml(baml)?;
    assert_hash_stable(&ir)?;
    Ok(())
}
