# Epimelr : A PDF library of Rust.

> [!CAUTION]
> This project is currently under development.

## Summary

**Epimelr** (pronounced /ˌɛpɪˈmɛlɚ/, “eh-pi-MEL-er”) is a PDF library written in Rust, designed to provide tools for creating, parsing, and manipulating PDF files in accordance with the PDF specification (ISO 32000). 

The library aims to offer a comprehensive set of features for working with PDF documents in a safe and efficient manner.

## Features (Planned)

- Parsing PDF files and extracting contents
- Creating new PDF documents
- Modifying PDF contents

## Roadmap

Currently, I'm learning the PDF specification and implementing basic features. 

This roadmap may changes as the development progresses.

- [x] Basic PDF Objects
  - [x] Header
    - [x] Version
    - [x] Binary Marker
  - [x] Basic PDF Objects
    - [x] Boolean Objects
    - [x] Numeric Objects
    - [x] String Objects
    - [x] Name Objects
    - [x] Array Objects
    - [x] Dictionary Objects
    - [x] Null Objects
  - [x] Indirect Objects
  - [x] Indirect References
  - [x] Stream Objects
- [ ] Structure
  - [ ] Document Catalog
  - [ ] Cross-Reference Table
  - [ ] Trailer
  - [ ] Object Streams
  - [ ] Cross-Reference Streams
- [ ] Creation
  - [ ] Document Structure
  - [ ] Page Management
  - [ ] Content Streams
- [ ] Parsing
  - [ ] Tokenization
  - [ ] Object Parsing
- [ ] Modification
  - [ ] Incremental Updates
- [ ] Standard Filters
- [ ] Functions
- [ ] Files
- [ ] Graphics
- [ ] Text
- [ ] Rendering
- [ ] Transparency