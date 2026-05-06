# openrouter-free

A fast CLI tool to browse and filter free models available on [OpenRouter](https://openrouter.ai/).

## Features

- List all free models from OpenRouter
- Sort by context length, name, or model ID
- Filter to show only free models or all models
- Output in table format or JSON
- Fast and lightweight (Rust)

## Installation

### From Source

```bash
git clone https://github.com/your-username/openrouter-free.git
cd openrouter-free
cargo build --release
./target/release/openrouter-free --help
```

### Binary

Download pre-built binaries from the [Releases](https://github.com/your-username/openrouter-free/releases) page.

## Usage

```bash
# List top 10 free models (default)
openrouter-free

# List top 5 free models
openrouter-free -l 5

# Show all models (including paid)
openrouter-free -a

# Sort by name
openrouter-free -s name

# Sort by model ID
openrouter-free -s id

# Show full descriptions
openrouter-free -v

# Output as JSON (useful for scripting)
openrouter-free -l 10 -j

# Combine options
openrouter-free -l 20 -a -s name -v
```

## Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--limit <N>` | `-l` | Number of models to show | `10` |
| `--all` | `-a` | Show all models (not just free) | `false` |
| `--verbose` | `-v` | Show full description | `false` |
| `--json` | `-j` | Output as JSON | `false` |
| `--sort <FIELD>` | `-s` | Sort by: `context`, `name`, `id` | `context` |
| `--help` | `-h` | Show help | |
| `--version` | `-V` | Show version | |

## Examples

### Table Output

```
$ openrouter-free -l 5

Found 5 free models (sorted by context, showing top 5):

========================================================================================================================
IDX      MODEL ID                                           CONTEXT         PROMPT       DESCRIPTION
========================================================================================================================
1        inclusionai/ling-2.6-1t:free                       262K            Free         Ling-2.6-1T is an instant (instruct) model from in
2        tencent/hy3-preview:free                           262K            Free         Hy3 preview is a high-efficiency Mixture-of-Expert
3        google/gemma-4-26b-a4b-it:free                     262K            Free         Gemma 4 26B A4B IT is an instruction-tuned Mixture
4        baidu/cobuddy:free                                 131K            Free         CoBuddy is a code generation model from Baidu, opt
5        nvidia/nemotron-3-nano-omni-30b-a3b-reasoning:fr   256K            Free         NVIDIA Nemotron™ 3 Nano Omni is a 30B-A3B open mul
========================================================================================================================

Tip: Use -v flag to see full descriptions
```

### JSON Output

```bash
$ openrouter-free -l 2 -j
[
  {
    "completion_price": "0",
    "context_length": 262144,
    "description": "Ling-2.6-1T is an instant (instruct) model...",
    "id": "inclusionai/ling-2.6-1t:free",
    "modality": "text->text",
    "name": "inclusionAI: Ling-2.6-1T (free)",
    "prompt_price": "0"
  }
]
```

## How It Works

This tool calls the OpenRouter Models API endpoint:

```
GET https://openrouter.ai/api/v1/models
```

Free models are identified by the `:free` suffix in the model ID (e.g., `google/gemma-4-26b-a4b-it:free`).

## Requirements

- Rust 1.75+
- Internet connection (to fetch model list)

## License

MIT
