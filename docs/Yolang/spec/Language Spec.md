# Yolang Language Specification

> **Status:** Design phase. This document describes the intended language. Nothing here is fully implemented yet.

---

## 1. Overview

Yolang is a strongly typed, compiled language with a Rust-inspired type system. Its core design principles are:

- **Strong static typing** with local type inference
- **No classes** — data and behavior are defined separately via structs, enums, and traits
- **Algebraic data types** — enums with data-carrying variants and exhaustive pattern matching
- **Explicit nullability** — absence of a value is represented by `Perhaps<T>` / `Nope`, never by null
- **Explicit error handling** — errors are values, represented as `Result<T, E>`
- **Memory managed by the runtime** — reference counting, no ownership semantics in the language

Source files use the `.yolo` extension.

---

## 2. Lexical Structure

### 2.1 Comments

```yolo
// Single-line comment

/* Multi-line
   comment */
```

Multi-line comments do not nest.

### 2.2 Identifiers

Identifiers start with a letter (`a–z`, `A–Z`) or underscore, followed by any combination of letters, digits, or underscores.

```
identifier := [a-zA-Z_][a-zA-Z0-9_]*
```

By convention:
- Types, structs, enums, and traits use `PascalCase`
- Variables, functions, and fields use `snake_case`

### 2.3 Keywords

```
and       as        else      enum      false     for
fun       if        impl      let       match     mut
nope      or        return    struct    trait     true
use       while     where
```

### 2.4 Literals

**Numbers — integers**
```yolo
42
1_000_000
```

**Numbers — floats**
```yolo
3.14
2.0
```

Integer and float are distinct types and do not implicitly coerce.

**Strings** — double-quoted UTF-8, with escape sequences:

| Sequence | Meaning        |
|----------|----------------|
| `\n`     | Newline        |
| `\t`     | Tab            |
| `\\`     | Backslash      |
| `\"`     | Double quote   |
| `\r`     | Carriage return|

```yolo
"hello\nworld"
```

**Booleans**
```yolo
true
false
```

