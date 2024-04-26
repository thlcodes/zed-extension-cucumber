# Cucumber/Gherkin support for Zed

_WIP_ Zed extension to add support for Cucumber/Gherkin.

## Features

- Gherkin Tree-Sitter Grammar (via [thlcodes/tree-sitter-gherkin](https://github.com/thlcodes/tree-sitter-gherkin)), including

  - highlights
  - injections (for `docstring`)
  - outline

- LSP (via @cucumber/languageserver)
  - looks for global executable `cucumber-language-server` first, installs/uses latest package `@cucumber/language-server` otherwise
  - supports configuration via project setting, e.g.
    ```json
    {
      "lsp": {
        "cucumber": {
          "settings": {
            "glue": ["src/**/*.ts"]
          }
        }
      }
    }
    ```
  - **Hint**: currently `@cucumber/language-server` uses a `node-tree-sitter` version that does not support Node 19/20
