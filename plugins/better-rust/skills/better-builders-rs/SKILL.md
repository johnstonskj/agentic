---
description: Guidelines for implementing builder patterns.
name: better-builders-rs

context:
  applyTo: '**/*.rs'
---
# Implementing Builder Patterns

The builder pattern in Rust is a design pattern used to construct complex
objects step by step. It allows for more readable and maintainable code by
providing a clear and fluent interface for setting up an object's properties
before finalizing its creation. In implementing the builder pattern, it is
common to provide methods that set individual fields (by convention prefixed
with `with_`) and a final `build` method that consumes the builder and returns
the constructed object.

However, there are different variations of the builder pattern depending on:

1. The mutability of `self` in field setting methods.
2. Whether the `build` method consumes itself, mutates itself, and can 
   therefore be reused multiple times.
3. Whether the builder provides a dedicated constructor method (e.g., `new` or 
   `default`) for initialization or is constructed by an associated method on 
   the output type.

While the first and second of these choices are technically independent, they
are often correlated in practice, as builders that consume themselves typically
also have `with_` methods that take `self` by value, while reusable builders
often have `with_` methods that take `&mut self`.

*Principles*

* Builders provide users a clear and fluent interface for constructing complex
  objects step by step.
* Builders separate the construction of an object from its representation, 
  allowing for more flexible and maintainable code.
* Self-consuming builders are suitable when the builder is intended for single
  use and may transfer ownership of resources to the constructed object.
* Self-mutable builders are suitable when the builder is intended to be reused
  multiple times and maintains internal state that can be modified with each
  method call.

## Self Consuming Builders

This variant of the pattern is the preferred choice when the builder is intended
to be consumed after a single use, usually because some resource or state is
transferred to the constructed object.

This also has the advantage that the builder can be chained directly as
constructed without *requiring* an intermediate mutable variable.

```rust
let val = builder::default()
  .with_field(22)
  .build();
```

### Field setter method

A field setter method for a self-consuming builder typically takes `self` by
value and returns the modified builder in the same manner. By adding the 
keyword **`mut`** to the parameter, the method can modify the builder's
internal state before returning it.

```rust
fn with_field(mut self, field: Type) -> Self {
    self.field = field;
    self
}
```

However, if you are writing a trait to capture your builder, you will find that
you cannot use `mut self` in the trait method signature, and instead must rely
on `self` alone. Additionally, as we are passing self by value, we need to 
ensure that the type implementing the trait is sized, which is why we include
a `where Self: Sized` bound.

```rust
fn with_field(self, field: Type) -> Self
where
    Self: Sized;
```

### Build method

The build method for a self-consuming builder typically takes `self` by value,
consuming the builder and producing the constructed object. It may return the
object directly (infallible builder) or wrap it in a `Result` to handle 
potential errors during construction (fallible builder).

```rust
fn build(self) -> OutputType // ...

fn build(self) -> Result<OutputType, ErrorType> // ...
```

## Self Mutable Builders

This variant of the pattern is used when the builder is intended to be reused
multiple times. The builder maintains internal state that can be modified with
each `with_` method call, and the `build` method does not consume the builder,
allowing it to be used again for constructing another object.

**IF** the builder has a single, resource or other stateful field that would
prevent safe reuse, but it is likely a client *would* want to create multiple
similar instances, use a `build_with` method (see below).

Such a builder *should clearly* document its behavior regarding field value
reuse; for example, are all set fields copied, cloned, or referenced on each
call to `build`?

```rust
let mut bld = builder::default();
let val = bld
  .with_field(22)
  .build();
```

### Field setter method

The following is the signature of a field setter method for a self-mutable
builder. This does not need to be altered for traits.

```rust
fn with_field(&mut self, field: Type) -> &mut Self;
```

### Build method

The build method for a self-mutable builder also takes `&mut self`, allowing
the builder to be reused. It may return the constructed object directly
(infallible builder) or wrap it in a `Result` to handle potential errors
(fallible builder).

```rust
fn build(&mut self) -> OutputType;
fn build(&mut self) -> Result<OutputType, ErrorType>;
```

If necessary, a `build_with` method can be provided to handle stateful fields
that require special handling during construction. Its signature should be a
clear combination of the build method and field setter method.

```rust
fn build_with(&mut self, stateful_field: Type) -> OutputType;
fn build_with(&mut self, stateful_field: Type) -> Result<OutputType, ErrorType>;
```

