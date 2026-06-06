# Model Profiles

Model Profiles are an optional future-facing configuration layer for model and
provider presets. They are intended to let users add models and OpenAI-compatible
providers through config instead of changing Claw core code for every new model.

Phase 2 only adds the safe loader. It does not activate profiles, switch
providers, rebuild API clients, or change existing `/model` behavior.

## Compatibility

If no profiles file exists, Claw keeps the existing model behavior:

- `--model`
- `/model <model>`
- `CLAW_AVAILABLE_MODELS`
- `OPENAI_BASE_URL`
- `OPENAI_API_KEY`
- `OLLAMA_HOST`
- existing settings `model` and `aliases`

Invalid JSON or validation errors are treated as recoverable loader reports for
later UI phases. They must not crash startup.

## Paths

The loader checks:

```text
CLAW_MODEL_PROFILES_FILE
```

If that environment variable is not set, it checks the Docker/profile path:

```text
/root/models/profiles.json
```

On Windows this corresponds to the mounted profile path:

```text
C:\Tools\claw-code\profile\models\profiles.json
```

## Secrets

API key values are not stored in `profiles.json`. Store only the environment
variable name:

```json
{
  "api_key_env": "CLAW_OPENROUTER_API_KEY"
}
```

The loader keeps the string `CLAW_OPENROUTER_API_KEY`; it does not resolve or
print the secret value.

## Example

```json
{
  "version": 1,
  "default_profile": "openrouter-qwen-coder",
  "profiles": [
    {
      "name": "openrouter-qwen-coder",
      "provider": "openai-compatible",
      "base_url": "https://openrouter.ai/api/v1",
      "api_key_env": "CLAW_OPENROUTER_API_KEY",
      "model": "openai/qwen/qwen3-coder:free",
      "description": "Free OpenRouter coding model with tools",
      "tags": ["cloud", "free", "tools", "coding"]
    },
    {
      "name": "local-qwen-coder",
      "provider": "ollama",
      "base_url": "http://host.docker.internal:11434",
      "api_key_env": "",
      "model": "openai/qwen3-coder:30b",
      "description": "Local Ollama coding model",
      "tags": ["local", "ollama", "coding"]
    }
  ]
}
```

## Planned Later Phases

- Show profiles in `/model`.
- Activate profiles inside the same provider.
- Add cross-provider switching.
- Generate `profiles.json` from model automation scripts.
