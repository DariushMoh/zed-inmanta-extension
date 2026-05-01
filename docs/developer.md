# Developer doc

This developer doc explain how to regenerate the parser after making changes to the grammar.

## Prerequisites

| Tool | Purpose | Install |
|------|---------|---------|
| [Node.js](https://nodejs.org) ≥ 18 | runs the tree-sitter CLI | via your package manager |
| [tree-sitter CLI](https://tree-sitter.github.io/tree-sitter/creating-parsers) | generates the C parser from `grammar.js` | `npm install -g tree-sitter-cli` |
| [git](https://git-scm.com) | Zed fetches grammars via git, even local ones | already installed on most systems |
| [Zed](https://zed.dev) | the editor | zed.dev |

This repository https://github.com/HeikoRibberink/yacc-to-ts was used to convert the inmanta.y file to grammar.js file.
The binary yacc-to-ts needs to be built from source (see repo for more information).

## Contents

```
tree-sitter-inmanta/   ← Tree-sitter grammar (compile this first)
  grammar.js
  package.json

zed-inmanta/           ← Zed extension (point Zed at this)
  extension.toml
  languages/inmanta/
    config.toml
    highlights.scm
```

## Setup

### Step 1 — Generate the C parser

```bash
cd tree-sitter-inmanta
npm install
npx tree-sitter generate      # creates src/parser.c
```

### Step 2 — Create a local git repo for the grammar

Zed fetches grammars via git, so the directory must be a git repository.

```bash
git init
git add .
git commit -m "initial grammar"
git rev-parse HEAD            # prints the commit SHA — copy it
```

### Step 3 — Edit `zed-inmanta/extension.toml`

Open `zed-inmanta/extension.toml` and fill in the two placeholders:

```toml
[grammars.inmanta]
repository = "file:///REPLACE/WITH/ABSOLUTE/PATH/TO/tree-sitter-inmanta"
commit = "REPLACE_WITH_SHA"
```

Use the **absolute path** to the `tree-sitter-inmanta` directory on your machine,
and paste the SHA you copied in step 2.

Example (adjust the path to match your system):
```toml
repository = "file:///home/alice/inmanta-zed/tree-sitter-inmanta"
commit = "a1b2c3d4e5f6..."
```

### Step 4 — Install as a dev extension in Zed

1. Open Zed.
2. Open the command palette (`Ctrl+Shift+P` / `Cmd+Shift+P`).
3. Run **`zed: install dev extension`**.
4. Select the `zed-inmanta/` directory.

Zed will compile the grammar to WASM and activate the extension.
Open any `.cf` file to verify syntax highlighting is working.
At this point you may have to select the language manually in the bottom menu bar.
However you can easily make it automatic by adding this one line in zed settings.json:

```
  "file_types": {
    ...
    "Inmanta": ["cf"],
    ...
  },
```


## Updating after grammar changes

If you edit `grammar.js`:

```bash
cd tree-sitter-inmanta
npx tree-sitter generate
git add .
git commit -m "update grammar"
git rev-parse HEAD            # copy new SHA
```

Then update the `commit` field in `zed-inmanta/extension.toml` with the new SHA,
and run **`zed: rebuild dev extension`** from the command palette.

If you only edit `highlights.scm`, no recompile is needed — just run
**`zed: reload extensions`**.

## Troubleshooting

**`.cf` files are not highlighted**
Make sure `path-suffixes = ["cf"]` is in both `extension.toml` and
`languages/inmanta/config.toml`. Restart Zed if needed.

**`tree-sitter generate` fails**
The grammar has a conflict or syntax error. Run with `--log` for details:
```bash
npx tree-sitter generate --log
```

**Zed reports a grammar build error**
Check that the `repository` path is absolute and the `commit` SHA matches
the current HEAD of the grammar repo exactly.
