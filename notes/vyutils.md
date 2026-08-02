# vyutils Overview

## Motivation
Common primitives are required to process structured text in a versioned filesystem,
and package them into efficient formats for use as an in-memory datastore with efficient
query language to analyze data. 
Project Vyasa publication view file takes that approach to build compelling search and 
visualization of structured corpora like scriptural text.

## Approach
We will model after Unix core-utils, build a set of CLI utils that make it possible to compose 
applications, with the option to create a single binary with zero dependencies. These
building blocks should also be available as WASM components for web browser and other hosts.
Rust is a natural choice to build components(crates) consumed by CLI.
If there is a need to create web UI or compoents, use Svelte.
Use astro starlight for documentation-site.

## Problems to tackle; all names are tentative to just get off the ground
1. Define a multi-part-text (`mpt`) format that is human friendly to view and edit using a plain text editor. Learn from MIME, md+frontmatter and other prior work.
2. `mpt` crate and CLI will allow creation, validation, manipulation (add a part, extract a part, delete a part, and so on).
3. `folder` processes files in a folder hierarchy. Learn from vyasac, static site generators, etc.
Examples of features include config file override per folder, naming convention for files to ignore,
infer meaning from path components, TBD dev server and watch mode, TBD production server
4. `mpt-db` builds on `folder` and `mpt` to create a db file suitable for delivering to a large
number of viewers. Technologies to consider include sqlite, turso db, etc.
This should enable delivering rich read-only applications like vyasa viewer, custom (readonly) DB apps 
with user defined schema, forms, reports and so on.
