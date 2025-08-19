use anyhow::Result;

use crate::helpers::*;

#[test]
fn test_simple_retry_policy() -> Result<()> {
    let baml = r#"
retry_policy SimpleRetry {
  max_retries 3
}
"#;
    let ir = parse_baml(baml)?;
    let hash = get_interface_hash(&ir, "SimpleRetry")?;
    assert_ne!(hash, 0, "Simple retry policy should have non-zero hash");
    Ok(())
}

#[test]
fn test_retry_policy_max_retries_change_affects_interface() -> Result<()> {
    let baml1 = r#"
retry_policy MyRetry {
  max_retries 3
}
"#;
    let baml2 = r#"
retry_policy MyRetry {
  max_retries 5
}
"#;
    assert_interface_hash_different(baml1, baml2, "MyRetry")?;
    Ok(())
}

#[test]
fn test_retry_policy_with_exponential_backoff() -> Result<()> {
    let baml = r#"
retry_policy ExponentialRetry {
  max_retries 5
  strategy {
    type exponential_backoff
    delay_ms 1000
    max_delay_ms 30000
  }
}
"#;
    let ir = parse_baml(baml)?;
    let hash = get_interface_hash(&ir, "ExponentialRetry")?;
    assert_ne!(
        hash, 0,
        "Retry policy with exponential backoff should have non-zero hash"
    );
    Ok(())
}

#[test]
fn test_retry_policy_with_constant_delay() -> Result<()> {
    let baml = r#"
retry_policy ConstantRetry {
  max_retries 10
  strategy {
    type constant_delay
    delay_ms 500
  }
}
"#;
    let ir = parse_baml(baml)?;
    let hash = get_interface_hash(&ir, "ConstantRetry")?;
    assert_ne!(
        hash, 0,
        "Retry policy with constant delay should have non-zero hash"
    );
    Ok(())
}

#[test]
fn test_retry_policy_strategy_type_change_affects_interface() -> Result<()> {
    let baml1 = r#"
retry_policy MyRetry {
  max_retries 3
  strategy {
    type constant_delay
    delay_ms 500
  }
}
"#;
    let baml2 = r#"
retry_policy MyRetry {
  max_retries 3
  strategy {
    type exponential_backoff
    delay_ms 500
    max_delay_ms 10000
  }
}
"#;
    assert_interface_hash_different(baml1, baml2, "MyRetry")?;
    Ok(())
}

#[test]
fn test_retry_policy_delay_change_affects_interface() -> Result<()> {
    let baml1 = r#"
retry_policy MyRetry {
  max_retries 3
  strategy {
    type constant_delay
    delay_ms 500
  }
}
"#;
    let baml2 = r#"
retry_policy MyRetry {
  max_retries 3
  strategy {
    type constant_delay
    delay_ms 1000
  }
}
"#;
    assert_interface_hash_different(baml1, baml2, "MyRetry")?;
    Ok(())
}

#[test]
fn test_retry_policy_max_delay_change_affects_interface() -> Result<()> {
    let baml1 = r#"
retry_policy MyRetry {
  max_retries 3
  strategy {
    type exponential_backoff
    delay_ms 1000
    max_delay_ms 10000
  }
}
"#;
    let baml2 = r#"
retry_policy MyRetry {
  max_retries 3
  strategy {
    type exponential_backoff
    delay_ms 1000
    max_delay_ms 30000
  }
}
"#;
    assert_interface_hash_different(baml1, baml2, "MyRetry")?;
    Ok(())
}

#[test]
fn test_multiple_retry_policies() -> Result<()> {
    let baml = r#"
retry_policy RetryA {
  max_retries 3
}

retry_policy RetryB {
  max_retries 5
  strategy {
    type exponential_backoff
    delay_ms 1000
    max_delay_ms 30000
  }
}
"#;
    let ir = parse_baml(baml)?;
    let hash_a = get_interface_hash(&ir, "RetryA")?;
    let hash_b = get_interface_hash(&ir, "RetryB")?;
    assert_ne!(
        hash_a, hash_b,
        "Different retry policies should have different hashes"
    );
    Ok(())
}

#[test]
fn test_retry_policy_hash_stability() -> Result<()> {
    let baml = r#"
retry_policy MyRetry {
  max_retries 5
  strategy {
    type exponential_backoff
    delay_ms 1000
    max_delay_ms 30000
  }
}
"#;
    let ir = parse_baml(baml)?;
    assert_hash_stable(&ir)?;
    Ok(())
}
