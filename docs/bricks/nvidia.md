# NVIDIA Brick

The NVIDIA brick integrates with NVIDIA's AI services including Automatic Speech Recognition (ASR), Optical Character Recognition (OCR), and text generation.

## Configuration

```json
{
  "brick_type": "nvidia",
  "config": {
    "api_key": "your-nvidia-api-key",
    "service": "asr",
    "model": "nvidia/nemospeech-1.3B-v1",
    "parameters": {
      "language": "en"
    }
  }
}
```

## Configuration Options

- **api_key** (required): Your NVIDIA API key
- **service** (required): Service type (`asr`, `ocr`, `text_generation`)
- **model** (required): Model identifier
- **parameters** (optional): Service-specific parameters

## Supported Services

### ASR (Automatic Speech Recognition)

Converts audio to text.

```json
{
  "brick_type": "nvidia",
  "config": {
    "api_key": "your-key",
    "service": "asr",
    "model": "nvidia/nemospeech-1.3B-v1",
    "parameters": {
      "language": "en",
      "audio_format": "wav"
    }
  }
}
```

### OCR (Optical Character Recognition)

Extracts text from images.

```json
{
  "brick_type": "nvidia",
  "config": {
    "api_key": "your-key",
    "service": "ocr",
    "model": "nvidia/ocr-model",
    "parameters": {
      "image_format": "png"
    }
  }
}
```

### Text Generation

Generates text using NVIDIA models.

```json
{
  "brick_type": "nvidia",
  "config": {
    "api_key": "your-key",
    "service": "text_generation",
    "model": "nvidia/text-model",
    "parameters": {
      "prompt": "{{input_text}}",
      "max_tokens": 500
    }
  }
}
```

## Input Format

Varies by service type. For text generation:

```json
{
  "input_text": "Text to process..."
}
```

For ASR/OCR, provide audio/image data in the appropriate format.

## Output Format

Service-specific output format.

## Use Cases

- Speech-to-text conversion
- Image text extraction
- AI-powered text generation
- Multilingual processing