## Builder Constructors

How a builder is constructed can vary depending on the design choices and the
intended usage patterns. Common approaches include providing a `default`
implementation, a `new`/custom constructor, or an associated function on the
output type itself. Each of these has pros/cons that should be considered in
the context of the specific builder design.

The simplest approach is to derive or implement the `Default` trait for the 
builder, providing a convenient and idiomatic way to create a new instance
with default values. However, does `Builder::default().build()` make sense?
If a default builder *can* construct a valid instance of the output type
without requiring additional field values, then not only does this makes sense
but its usage is also idiomatic and optiomal in Rust. Otherwise, provide a
custom constructor or associated function that enforces the minimum required 
fields to guarantee a valid instance.

```rust
#[derive(Default)]
struct Builder {
    field: Type,
}
```

The custom constructor approach allows for more control over the initialization
of the builder, ensuring that required fields are provided and potentially
performing conversions or validations upfront. The goal is that the builder
*should* always be in a valid state and can therefore produce a valid instance
of the output type barring any cross-field inconsistencies that are resolved
during `build`.

```rust
impl Builder {
    fn new_foo(foo: TypeA) -> Self {
        Self {
            field: foo.into(),
        }
    }
    fn new_bar(bar: TypeB) -> Self {
        Self {
            field: bar.into(),
        }
    }
}
```

Finally, an associated function on the output type itself can be provided to return
a builder instance. This approach is useful when the output type wants to control
the creation of its builder, potentially enforcing certain invariants or providing
a more discoverable API for users. In this approach we use the name `builder` for
cases akin to the `default` constructor above, custom constructors may be used
instead.

```rust
impl OutputType {
    fn builder() -> Builder {
        Builder::default()
    }
}
```

A variant of this approach is to have the associated function return an opaque type
that implements a builder trait. This allows the output type to control the exact
builder type while still exposing a consistent interface to users, *or* simply to 
keep the concrete builder type hidden from the user.

```rust
impl OutputType {
    fn builder() -> impl OutputBuilder {
        Builder::default()
    }
}
```

### Short Names

While the convention for builders is to provide field setter methods that start 
with `with_`, it is also common to provide shorter, more convenient names for
well known fields where such names are easily recognizable and improve code
readability. It is also useful to think about the common order in which fields
may be set in typical usage and provide short names accordingly.

The following examples illustrate the use of both conventional `with_` methods 
and shorter, more convenient method names for setting fields in builders. It is
a relatively simple change noting that the hostname is required and so can be
an effective constructor for the builder whereas the port is optional and always
secondary, the `and_` prefix captures this relationship in the method naming.

```rust
let address = NetAddress::default()
    .with_hostname("example.com")
    .with_port(80)
    .build();
    
let address = NetAddress::hostname("example.com")
    .and_port(80)
    .build();
```

This is a more complex set of changes that make perfect sense in a specific
domain -- RDF and ontology modeling. While the former is a perfectly valid
builder interface it is cumbersome and less expressive compared to the latter,
more domain-specific naming. In the case of the method `sub_class_of` it
even reverses the logical meaning of the original `with_super_class_uri`
method, making the code more intuitive.

```rust 
let class = ClassBuilder::default()
    .with_iri("https:://example.com/ns/Type")
    .with_super_class_uri("https:://example.com/ns/SuperType")
    .with_name_label("Type")
    .build();

let class = ClassBuilder::new("https:://example.com/ns/Type")
    .sub_class_of("https:://example.com/ns/SuperType")
    .named("Type")
    .build();
```

Having said this, if in doubt about the universality of a short name, provide
both, or even multiple short names, but alway provide the `with_` form and
have the short names delegate to it internally. This ensures consistency and
reduces the risk of errors while still providing the convenience of shorter,
more readable method names.

## Implementation

The following are steps to design and implement a builder for a given output
type.

1. Given a type to build, generically `Output`, determine how the builder
   should behave in terms of reusability, error handling, and stateful
   fields.
   1. Determine if the builder should be reusable or consumed after a 
      single build.
   2. Determine if the builder has rules during `build` that may cause
      errors or not.
   3. *If* reusable, determine if the builder requires a `build_with` 
      method for stateful fields.
2. Determine a constructor strategy for the builder, such as whether to
   provide a `default` implementation, custom constructor, or both, *or*
   whether the builder instance is provided by the `Output` type.
