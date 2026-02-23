# NATLaB_BUILDR26

**High-precision cross-platform app builder and repair engine** — v26.0.0

NATLaB_BUILDR26 is a revolutionary new concept in application design and building, focused entirely on user ease.  It creates full, working, highly-complex applications with:

- ⚡ **Full boot manager** for instant startup
- 📦 **Full distributable** ready-to-run cross-platform packages (CLI, Desktop, Web, Service)
- 🌍 **Cross-platform** – Linux, Windows, macOS, mobile-ready
- 🔧 **High-performance & scalable** – runs locally or on a remote network of build agents
- 🤖 **AI-assisted** with API key activation for code generation and repair
- 🎨 **Icon / logo builder** with full icon allocation for loadable startup (ICO, PNG, SVG, web)

---

## Quick Start

```bash
pip install natlab-buildr26
```

### Build a new application

```bash
# CLI application
natlab build MyApp --type cli

# Desktop GUI application
natlab build MyDesktopApp --type desktop --platform linux

# Web application
natlab build MyWebApp --type web --output ./dist/MyWebApp

# Background service
natlab build MyService --type service
```

### Repair an existing project

```bash
natlab repair ./path/to/project
```

### Generate icons

```bash
natlab icons --name "My App" --color "#2563EB" --output ./icons
```

### List available templates

```bash
natlab templates
```

### Start a remote build server

```bash
natlab server --port 5026 --token my-secret-token
```

---

## Architecture

```
natlab/
├── builder/          # Core build engine, templates & packager
│   ├── engine.py     # BuildEngine – orchestrates the full build lifecycle
│   ├── templates.py  # App templates (CLI, Desktop, Web, Service)
│   └── packager.py   # Cross-platform distributable archive creator
├── boot/             # Boot manager for instant application startup
│   └── manager.py    # BootManager – fast init, config, platform detection
├── ai/               # AI-assisted code enhancement
│   └── assistant.py  # AIAssistant – API-key-activated code generation/repair
├── icon/             # Icon & logo builder
│   └── builder.py    # IconBuilder – multi-platform, multi-size icon set generator
├── network/          # Remote network build architecture
│   ├── server.py     # BuildServer – HTTP build server (Flask)
│   └── client.py     # BuildClient – HTTP client for remote builds
└── cli.py            # Command-line interface (Click + Rich)
```

---

## AI Integration

Enable AI-assisted enhancements by providing an API key:

```bash
export NATLAB_AI_KEY=sk-your-openai-key

natlab build MyApp --type cli --ai
```

Custom AI endpoint:

```bash
export NATLAB_AI_URL=https://my-llm-server/v1/chat/completions
export NATLAB_AI_MODEL=gpt-4o
```

---

## Remote Build Network

Run a build server on a remote machine:

```bash
# On the build server
natlab server --host 0.0.0.0 --port 5026 --token my-token

# From any client
python - <<'EOF'
from natlab.network.client import BuildClient
client = BuildClient("http://buildserver:5026", api_token="my-token")
result = client.build("MyApp", app_type="desktop", platform="linux")
print(result)
EOF
```

---

## Python API

```python
from pathlib import Path
from natlab.builder.engine import BuildEngine

engine = BuildEngine()
result = engine.build(
    app_name="MyApp",
    app_type="desktop",
    output_dir=Path("./dist/MyApp"),
    platform="linux",
    with_boot_manager=True,
    with_icons=True,
)
print(result)
```

---

## License

See [LICENSE](LICENSE).
