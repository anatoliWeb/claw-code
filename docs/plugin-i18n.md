# Plugin i18n

Claw Code plugins can optionally localize user-facing descriptions without
changing `plugin.json`.

Place translation files in the plugin root:

```text
workspace-tools/
|-- plugin.json
`-- i18n/
    |-- en.properties
    `-- uk.properties
```

The same layout works when the manifest is stored at
`.claude-plugin/plugin.json`; the `i18n` directory still belongs in the plugin
root.

Claw selects the language from `CLAW_UI_LANG`. Translation fallback is:

```text
CLAW_UI_LANG -> en -> original plugin.json value
```

Supported keys:

```properties
plugin.description=Safe workspace helper tools for Claw Code.
tool.project_tree.description=Print a compact project tree.
tool.project_tree.input.description=Options for rendering the tree.
tool.project_tree.input.max_depth.description=Maximum directory depth.
command.tree.description=Print the workspace tree.
```

Nested input properties use their dotted property path:

```properties
tool.example.input.options.limit.description=Maximum result count.
```

Plugin names, plugin ids, tool names, command names, and other identifiers are
never translated. Invalid translation files produce a warning and Claw
continues with the fallback description.
