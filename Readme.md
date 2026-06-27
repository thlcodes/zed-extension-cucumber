# Cucumber/Gherkin support for Zed

Zed extension adding Cucumber/Gherkin syntax highlighting and LSP support.

## Prerequisites

Install the language server globally with npm:

```bash
npm install -g @cucumber/language-server
```

> **Note:** `npm` is required (not bun/yarn/pnpm). The language server resolves tree-sitter WASM files via nested `node_modules` paths that only npm's global install layout provides.

## Features

- Gherkin Tree-Sitter Grammar (via [alistairstead/tree-sitter-gherkin](https://github.com/alistairstead/tree-sitter-gherkin), fork of [binhtddev/tree-sitter-gherkin](https://github.com/binhtddev/tree-sitter-gherkin))
  - Syntax highlighting with semantic step coloring (Given/When/Then)
  - Injections (docstring language detection)
  - Outline navigation
  - Full Gherkin spec: Feature, Scenario, Scenario Outline, Background, Examples, Rules, tags, data tables, docstrings
  - i18n keyword support

- LSP (via `@cucumber/language-server`)
  - Autocompletion for step definitions
  - Go-to-definition for steps
  - Configuration via `.zed/settings.json`:
    ```json
    {
      "lsp": {
        "cucumber": {
          "settings": {
            "features": ["features/**/*.feature"],
            "glue": ["steps/**/*.ts"]
          }
        }
      }
    }
    ```