3. Determine the list of fields that the builder should have and how they
   should be set, including whether to provide conventional `with_` methods,
   short names (see below), or both.
4. Implement the builder, either directly, or by providing a trait and impl.
   1. a trait that defines the builder interface is often used where the output
      type provides the builder through an associated method thus keeping the
      actual builder type private.

### Crate particle-traits

The crate [particle-traits](https://crates.io/crates/particle-traits)
provides traits for implementing various types of builders in Rust.

| Builder                 | Constructor                 | Description                                                                           |
|-------------------------|-----------------------------|---------------------------------------------------------------------------------------|
| `OnceBuilder`           | `HasOnceBuilder`            | Consumes the builder and may return an error.                                         |
| `InfallibleOnceBuilder` | `HasInfallibleOnceBuilder`  | Consumes the builder and does not return an error.                                    |
| `MutBuilder`            | `HasMutBuilder`             | Can be used multiple times, may mutate internal state, and may return an error.       |
| `InfallibleMutBuilder`  | `HasInfallibleMutBuilder`   | Can be used multiple times, may mutate internal state, and does not return an error.  |
| `Builder`               | `HasBuilder`                | Can be used multiple times and may return an error.                                   |
| `InfallibleBuilder`     | `HasInfallibleBuilder`      | Can be used multiple times and does not return an error.                              |

Using this crate the steps above are amended in the following manner.

**1.1** Determine whether the builder requires the reuse behavior of
`OnceBuilder`, `MutBuilder`, or `Builder`.

**1.2** *If* the builder *does not* return errors during the build process, 
choose the infallible variant of the trait you selected in *1.1*.

**4** *If* you are implementing a trait for your builder, make the trait
you chose in *1.2* the super-type of your trait, setting `Output` (and
`Error` if applicable) for the builder.

**1.3** Add any additional `build_with` methods to your trait/implementation as
needed.

**2**: 

* Add an implementation of `Default` if applicable.
* Add any custom constructors to your trait/implementation as needed.
* Add any associated constructor functions to the output type if applicable.

**3** Add any short name methods to your trait/implementation as needed.

#### Original Structure

* Assuming a structure type `Address` with fields `street`, `city`, `zip` code, and country exists.
* We wish to create a builder for the `Address` structure.
* We want the builder to be reusable and have error handling.
* Builders are provided by the Address type.

```rust
#[derive(Debug, Clone)]
pub struct Address {
    street: String,
    city: String,
    zip: String,
    country: String,
}
```

#### Has Builder Implementation

To require clients to construct builders using `Address::builder()`, implement
the `HasOnceBuilder` trait for the `Address` type.

```rust
use particle_traits::HasOnceBuilder;

impl HasOnceBuilder for Address {
    type Address = Self;
    type Builder = AddressBuilder;

    fn builder() -> Self::Builder {
        AddressBuilder::default()
    }
}
```

#### Address Builder Implementation

The Address builder is implemented as a separate struct with optional fields
for each component of the address. This allows for flexible construction and
error handling during the build process. An implementation of the `OnceBuilder`
trait provides the `build` method to construct the final `Address` instance,
handling any missing fields as errors. Field setters are provided for each
component, allowing clients to set values before building the final `Address`
instance.

Note: this example breaks the rule that a builder should always be ready to 
generate a valid instance as its default state is for all values to be `None`.

```rust
use particle_traits::OnceBuilder;

#[derive(Default)]
struct AddressBuilder {
    street: Option<String>,
    city: Option<String>,
    zip: Option<String>,
    country: Option<String>,
}

impl OnceBuilder for AddressBuilder {
    type Output = Address;
    type Error = String;

    fn build(&mut self) -> Result<Self::Output, Self::Error> {
        Ok(Address {
            street: self.street.take().ok_or("street is missing")?,
            city: self.city.take().ok_or("city is missing")?,
            zip: self.zip.take().ok_or("zip is missing")?,
            country: self.country.take().ok_or("country is missing")?,
        })
    }
}

impl AddressBuilder {
    pub fn with_street(mut self, street: String) -> Self {
        self.street = Some(street);
        self
    }

    pub fn with_city(mut self, city: String) -> Self {
        self.city = Some(city);
        self
    }

    pub fn with_zip(mut self, zip: String) -> Self {
        self.zip = Some(zip);
        self
    }

    pub fn with_country(mut self, country: String) -> Self {
        self.country = Some(country);
        self
    }
}
```
