# OpenAI Brick

The OpenAI brick integrates with OpenAI's API to provide AI text generation and processing capabilities.

## Configuration

```json
{
  "brick_type": "openai",
  "config": {
    "api_key": "sk-your-api-key-here",
    "model_name": "gpt-3.5-turbo",
    "prompt_template": "Summarize the following: {{input_text}}",
    "temperature": 0.7,
    "max_tokens": 1000
  }
}
```

## Configuration Options

- **api_key** (required): Your OpenAI API key
- **model_name** (required): The model to use (e.g., `gpt-3.5-turbo`, `gpt-4`)
- **prompt_template** (required): Template string with placeholders (e.g., `{{input_text}}`)
- **temperature** (optional): Sampling temperature (0.0 to 2.0, default: 0.7)
- **max_tokens** (optional): Maximum tokens to generate (default: 1000)

## Input Format

The brick expects input data that matches the placeholders in the prompt template:

```json
{
  "input_text": "Long text to summarize..."
}
```

## Output Format

```json
{
  "response": "Generated text from OpenAI",
  "usage": {
    "prompt_tokens": 10,
    "completion_tokens": 20,
    "total_tokens": 30
  }
}
```

## Use Cases

- Text summarization
- Content generation
- Customer support responses
- Data extraction and analysis
- Translation

## Examples

### Summarization

```json
{
  "brick_type": "openai",
  "config": {
    "api_key": "sk-...",
    "model_name": "gpt-3.5-turbo",
    "prompt_template": "Summarize in one sentence: {{text}}"
  }
}
```

### Content Generation

```json
{
  "brick_type": "openai",
  "config": {
    "api_key": "sk-...",
    "model_name": "gpt-4",
    "prompt_template": "Write a product description for: {{product_name}}",
    "temperature": 0.8
  }
}
```

