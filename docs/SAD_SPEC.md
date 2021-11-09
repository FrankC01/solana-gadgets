## Solana Account data Deserializer (sad) specification

`sad` data descriptors for deserialization are written in YAML files.

## The `sad` Descriptor
The `sad` high level descriptor starts after the normal YAML document start notation,`---`. Note that currently only 1 document per YAML file
`somefile.yml`
```yaml
---
descriptor_id:
    # remainder of descriptor for 'descriptor_id'
```

A high level descriptor is followed by a sequence of one or more Data Section Descriptors.

### Data Section Descriptors
Data Sections are layed out in the in the actual Solana account's data order. Each have at least one (1) property `type`.

*For example, it is not unusual that the first Data Section refers to a single byte that represents a boolean indicating whether the account has been initialized or not*

`somefile1.yml`
```yaml
---
descriptor_id:
    - initialized:              # Arbitrarily named
        type: DeclarationType   # Indicator of bytes type may occupy (see DeclarationType below)
    - next_section:
        type: DeclarationType
```

### DeclarationType

At this time, `Enum` is not supported.

Simple types
Type Semantic | Supported Type Syntax
------------- | ------------
Integer | I8, I16, I32, I64, I128
Unsigned Integer | U8, U16, U32, U64, U128
Float | F32, F64
Misc | Bool, String

Container types: Container types have a child construct describing the type the container holds
Type Semantic | Supported Type Syntax
------------- | ------------
Dynamic Size Arrays | Vec
Option | Option
HashSet | HashSet

Collection types: Collection types have children that describe the fields in the collection
Type Semantic | Supported Type Syntax
------------- | ------------
Key/Value pairs (Rust BTreeMap) | HashMap
Structure  | CStruct
Tuple (compound type) | Tuple

Special types
Type Semantic | Supported Type Syntax | Notes
------------- | ------------ | -------------
Named fields | NamedField | Needed for CStruct
Public Key | PublicKey | 32 byte array
Fixed Size Arrays (compound type) | array | The child supports the size and types in array
Length Prefix | length_prefix | Controls the number of bytes to deserialize for the child.

### Simple Example

File: `SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.yml`

This file is used to deserialize data from accounts owned by [program](https://github.com/hashblock/solana-cli-program-template) in devnet

```yaml
---
SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv: # Can be arbitrary but this is an actual devnet program ID
    - initialized:
        type: Bool              # byte 0
    - size_and_map:
        type: length_prefix
        size_type: U32          # byte 1-4
        contains:
          - type: HashMap       # byte 5 through bytes from deserializing 'size_type' above
            fields:
              - type: String
              - type: String
```