**Nope** — the absence-of-value literal (equivalent to Rust's `None`)
```yolo
nope
```

### 2.5 Operators

| Category       | Operators                                    |
|----------------|----------------------------------------------|
| Arithmetic     | `+`  `-`  `*`  `/`  `%`                      |
| Comparison     | `==`  `!=`  `<`  `<=`  `>`  `>=`            |
| Logical        | `&&` (`and`)  `\|\|` (`or`)  `!`             |
| Assignment     | `=`                                          |
| Range          | `..`  `..=`                                  |
| Error propagation | `?`                                       |
| Type ascription | `:`                                         |
| Return type    | `->`                                         |
| Path           | `::`                                         |

---

## 3. Type System

Yolang is **statically and strongly typed**. Types are checked at compile time. There are no implicit conversions between types.

### 3.1 Primitive Types

| Type      | Description                         | Example         |
|-----------|-------------------------------------|-----------------|
| `Int`     | 64-bit signed integer               | `42`            |
| `Float`   | 64-bit floating point               | `3.14`          |
| `Bool`    | Boolean                             | `true`          |
| `String`  | UTF-8 string                        | `"hello"`       |
| `()`      | Unit type — represents no value     | `()`            |

The unit type `()` is only used explicitly when needed as a type parameter (e.g. `Result<(), Error>`). Functions that return nothing omit the `->` return type annotation entirely.

### 3.2 Type Inference

Types are inferred for local variable bindings. Type annotations are required at:
- Function parameter and return types
- Struct and enum field types
- Trait method signatures

```yolo
let x = 42;           // inferred: Int
let name = "Vlad";    // inferred: String
let y: Float = 3.14;  // explicit annotation
```

### 3.3 Generics

Types and functions can be parameterized with generic type parameters using `<T>` syntax.

```yolo
struct Stack<T> {
    items: List<T>,
}

fun first<T>(list: List<T>) -> Perhaps<T> {
    // ...
}
```

Generic constraints are expressed with `where` clauses or inline trait bounds:

```yolo
fun largest<T>(a: T, b: T) -> T where T: Comparable {
    // ...
}

// inline form
fun largest<T: Comparable>(a: T, b: T) -> T {
    // ...
}
```

---

## 4. Variables

### 4.1 Immutable bindings (`let`)

```yolo
let x = 42;
let name: String = "Vlad";
```

`let` bindings cannot be reassigned after initialization. They must always be initialized.

### 4.2 Mutable bindings (`mut`)

```yolo
mut counter = 0;
counter = counter + 1;
```

`mut` bindings can be reassigned. They must also be initialized at declaration.

### 4.3 Scoping

Variables are lexically scoped. Each block `{ }` introduces a new scope. Inner scopes can shadow outer variables.

---

## 5. Functions

```yolo
fun add(a: Int, b: Int) -> Int {
    return a + b;
}
```

- Parameters always require type annotations.
- The return type follows `->`. If omitted, the function returns nothing (unit).
- Functions are first-class values and can be assigned, passed, and returned.

### 5.1 No return type (unit)

```yolo
fun log(msg: String) {
    // returns nothing
}
```

### 5.2 Generic functions

```yolo
fun identity<T>(value: T) -> T {
    return value;
}
```

### 5.3 Closures

Anonymous functions are written with `fun` in expression position:

```yolo
let double = fun(x: Int) -> Int { return x * 2; };
double(5); // 10
```

Closures capture variables from their enclosing scope by reference. Captured `mut` variables are shared — mutations are visible in the outer scope.

### 5.4 The `?` operator

Inside a function that returns `Result<T, E>`, the `?` operator propagates errors:

```yolo
fun read_and_parse(path: String) -> Result<Int, Error> {
    let content = read_file(path)?;
    let number = parse_int(content)?;
    return number;
}
```

If the expression evaluates to an error variant, `?` returns early from the function with that error. Otherwise it unwraps the success value.

---

## 6. Structs

Structs define named data types.

```yolo
struct Point {
    x: Float,
    y: Float,
}
```

### 6.1 Instantiation

```yolo
let p = Point { x: 1.0, y: 2.0 };
```

### 6.2 Field access

```yolo
let x = p.x;
```

### 6.3 Methods (`impl`)

Behavior is added to structs via `impl` blocks, separate from the struct definition.

```yolo
impl Point {
    fun distance(self, other: Point) -> Float {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        return sqrt(dx * dx + dy * dy);
    }
}
```

`self` refers to the receiver instance. Methods are called with dot syntax:

```yolo
let d = p.distance(q);
```

### 6.4 Generic structs

```yolo
struct Pair<A, B> {
    first: A,
    second: B,
}
```

---

## 7. Enums

Enums define types with a fixed set of variants. Variants may carry data.

```yolo
enum Direction {
    North,
    South,
    East,
    West,
}

enum Shape {
    Circle { radius: Float },
    Rectangle { width: Float, height: Float },
    Triangle { base: Float, height: Float },
}
```

### 7.1 Instantiation

```yolo
let dir = Direction::North;
let s = Shape::Circle { radius: 5.0 };
```

### 7.2 Methods on enums

```yolo
impl Shape {
    fun area(self) -> Float {
        match self {
            Shape::Circle { radius } => 3.14159 * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}
```

---

## 8. Perhaps\<T\> and Nope

`Perhaps<T>` is the built-in optional type. It represents a value that may or may not be present. There is no null in Yolang — all absence must be expressed through `Perhaps<T>`.

```yolo
fun find_user(id: Int) -> Perhaps<User> {
    // ...
}
```

`Nope` is the literal for the absent case (equivalent to `None` in Rust).

```yolo
let result: Perhaps<Int> = nope;
let value: Perhaps<Int> = 42;
```

`Perhaps<T>` is a standard enum defined conceptually as:

```yolo
enum Perhaps<T> {
    Nope,
    Some(T),   // or: Perhaps { value: T }
}
```

Use `match` to safely unwrap:

```yolo
match find_user(1) {
    Perhaps::Some { value } => print(value.name),
    Perhaps::Nope => print("not found"),
}
```

The `.yolo()` method performs an unwrap, panicking if the value is `Nope`:

```yolo
let user = find_user(1).yolo(); // panics if not found
```

---

## 9. Result\<T, E\>

`Result<T, E>` represents the outcome of a fallible operation. It is a standard enum:

```yolo
enum Result<T, E> {
    Ok { value: T },
    Err { error: E },
}
```

Functions that can fail return `Result<T, E>`:

```yolo
fun divide(a: Float, b: Float) -> Result<Float, String> {
    if (b == 0.0) {
        return Result::Err { error: "division by zero" };
    }
    return Result::Ok { value: a / b };
}
```

Use `match` to handle both cases, or `?` to propagate errors up the call stack.

---

## 10. Traits

Traits define shared behavior across types.

```yolo
trait Printable {
    fun print(self);
}

trait Comparable {
    fun compare(self, other: Self) -> Int;
}
```

### 10.1 Implementing a trait

```yolo
impl Printable for Point {
    fun print(self) {
        print("(" + self.x + ", " + self.y + ")");
    }
}
```

### 10.2 Trait bounds in generics

```yolo
fun print_all<T: Printable>(items: List<T>) {
    for (let item in items) {
        item.print();
    }
}
```

### 10.3 Default method implementations

Traits may provide default implementations that types can override:

```yolo
trait Greet {
    fun name(self) -> String;

    fun greet(self) {
        print("Hello, " + self.name() + "!");
    }
}
```

---

## 11. Pattern Matching

`match` expressions perform exhaustive pattern matching. The compiler enforces that all cases are covered.

```yolo
match value {
    pattern => expression,
    pattern => expression,
    _ => expression,   // catch-all
}
```

### 11.1 Matching on enums

```yolo
match direction {
    Direction::North => print("going north"),
    Direction::South => print("going south"),
    Direction::East  => print("going east"),
    Direction::West  => print("going west"),
}
```

### 11.2 Matching with destructuring

```yolo
match shape {
    Shape::Circle { radius } => print("circle, r=" + radius),
    Shape::Rectangle { width, height } => print("rect " + width + "x" + height),
    Shape::Triangle { base, height } => print("triangle"),
}
```

### 11.3 Matching literals and guards

```yolo
match x {
    0 => print("zero"),
    n if n < 0 => print("negative"),
    _ => print("positive"),
}
```

### 11.4 `match` is an expression

```yolo
let label = match x {
    0 => "zero",
    1 => "one",
    _ => "other",
};
```

---

## 12. Control Flow

### 12.1 If / else

```yolo
if (condition) {
    // ...
} else if (other) {
    // ...
} else {
    // ...
}
```

`if` is also an expression:

```yolo
let label = if (x > 0) { "positive" } else { "non-positive" };
```

### 12.2 While loop

```yolo
while (condition) {
    // ...
}
```

### 12.3 For loop (C-style)

```yolo
for (mut i = 0; i < 10; i = i + 1) {
    // ...
}
```

### 12.4 For-in loop (iterator)

```yolo
for (let item in collection) {
    // ...
}
```

### 12.5 Return

```yolo
return;         // from a function with no return type
return value;   // from a function with a return type
```

---

## 13. Grammar

> This grammar will be refined as the implementation progresses.

```
Program            → Declaration* EOF

Declaration        → LetDeclaration
                   | MutDeclaration
                   | FunDeclaration
                   | StructDeclaration
                   | EnumDeclaration
                   | ImplBlock
                   | TraitDeclaration
                   | Statement

LetDeclaration     → "let" IDENTIFIER ( ":" Type )? "=" Expression ";"
MutDeclaration     → "mut" IDENTIFIER ( ":" Type )? "=" Expression ";"
FunDeclaration     → "fun" IDENTIFIER GenericParams? "(" Params? ")" ( "->" Type )? Block
StructDeclaration  → "struct" IDENTIFIER GenericParams? "{" StructFields "}"
EnumDeclaration    → "enum" IDENTIFIER GenericParams? "{" EnumVariants "}"
ImplBlock          → "impl" ( IDENTIFIER "for" )? Type "{" FunDeclaration* "}"
TraitDeclaration   → "trait" IDENTIFIER "{" TraitMethod* "}"

Params             → Param ( "," Param )*
Param              → IDENTIFIER ":" Type
StructFields       → StructField ( "," StructField )*
StructField        → IDENTIFIER ":" Type
EnumVariants       → EnumVariant ( "," EnumVariant )*
EnumVariant        → IDENTIFIER ( "{" StructFields "}" )?
GenericParams      → "<" IDENTIFIER ( ":" Type )? ( "," IDENTIFIER ( ":" Type )? )* ">"

Statement          → ExpressionStatement
                   | Block
                   | IfStatement
                   | WhileStatement
                   | ForStatement
                   | ReturnStatement

ExpressionStatement → Expression ";"
Block               → "{" Declaration* "}"
IfStatement         → "if" "(" Expression ")" Block ( "else" ( IfStatement | Block ) )?
WhileStatement      → "while" "(" Expression ")" Block
ForStatement        → "for" "(" ( MutDeclaration | ExpressionStatement | ";" )
                                  Expression? ";"
                                  Expression? ")" Block
                    | "for" "(" "let" IDENTIFIER "in" Expression ")" Block
ReturnStatement     → "return" Expression? ";"

Expression         → AssignmentExpression
AssignmentExpression → IDENTIFIER "=" AssignmentExpression | LogicalOrExpression
LogicalOrExpression  → LogicalAndExpression ( "||" LogicalAndExpression )*
LogicalAndExpression → ComparisonExpression ( "&&" ComparisonExpression )*
ComparisonExpression → TermExpression ( ( ">" | ">=" | "<" | "<=" | "!=" | "==" ) TermExpression )?
TermExpression       → FactorExpression ( ( "+" | "-" ) FactorExpression )*
FactorExpression     → UnaryExpression ( ( "*" | "/" | "%" ) UnaryExpression )*
UnaryExpression      → ( "!" | "-" ) UnaryExpression | CallExpression
CallExpression       → PostfixExpression ( "(" Arguments? ")" | "." IDENTIFIER | "?" )*
PostfixExpression    → PrimaryExpression
Arguments            → Expression ( "," Expression )*
PrimaryExpression    → INT | FLOAT | STRING | "true" | "false" | "nope" | "()"
                     | "(" Expression ")"
                     | IDENTIFIER ( "::" IDENTIFIER )*
                     | StructLiteral
                     | MatchExpression
                     | IfExpression
                     | ClosureExpression

MatchExpression      → "match" Expression "{" MatchArm ( "," MatchArm )* ","? "}"
MatchArm             → Pattern ( "if" Expression )? "=>" Expression
IfExpression         → "if" "(" Expression ")" Block "else" Block

StructLiteral        → IDENTIFIER ( "::" IDENTIFIER )* "{" FieldInit ( "," FieldInit )* ","? "}"
FieldInit            → IDENTIFIER ":" Expression

ClosureExpression    → "fun" "(" Params? ")" ( "->" Type )? Block

Pattern              → "_"
                     | "nope"
                     | IDENTIFIER
                     | IDENTIFIER "::" IDENTIFIER ( "{" PatternFields "}" )?
                     | INT | FLOAT | STRING | "true" | "false"
PatternFields        → IDENTIFIER ( "," IDENTIFIER )*

Type               → IDENTIFIER ( "<" TypeArgs ">" )?
                   | "()"
TypeArgs           → Type ( "," Type )*
```

---

## 14. Standard Library (Planned)

The following builtins and stdlib modules are planned. None are final.

### Built-in functions

| Name       | Signature                     | Description                            |
|------------|-------------------------------|----------------------------------------|
| `print`    | `(value: T)`                  | Print any printable value to stdout    |
| `clock`    | `() -> Int`                   | Unix timestamp in milliseconds         |

### Planned modules

- **`math`** — `floor`, `ceil`, `abs`, `sqrt`, `pow`, `min`, `max`
- **`string`** — `len`, `split`, `trim`, `contains`, `to_upper`, `to_lower`
- **`list`** — `push`, `pop`, `len`, `map`, `filter`, `fold`
- **`io`** — `read_line`, `read_file`, `write_file`

---

## 15. Error Handling Summary

| Situation                   | Mechanism                          |
|-----------------------------|------------------------------------|
| Value may be absent         | `Perhaps<T>` / `nope`              |
| Operation may fail          | `Result<T, E>`                     |
| Unwrap with panic on absent | `.yolo()` method                   |
| Propagate error up          | `?` operator                       |
| Handle all cases            | `match`                            |

---

## 16. Design Decisions Log

This section records key decisions and their rationale for future reference.

| Decision | Choice | Rationale |
|---|---|---|
| Null handling | `Perhaps<T>` / `nope` | Forces explicit handling of absence at the type level |
| Error handling | `Result<T, E>` | Errors are values; no hidden control flow from exceptions |
| No classes | Structs + traits | Cleaner separation of data and behavior; avoids inheritance complexity |
| Enums | ADTs with data-carrying variants | Enables `Perhaps`, `Result`, and expressive domain modeling |
| Type inference | Local inference, annotated boundaries | Reduces noise without sacrificing clarity at API boundaries |
| Memory | Reference counting (runtime) | Simpler implementation; no ownership semantics exposed to the user |
| Void return | Omit `->` annotation | Less noise; `()` only appears when explicitly needed as a type argument |
| Generic syntax | `<T>` (Rust-style) | Consistent with the overall Rust-inspired aesthetic |
| Function keyword | `fun` | Carried over from the original language |
| Mutability | `let` / `mut` | Carried over; consistent with the immutability-by-default philosophy |
| For loops | C-style + for-in | C-style carried over; for-in added for iterator ergonomics |
| Pattern matching | `match` (Rust-style) | Natural fit for ADTs and `Perhaps`/`Result` handling |
