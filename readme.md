# Scheme Interpreter in Rust

A Turing-complete Scheme interpreter built from scratch in Rust, featuring closures, lexical scoping, and multiple evaluation strategies.

---

## What This Is

An educational interpreter implementing core Scheme language features:

- **Arithmetic**: `(+ 2 3)`, `(* 5 (- 10 3))`
- **Booleans & Conditionals**: `(if (< 2 3) 10 20)`
- **Functions**: `(λ (x) (* x 2))`
- **Closures**: Functions that capture their environment
- **Let Bindings**: `(let ((x 10)) (+ x 5))`
- **Higher-Order Functions**: Functions as arguments/return values

**Status**: Turing-complete with proper lexical scoping and closures.

---

## Build & Run

```bash
# Build
cargo build

# Run tests
cargo test

# Run examples
cargo run
```

---

## Example Programs

```scheme
; Simple function
((λ (x) (+ x 1)) 5)  ; → 6

; Let bindings
(let ((x 10) (y 20))
  (+ x y))           ; → 30

; Closure (captures x=10)
(let ((x 10))
  (let ((add-x (λ (y) (+ x y))))
    (add-x 5)))      ; → 15

; Higher-order function
((λ (f x) (f (f x)))
 (λ (n) (+ n 1))
 5)                  ; → 7
```

---

## Architecture

```
Source Code → reader() → SExpr → desugar() → Expr → eval() → Value
```

**Key components:**
- `reader`: Text → S-expressions (parsing)
- `desugar`: S-expressions → AST
- `eval`: AST → Values (with environments)

**Three evaluation strategies implemented:**
- Big-step (recursive)
- Small-step (with contexts)
- CK machine (state-based)

---

## What I Learned

**Core concepts:**
- Parsing and AST design
- Environment-based evaluation
- Closures and lexical scoping
- Desugaring (syntactic sugar → core forms)
- Abstract machines (CK machine)

**Skills developed:**
- Building interpreters from scratch
- Recursive data structure traversal
- Test-driven development
- Problem decomposition

---

## Potential Extensions

Not implemented but would complete the language:

- **Recursion**: `letrec` or Y-combinator
- **Data structures**: Lists, pairs
- **Mutation**: Boxes, `set!`
- **Continuations**: `call/cc`

---

