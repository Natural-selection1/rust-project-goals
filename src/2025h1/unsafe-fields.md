# Unsafe Fields

| Metadata         |                                    |
| :--------------- | :--------------------------------- |
| Point of contact | @jswrenn                           |
| Status           | Accepted                           |
| Zulip channel    | N/A                                |
| Tracking issue   | [rust-lang/rust-project-goals#273] |

## Summary

Design and implement a mechanism for denoting when fields carry library safety invariants.

## Motivation

The absence of a mechanism for denoting the presence of library safety invariants increases both the risk of working with `unsafe` code and the difficulty of evaluating its soundness.

### The status quo

Presently, Rust lacks mechanisms for denoting when fields carry library safety invariants, and for enforcing extra care around their use. Consequently, to evaluate the soundness of `unsafe` code (i.e., code which relies on safety invariants being upheld), it is not enough to check the contents of `unsafe` blocks — one must check all places (including safe contexts) in which safety invariants might be violated. (See [*The Scope of Unsafe*])

For example, consider this idealized `Vec`:

```rust
pub struct Vec<T> {
    data: Box<[MaybeUninit<T>]>,
    len: usize,
}
```

Although `len` is bound by a safety invariant, it is trivial to violate its invariant in entirely safe code:

```rust
impl Vec<T> {
    pub fn evil(&mut self) {
        self.len += 2;
    }
}
```

Rust cannot enforce that modifications of `len` require `unsafe`, because the language does not provide the programmer a way of communicating to the compiler that `len` carries safety invariants.

[*The Scope of Unsafe*]: https://www.ralfj.de/blog/2016/01/09/the-scope-of-unsafe.html

### The "shiny future" we are working towards

Rust programmers will use the `unsafe` keyword to denote fields that carry library safety invariants; e.g.:

```rust
struct Vec<T> {
    // SAFETY: The elements `data[i]` for
    // `i < len` are in a valid state.
    unsafe data: Box<[MaybeUninit<T>]>,
    unsafe len: usize,
}
```

Rust will require that usages of `unsafe` fields which could violate their safety invariants must _only_ occur within `unsafe` contexts.

### The next 6 months

In the next six months, we will iterate on the design and implementation of unsafe fields. An RFC for `unsafe` fields will be accepted, and a candidate implementation will — at the very least — be ready to enter the stabilization process.

## Design axioms

The design of `unsafe` fields is guided by three axioms:

1. **Unsafe Fields Denote Safety Invariants**
   A field _should_ be marked `unsafe` if it carries arbitrary library safety invariants with respect to its enclosing type.
2. **Unsafe Usage is Always Unsafe**
   Uses of `unsafe` fields which could violate their invariants _must_ occur in the scope of an `unsafe` block.
3. **Safe Usage is Usually Safe**
   Uses of `unsafe` fields which cannot violate their invariants _should not_ require an unsafe block.

## Ownership and team asks

**Owner:** @jswrenn

| Task                         | Owner(s) or team(s)  | Notes                          |
| ---------------------------- | -------------------- | ------------------------------ |
| Discussion and moral support | ![Team][] [lang]     |                                |
| Author RFC                   | @jhpratt             | [RFC3458], [Living Design Doc] |
| Implementation               | @veluca93            |                                |
| Standard reviews             | ![Team][] [compiler] |                                |
| Design meeting               | ![Team][] [lang]     |                                |
| RFC decision                 | ![Team][] [lang]     |                                |

Ongoing discussion on [Zulip][].

[Zulip]: https://rust-lang.zulipchat.com/#narrow/channel/213817-t-lang/topic/unsafe.20fields.20RFC
[RFC3458]: https://github.com/rust-lang/rfcs/pull/3458
[Living Design Doc]: https://hackmd.io/SJqXa_8lJe

### Definitions

Definitions for terms used above:

- _Discussion and moral support_ is the lowest level offering, basically committing the team to nothing but good vibes and general support for this endeavor.
- _Author RFC_ and _Implementation_ means actually writing the code, document, whatever.
- _Design meeting_ means holding a synchronous meeting to review a proposal and provide feedback (no decision expected).
- _RFC decisions_ means reviewing an RFC and deciding whether to accept.
- _Org decisions_ means reaching a decision on an organizational or policy matter.
- _Secondary review_ of an RFC means that the team is "tangentially" involved in the RFC and should be expected to briefly review.
- _Stabilizations_ means reviewing a stabilization and report and deciding whether to stabilize.
- _Standard reviews_ refers to reviews for PRs against the repository; these PRs are not expected to be unduly large or complicated.
- _Prioritized nominations_ refers to prioritized lang-team response to nominated issues, with the expectation that there will be _some_ response from the next weekly triage meeting.
- _Dedicated review_ means identifying an individual (or group of individuals) who will review the changes, as they're expected to require significant context.
- Other kinds of decisions:
  - [Lang team experiments](https://lang-team.rust-lang.org/how_to/experiment.html) are used to add nightly features that do not yet have an RFC. They are limited to trusted contributors and are used to resolve design details such that an RFC can be written.
  - Compiler [Major Change Proposal (MCP)](https://forge.rust-lang.org/compiler/mcp.html) is used to propose a 'larger than average' change and get feedback from the compiler team.
  - Library [API Change Proposal (ACP)](https://std-dev-guide.rust-lang.org/development/feature-lifecycle.html) describes a change to the standard library.

## Frequently asked questions

TBD
