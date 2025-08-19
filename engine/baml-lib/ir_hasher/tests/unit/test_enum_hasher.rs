use anyhow::Result;

use crate::helpers::*;

#[test]
fn test_simple_enum() -> Result<()> {
    let baml = r#"
enum Color {
  RED
  GREEN
  BLUE
}
"#;
    let ir = parse_baml(baml)?;
    let hash = get_interface_hash(&ir, "Color")?;
    assert_ne!(hash, 0, "Simple enum should have non-zero hash");
    Ok(())
}

#[test]
fn test_enum_value_addition_affects_interface() -> Result<()> {
    let baml1 = r#"
enum Color {
  RED
  GREEN
}
"#;
    let baml2 = r#"
enum Color {
  RED
  GREEN
  BLUE
}
"#;
    assert_interface_hash_different(baml1, baml2, "Color")?;
    Ok(())
}

#[test]
fn test_enum_value_removal_affects_interface() -> Result<()> {
    let baml1 = r#"
enum Color {
  RED
  GREEN
  BLUE
}
"#;
    let baml2 = r#"
enum Color {
  RED
  GREEN
}
"#;
    assert_interface_hash_different(baml1, baml2, "Color")?;
    Ok(())
}

#[test]
fn test_enum_value_rename_affects_interface() -> Result<()> {
    let baml1 = r#"
enum Color {
  RED
  GREEN
  BLUE
}
"#;
    let baml2 = r#"
enum Color {
  RED
  GREEN
  PURPLE
}
"#;
    assert_interface_hash_different(baml1, baml2, "Color")?;
    Ok(())
}

#[test]
fn test_enum_value_alias_only_affects_implementation() -> Result<()> {
    let baml1 = r#"
enum Status {
  ACTIVE @alias("active")
  INACTIVE
}
"#;
    let baml2 = r#"
enum Status {
  ACTIVE @alias("is_active")
  INACTIVE
}
"#;
    assert_interface_hash_equal(baml1, baml2, "Status")?;
    assert_implementation_hash_different(baml1, baml2, "Status")?;
    Ok(())
}

#[test]
fn test_enum_value_description_only_affects_implementation() -> Result<()> {
    let baml1 = r#"
enum Status {
  ACTIVE @description("User is active")
  INACTIVE
}
"#;
    let baml2 = r#"
enum Status {
  ACTIVE @description("Currently active")
  INACTIVE
}
"#;
    assert_interface_hash_equal(baml1, baml2, "Status")?;
    assert_implementation_hash_different(baml1, baml2, "Status")?;
    Ok(())
}

#[test]
fn test_enum_value_skip_only_affects_implementation() -> Result<()> {
    let baml1 = r#"
enum Status {
  ACTIVE
  INACTIVE
  DEPRECATED
}
"#;
    let baml2 = r#"
enum Status {
  ACTIVE
  INACTIVE
  DEPRECATED @skip
}
"#;
    assert_interface_hash_equal(baml1, baml2, "Status")?;
    assert_implementation_hash_different(baml1, baml2, "Status")?;
    Ok(())
}

#[test]
fn test_enum_value_ordering_affects_implementation() -> Result<()> {
    let baml1 = r#"
enum Priority {
  HIGH
  MEDIUM
  LOW
}
"#;
    let baml2 = r#"
enum Priority {
  LOW
  MEDIUM
  HIGH
}
"#;
    // For enums, order might affect both interface and implementation
    // depending on the implementation, but typically order affects implementation
    assert_implementation_hash_different(baml1, baml2, "Priority")?;
    Ok(())
}

#[test]
fn test_large_enum() -> Result<()> {
    let mut baml = String::from("enum LargeEnum {\n");
    for i in 1..=100 {
        baml.push_str(&format!("  VALUE_{:03}\n", i));
    }
    baml.push_str("}\n");

    let ir = parse_baml(&baml)?;
    let hash = get_interface_hash(&ir, "LargeEnum")?;
    assert_ne!(hash, 0, "Large enum should have non-zero hash");
    Ok(())
}

#[test]
fn test_enum_hash_stability() -> Result<()> {
    let baml = r#"
enum Status {
  ACTIVE
  INACTIVE
  PENDING
}
"#;
    let ir = parse_baml(baml)?;
    assert_hash_stable(&ir)?;
    Ok(())
}
