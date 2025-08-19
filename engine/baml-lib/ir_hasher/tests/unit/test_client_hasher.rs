use anyhow::Result;

use crate::helpers::*;

#[test]
fn test_simple_client() -> Result<()> {
    let baml = r#"
client MyClient {
  provider openai
  options {
    model "gpt-4"
  }
}
"#;
    let ir = parse_baml(baml)?;
    let hash = get_interface_hash(&ir, "MyClient")?;
    assert_ne!(hash, 0, "Simple client should have non-zero hash");
    Ok(())
}

#[test]
fn test_client_provider_change_affects_interface() -> Result<()> {
    let baml1 = r#"
client MyClient {
  provider openai
  options {
    model "gpt-4"
  }
}
"#;
    let baml2 = r#"
client MyClient {
  provider anthropic
  options {
    model "claude-3"
  }
}
"#;
    assert_interface_hash_different(baml1, baml2, "MyClient")?;
    Ok(())
}

#[test]
fn test_client_model_change_affects_implementation() -> Result<()> {
    let baml1 = r#"
client MyClient {
  provider openai
  options {
    model "gpt-4"
  }
}
"#;
    let baml2 = r#"
client MyClient {
  provider openai
  options {
    model "gpt-3.5-turbo"
  }
}
"#;
    // Model change should affect implementation
    assert_implementation_hash_different(baml1, baml2, "MyClient")?;
    Ok(())
}

#[test]
fn test_client_temperature_change_affects_implementation() -> Result<()> {
    let baml1 = r#"
client MyClient {
  provider openai
  options {
    model "gpt-4"
    temperature 0.7
  }
}
"#;
    let baml2 = r#"
client MyClient {
  provider openai
  options {
    model "gpt-4"
    temperature 0.9
  }
}
"#;
    assert_implementation_hash_different(baml1, baml2, "MyClient")?;
    Ok(())
}

#[test]
fn test_client_with_retry_policy() -> Result<()> {
    let baml = r#"
retry_policy MyRetry {
  max_retries 3
}

client MyClient {
  provider openai
  retry_policy MyRetry
  options {
    model "gpt-4"
  }
}
"#;
    let ir = parse_baml(baml)?;
    let hash = get_interface_hash(&ir, "MyClient")?;
    assert_ne!(
        hash, 0,
        "Client with retry policy should have non-zero hash"
    );
    Ok(())
}

#[test]
fn test_client_retry_policy_change_affects_implementation() -> Result<()> {
    let baml1 = r#"
retry_policy RetryA {
  max_retries 3
}

retry_policy RetryB {
  max_retries 5
}

client MyClient {
  provider openai
  retry_policy RetryA
  options {
    model "gpt-4"
  }
}
"#;
    let baml2 = r#"
retry_policy RetryA {
  max_retries 3
}

retry_policy RetryB {
  max_retries 5
}

client MyClient {
  provider openai
  retry_policy RetryB
  options {
    model "gpt-4"
  }
}
"#;
    assert_implementation_hash_different(baml1, baml2, "MyClient")?;
    Ok(())
}

#[test]
fn test_client_max_tokens_change_affects_implementation() -> Result<()> {
    let baml1 = r#"
client MyClient {
  provider openai
  options {
    model "gpt-4"
    max_tokens 1000
  }
}
"#;
    let baml2 = r#"
client MyClient {
  provider openai
  options {
    model "gpt-4"
    max_tokens 2000
  }
}
"#;
    assert_implementation_hash_different(baml1, baml2, "MyClient")?;
    Ok(())
}

#[test]
fn test_multiple_clients() -> Result<()> {
    let baml = r#"
client ClientA {
  provider openai
  options {
    model "gpt-4"
  }
}

client ClientB {
  provider anthropic
  options {
    model "claude-3"
  }
}
"#;
    let ir = parse_baml(baml)?;
    let hash_a = get_interface_hash(&ir, "ClientA")?;
    let hash_b = get_interface_hash(&ir, "ClientB")?;
    assert_ne!(
        hash_a, hash_b,
        "Different clients should have different hashes"
    );
    Ok(())
}

#[test]
fn test_client_hash_stability() -> Result<()> {
    let baml = r#"
client MyClient {
  provider openai
  options {
    model "gpt-4"
    temperature 0.7
    max_tokens 2000
  }
}
"#;
    let ir = parse_baml(baml)?;
    assert_hash_stable(&ir)?;
    Ok(())
}
