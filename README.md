# miniforth
A toy interpreter for a small subset of Forth

---
## Usage

```miniforth input.forth```

## Types
- 64-bit signed integers
- Booleans: true, false

## Words
- Arithmetic: +, -, *, /, negate
- Bitwise/Logical: and, or, xor, lshift, rshift, negate
- (In)equality: =, <, >
- Stack Ops: dup, drop, swap, over, .
- Definitions: :, ;