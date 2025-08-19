retry_policy SimpleRetry {
  max_retries 3
}

retry_policy ComplexRetry {
  max_retries 5
  strategy {
    type exponential_backoff
    delay_ms 1000
    max_delay_ms 30000
  }
}

retry_policy ConstantDelayRetry {
  max_retries 10
  strategy {
    type constant_delay
    delay_ms 500
  }
}