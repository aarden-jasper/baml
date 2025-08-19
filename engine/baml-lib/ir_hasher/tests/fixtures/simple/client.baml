client OpenAIClient {
  provider openai
  options {
    model "gpt-4"
    temperature 0.7
  }
}

client AnthropicClient {
  provider anthropic
  options {
    model "claude-3-opus"
    max_tokens 4096
  }
}

retry_policy SimpleRetry {
  max_retries 3
}

client ClientWithRetry {
  provider openai
  retry_policy SimpleRetry
  options {
    model "gpt-3.5-turbo"
  }
}