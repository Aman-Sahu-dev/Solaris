# Solaris

**Solaris** is an AI-assisted PDF editor designed to preserve the original document while allowing precise, object-level edits.

The project separates responsibilities clearly:

- **Spring Boot** is the control plane: users, projects, permissions, AI orchestration, persistence, and audit history.
- **Rust engine** is the PDF processing service: parsing, inspection, content-stream editing, incremental saving, and preservation checks.
- **AI** only proposes structured edit commands. It never directly modifies PDF bytes.

## Architecture

```text
User / Frontend
      │
      ▼
Spring Boot control plane
      │  gRPC
      ▼
Solaris Rust PDF engine
      │
      ▼
PDF file / object storage
```

Spring Boot decides **what may be done**. The Rust engine decides **whether an edit can be performed faithfully to the PDF**, then applies it.

## Core principles

- Preserve the original PDF whenever possible.
- Edit the PDF object graph and content streams, not a reconstructed approximation.
- Treat AI output as an untrusted proposal, not a direct instruction.
- Validate every edit against the actual PDF state.
- Use incremental updates where possible instead of rebuilding documents.
- Verify that edits affect only the intended region.
- Fail clearly rather than silently producing a damaged PDF.

## Repository layout

```text
solaris-rust-engine/
├── Cargo.toml
├── build.rs
├── proto/
│   └── engine.proto
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config.rs
│   ├── error.rs
│   │
│   ├── grpc/
│   │   ├── mod.rs
│   │   ├── server.rs
│   │   └── engine_service.rs
│   │
│   ├── pdf/
│   │   ├── mod.rs
│   │   ├── document.rs
│   │   ├── pages.rs
│   │   ├── resources.rs
│   │   ├── writer.rs
│   │   └── content/
│   │       ├── mod.rs
│   │       ├── parser.rs
│   │       ├── model.rs
│   │       └── compiler.rs
│   │
│   ├── editing/
│   │   ├── mod.rs
│   │   ├── commands.rs
│   │   ├── validation.rs
│   │   └── execution.rs
│   │
│   ├── documents/
│   │   ├── mod.rs
│   │   ├── store.rs
│   │   └── versions.rs
│   │
│   └── verification/
│       ├── mod.rs
│       ├── structure.rs
│       ├── render_diff.rs
│       └── text_check.rs
│
└── tests/
    ├── content_stream_test.rs
    ├── text_replace_test.rs
    └── preservation_test.rs
```

## Module responsibilities

| Module | Responsibility |
|---|---|
| `proto/` | Shared gRPC contract between Spring Boot and Rust |
| `build.rs` | Generates Rust types and gRPC service stubs from `engine.proto` |
| `grpc/` | Receives requests from Spring Boot and returns structured results |
| `pdf/document.rs` | Opens and owns the parsed PDF document |
| `pdf/pages.rs` | Reads the page tree, page boxes, rotation, and page references |
| `pdf/resources.rs` | Resolves fonts, images, and other resources used by page content |
| `pdf/content/parser.rs` | Parses content-stream operators and operands |
| `pdf/content/model.rs` | Represents reliably recognized editable content |
| `pdf/content/compiler.rs` | Converts model edits back into valid PDF operators |
| `pdf/writer.rs` | Saves modified PDFs safely |
| `editing/` | Defines, validates, and executes edit commands |
| `documents/` | Stores documents and manages version snapshots |
| `verification/` | Checks structural validity, visual preservation, and text integrity |

## PDF model

A PDF is not a simple list of layers or text boxes. It is an object graph:

```text
Document catalog
└── Page tree
    └── Page
        ├── MediaBox / CropBox
        ├── Rotation
        ├── Resources
        └── Content stream
            ├── Text operators
            ├── Image operators
            ├── Path operators
            └── Graphics-state operators
```

A content stream contains drawing instructions. For example:

```text
BT
/F1 24 Tf
72 720 Td
(Hello Solaris) Tj
ET
```

This means:

- Begin a text object.
- Use font `/F1` at size 24.
- Move the text origin to `(72, 720)`.
- Draw the text `Hello Solaris`.
- End the text object.

Solaris must preserve this structure. Moving or replacing text should modify only the necessary operators, not rebuild the entire document.

## Development milestones

### 1. Inspect a PDF

- Open a PDF using `lopdf`.
- Read the catalog and page tree.
- Return page count, page boxes, rotation, and basic metadata.

### 2. Inspect page content

- Read a page’s content stream.
- Parse PDF operators.
- Identify basic text runs and their transforms.
- Return an editable page model.

### 3. Apply the first edit

- Implement `MoveText`.
- Modify only the relevant transform or positioning operator.
- Save the document.
- Verify that unrelated page content remains unchanged.

### 4. Add preservation checks

- Validate PDF structure after editing.
- Compare rendered output before and after an edit.
- Confirm that changes are limited to the intended area.
- Reject edits that cannot be performed faithfully.

### 5. Connect Spring Boot

- Implement the gRPC contract.
- Add document upload and storage in Spring Boot.
- Persist document metadata, versions, and edit history.
- Add AI command generation later.

## Technology choices

| Area | Choice | Reason |
|---|---|---|
| Backend control plane | Spring Boot | Authentication, persistence, AI integration, and service orchestration |
| PDF engine | Rust | Performance, memory safety, and low-level PDF control |
| Service boundary | gRPC | Strong typed contract between Java and Rust |
| PDF object access | `lopdf` | Direct access to PDF objects, streams, and references |
| Rendering / verification | `pdfium-render` | Useful for visual comparison and later rendering features |
| AI integration | Spring AI | Keeps provider choice and orchestration outside the Rust engine |

## What Solaris does not do yet

- No AI provider integration in the Rust engine.
- No WASM target.
- No OCR.
- No full document reconstruction.
- No collaborative editing.
- No generic “Photoshop-style layers” model.
- No promise that every PDF can be edited without visual changes.

## Development rules

1. Do not add an abstraction before a real requirement exists.
2. Do not rebuild a PDF from extracted data when a targeted edit is possible.
3. Do not trust AI-generated commands without validation.
4. Do not save an edit that cannot be verified.
5. Keep the Rust engine free of user, billing, and AI-provider concerns.
6. Add tests using real-world PDFs, including malformed and unusual files.

## First command

The first useful edit command should be:

```json
{
  "type": "ReplaceText",
  "page": 0,
  "text_run": 12,
  "new_text": "Invoice Total",
  "preserve_layout": true
}
```

Spring Boot may receive this proposal from an AI model, validate user permissions and business rules, then send a typed gRPC command to Rust. Rust validates whether the edit is technically possible and applies it only if it can preserve the document faithfully.
