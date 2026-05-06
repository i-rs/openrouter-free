---
name: "openrouter-llm"
description: "Helps browse and query free LLM models from OpenRouter. Invoke when user wants to find or use free AI models, or needs model recommendations for specific tasks."
---

# OpenRouter LLM Skill

This skill helps you discover and interact with free LLM models available on OpenRouter.

## Commands

### 1. List Free Models

```bash
# Default: top 10 free models sorted by context length
openrouter-free

# Custom limit
openrouter-free -l 20

# Show all models (including paid)
openrouter-free -a

# Sort options: context, name, id
openrouter-free -s name
```

### 2. Get Model Details (JSON)

```bash
# Output as JSON for parsing
openrouter-free -l 5 -j
```

## Finding Models for Specific Tasks

**Code Generation:**
```bash
openrouter-free -l 20 -s context | grep -i code
openrouter-free -l 20 -s context | grep -i coding
```

**Long Context Tasks:**
```bash
# Models with largest context window
openrouter-free -l 10 -s context
```

**Multimodal (Image + Text):**
```bash
openrouter-free -l 20 -j | grep "image"
```

## Using Free Models

To use a free model with OpenRouter API:

```bash
# Example: Using openrouter/free router (automatically selects best free model)
curl -X POST "https://openrouter.ai/api/v1/chat/completions" \
  -H "Authorization: Bearer $OPENROUTER_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "openrouter/free",
    "messages": [{"role": "user", "content": "Hello!"}]
  }'

# Or use a specific free model
curl -X POST "https://openrouter.ai/api/v1/chat/completions" \
  -H "Authorization: Bearer $OPENROUTER_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "google/gemma-4-26b-a4b-it:free",
    "messages": [{"role": "user", "content": "Hello!"}]
  }'
```

## Common Free Models

| Model ID | Context | Best For |
|----------|---------|----------|
| `google/gemma-4-26b-a4b-it:free` | 262K | General tasks |
| `baidu/cobuddy:free` | 131K | Code generation |
| `inclusionai/ling-2.6-1t:free` | 262K | Fast inference |
| `nvidia/nemotron-3-nano-omni-30b-a3b-reasoning:free` | 256K | Multimodal |

## Rate Limits

- Free models: 50 requests/day (new users)
- After purchasing 10+ credits: 1000 requests/day
- Use `openrouter/free` router for automatic fallback
