Contributing guidelines
=======================

This project does not accept external contributions,
neither from humans, nor from LLMs;
unless they are directly invited by the maintainers.

When invited to contribute, please follow these simple guidelines.

#### Table Of Contents
- [Overview](#overview)
- [Contribution workflow](#contribution-workflow)
    * [Proposing changes](#proposing-changes)
    * [Preparing PRs](#preparing-prs)
    * [Writing Git commit messages](#writing-git-commit-messages)
    * [Peer review](#peer-review)
- [Coding conventions](#coding-conventions)
- [Security](#security)
- [Testing](#testing)
    * [Workspace and feature tests](#workspace-and-feature-tests)
    * [Platform and standalone fixtures](#platform-and-standalone-fixtures)
    * [Miri and audit checks](#miri-and-audit-checks)
    * [Writing tests](#writing-tests)
- [Going further](#going-further)

Overview
--------

* Before adding any code dependencies, check with the maintainers if this is okay.
* Write properly formatted comments: they should be English sentences, eg:

        // Return the current UNIX time.

* Read the DCO and make sure all commits are signed off, using `git commit -s`.
* Follow the guidelines when proposing code changes (see below).
* Write properly formatted git commits (see below).
* Run the tests with `cargo test --workspace --all-features`.
* Make sure you run `rustfmt` on your code (see below details).
* Please don't file an issue to ask a question. Each repository - or  
  GitHub organization has a "Discussions" with Q&A section; please post your
  questions there. You'll get faster results by using this channel.

Contribution Workflow
---------------------
The codebase is maintained using the "contributor workflow" where everyone
without exception contributes patch proposals using "pull requests". This
facilitates social contribution, easy testing and peer review.

To contribute a patch, the workflow is a as follows:

1. Fork Repository
2. Create topic branch
3. Commit patches

In general commits should be atomic and diffs should be easy to read. For this
reason do not mix any formatting fixes or code moves with actual code changes.
Further, each commit, individually, should compile and pass tests, in order to
ensure git bisect and other automated tools function properly.

When adding a new feature thought must be given to the long term technical debt.
Every new features should be covered by unit tests.

When refactoring, structure your PR to make it easy to review and don't hesitate
to split it into multiple small, focused PRs.

Commits should cover both the issue fixed and the solution's rationale.
These [guidelines](https://chris.beams.io/posts/git-commit/) should be kept in
mind.

To facilitate communication with other contributors, the project is making use
of GitHub's "assignee" field. First check that no one is assigned and then
comment suggesting that you're working on it. If someone is already assigned,
don't hesitate to ask if the assigned party or previous commenters are still
working on it if it has been awhile.

### Proposing changes

When proposing changes via a pull-request or patch:

* Isolate changes in separate commits to make the review process easier.
* Don't make unrelated changes, unless it happens to be an obvious improvement to
  code you are touching anyway ("boyscout rule").
* Rebase on `master` when needed.
* Keep your changesets small, specific and uncontroversial, so that they can be
  merged more quickly.
* If the change is substantial or requires re-architecting certain parts of the
  codebase, write a proposal in English first, and get consensus on that before
  proposing the code changes.

### Preparing PRs

The main library development happens in the `master` branch. This branch must
always compile without errors (using Travis CI). All external contributions are
made within PRs into this branch.

Prerequisites that a PR must satisfy for merging into the `master` branch:
* the tip of any PR branch must compile and pass unit tests with no errors, with
  every feature combination (including compiling the fuzztests) on MSRV, stable
  and nightly compilers (this is partially automated with CI, so the rule
  is that we will not accept commits which do not pass GitHub CI);
* contain all necessary tests for the introduced functional (either as a part of
  commits, or, more preferably, as separate commits, so that it's easy to
  reorder them during review and check that the new tests fail without the new
  code);
* contain all inline docs for newly introduced API and pass doc tests;
* be based on the recent `master` tip from the original repository at.

NB: reviewers may run more complex test/CI scripts, thus, satisfying all the
requirements above is just a preliminary, but not necessary sufficient step for
getting the PR accepted as a valid candidate PR for the `master` branch.

Additionally, to the `master` branch some repositories may have `develop` branch
for any experimental developments. This branch may not compile and should not be
used by any projects depending on the library.

### Writing Git commit messages

A properly formed git commit subject line should always be able to complete the
following sentence:

     If applied, this commit will _____

In addition, it should be capitalized and *must not* include a period.

For example, the following message is well formed:

     Add support for .gif files

While these ones are **not**: `Adding support for .gif files`,
`Added support for .gif files`.

When it comes to formatting, here's a model git commit message[1]:

     Capitalized, short (50 chars or less) summary

     More detailed explanatory text, if necessary.  Wrap it to about 72
     characters or so.  In some contexts, the first line is treated as the
     subject of an email and the rest of the text as the body.  The blank
     line separating the summary from the body is critical (unless you omit
     the body entirely); tools like rebase can get confused if you run the
     two together.

     Write your commit message in the imperative: "Fix bug" and not "Fixed bug"
     or "Fixes bug."  This convention matches up with commit messages generated
     by commands like git merge and git revert.

     Further paragraphs come after blank lines.

     - Bullet points are okay, too.

     - Typically a hyphen or asterisk is used for the bullet, followed by a
       single space, with blank lines in between, but conventions vary here.

     - Use a hanging indent.

### Peer review

Anyone may participate in peer review which is expressed by comments in the pull
request. Typically reviewers will review the code for obvious errors, as well as
test out the patch set and opine on the technical merits of the patch. PR should
be reviewed first on the conceptual level before focusing on code style or
grammar fixes.

Coding Conventions
------------------
Our CI enforces [clippy's](https://github.com/rust-lang/rust-clippy)
[default linting](https://rust-lang.github.io/rust-clippy/rust-1.52.0/index.html)
and [rustfmt](https://github.com/rust-lang/rustfmt) formatting defined by rules
in [.rustfmt.toml](./.rustfmt.toml). The linter should be run with current
stable rust compiler, while formatter requires nightly version due to the use of
unstable formatting parameters.

If you use rustup, to lint locally you may run the following instructions:

```console
rustup component add clippy
rustup component add fmt
cargo +stable clippy --workspace --all-features
cargo +nightly fmt --all
```

Security
--------
Responsible disclosure of security vulnerabilities helps prevent user loss of
privacy. If you believe a vulnerability may affect other implementations, please
inform them. Guidelines for a responsible disclosure can be found in 
[SECURITY.md](./SECURITY.md) file in the project root.

Note that some of our projects are currently considered "pre-production".
Such projects can be distinguished by the absence of `SECURITY.md`. In such
cases there are no special handling of security issues; please simply open
an issue on GitHub.

Testing
-------
All changes are verified across feature combinations, targets, and safety checks
automated in the [CI workflows](./.github/workflows). The minimum supported Rust
version (MSRV) is **1.100.0** (matching stable `rust-toolchain.toml`), which is
required for allocator-aware collection support (`Vec<T, A>`).

### Workspace and feature tests

Tests in the root crate cover:

* Unit tests in `src/byte_array.rs` (`ByteArray` implementations, requiring no
  optional features).
* Doctests in `src/traits.rs` (including `compile_fail` checks), `src/macros.rs`
  (under `alloc`), and [README.md](./README.md) (compiled via `ReadmeDoctests`
  when `std` is enabled).
* Integration tests in `tests/`, sharing the counting `TestAlloc` allocator from
  `tests/common/mod.rs`:
  - `tests/collection.rs` (`alloc`): `CollectionBase`, `Collection`, and
    `CollectionBytes` contracts across `alloc`, `std`, `ascii`, `indexmap`, and
    `hashbrown`.
  - `tests/capacity.rs` (`alloc`): `CollectionCapacity` contract, UTF-8 byte
    width reservation for `String`, and deduplication on sets and maps.
  - `tests/alloc.rs` (`alloc`): `CollectionAlloc` contract with `Global`,
    `System`, `TestAlloc`, and `hashbrown`.
  - `tests/keyed.rs` (`alloc`): `CollectionMap`, `CollectionEntry`, and
    `MapEntry` contracts, non-hashable `Ord`-only keys, and `IndexMap` ordering.
  - `tests/set.rs` (`alloc`): `CollectionSet` contract and `IndexSet` ordering.
  - `tests/macros.rs` (`alloc`, `std`): collection construction macros (`s!`,
    `bmap!`, `bset!`, `list!`, `deque!`, `heap!`, `map!`, `set!`).
  - `tests/fixed.rs` (`heapless`): fixed-capacity collections, fallible
    insertion, full-capacity panic contracts, and generic interoperability.

Run the CI feature matrix locally with `--locked`:

```console
cargo test --workspace --locked --no-fail-fast
cargo test --workspace --locked --no-default-features --no-fail-fast
cargo test --workspace --locked --no-default-features --features alloc --no-fail-fast
cargo test --workspace --locked --no-default-features --features heapless --no-fail-fast
cargo test --workspace --locked --all-features --no-fail-fast
```

To check every valid feature combination (using `cargo-hack`) and verify builds
across toolchains (`1.100.0`, `stable`, `beta`, `nightly`):

```console
cargo hack check --locked --all-targets --feature-powerset --exclude-features default,all
cargo +1.100.0 check --workspace --locked --all-targets --all-features
```

### Platform and standalone fixtures

The subdirectories `tests/no_std_consumer`, `tests/embedded`, and
`tests/wasm_consumer` define separate standalone packages (`[workspace]`), so
`cargo test --workspace` from the repository root does not compile or run them.

To verify macro expansion in a `#![no_std]` crate:

```console
cargo check --manifest-path tests/no_std_consumer/Cargo.toml
```

To verify `no_std` and bare-metal target compilation (note that `ascii` requires
`alloc::sync::Arc` and is excluded on targets without atomic CAS such as
`thumbv6m-none-eabi` and `riscv32imc-unknown-none-elf`):

```console
cargo check --locked --target thumbv6m-none-eabi --no-default-features
cargo check --locked --target thumbv6m-none-eabi --no-default-features --features heapless
cargo check --locked --target thumbv6m-none-eabi --no-default-features --features alloc
cargo check --locked --target thumbv6m-none-eabi --no-default-features --features heapless,indexmap,hashbrown
```

The same checks also run against `thumbv7em-none-eabi`,
`riscv32imc-unknown-none-elf`, and `wasm32-unknown-unknown` (with `ascii`
included on `thumbv7em-none-eabi` and `wasm32-unknown-unknown`).

To build and run the Cortex-M firmware fixture under QEMU (requires the
`thumbv7em-none-eabi` and `thumbv7m-none-eabi` targets and `qemu-system-arm`):

```console
cd tests/embedded
cargo build --locked --release --target thumbv7em-none-eabi --bin noheap
cargo build --locked --release --target thumbv7em-none-eabi --bin heap --features heap
cargo run --locked --release --target thumbv7m-none-eabi --bin noheap
cargo run --locked --release --target thumbv7m-none-eabi --bin heap --features heap
```

`noheap` links without an allocator (failing the build if anything pulls in
`alloc`) and prints `collections: no-heap contract OK`; `heap` runs on a 16 KiB
`embedded-alloc` heap and prints `collections: bare-metal heap contract OK`.

To build and run the WebAssembly contract fixture under Node.js (requires the
`wasm32-unknown-unknown` target and Node):

```console
cd tests/wasm_consumer
cargo build --release --target wasm32-unknown-unknown
node run.mjs target/wasm32-unknown-unknown/release/collections_wasm_consumer.wasm run
node run.mjs target/wasm32-unknown-unknown/release/collections_wasm_consumer.wasm run_failing
```

`run` must exit with status `0`; `run_failing` is a negative control that traps
and must exit with status `1`.

### Miri and audit checks

Although the library forbids `unsafe` code (`#![forbid(unsafe_code)]`), CI runs
the test suite under [Miri](https://github.com/rust-lang/miri) on `nightly` to
verify dependencies and the test allocator for undefined behavior, checks the
docs.rs build with warnings denied, and audits dependencies against
[deny.toml](./deny.toml) using `cargo-deny`:

```console
rustup +nightly component add miri rust-src
cargo +nightly miri test --locked --all-features
RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc --workspace --locked --all-features --no-deps
cargo deny --locked check
```

### Writing tests

When adding or updating tests:

* Gate integration tests with `#![cfg(feature = "...")]` or
  `#[cfg(feature = "...")]` so `--no-default-features` and single-feature runs
  (`alloc`, `heapless`) compile and pass cleanly.
* Keep `src/` free of `unsafe`; reuse `TestAlloc` in `tests/common/mod.rs` for
  allocator tests and document any `unsafe` invariants in tests with
  `// SAFETY:` comments (enforced via `[lints]` in [Cargo.toml](./Cargo.toml)).
* Cover failure and edge cases: `try_reserve`/`try_push` errors on overflow or
  full capacity, `"fixed-capacity collection is full"` panics on infallible
  `heapless` operations, `item_count()` vs `byte_len()` vs `capacity()` with
  multi-byte UTF-8 `char`s, non-`Default` `BuildHasher`s, and `compile_fail`
  doctests for intentional API omissions.
* Update `tests/no_std_consumer`, `tests/embedded`, `tests/wasm_consumer`, and
  [README.md](./README.md) examples when adding traits, implementations, or
  macros.
* Keep code coverage at or above the 95% project and patch target configured in
  [codecov.yml](./codecov.yml).

Going further
-------------
You may be interested in Jon Atack guide on
[How to review Bitcoin Core PRs][Review] and [How to make Bitcoin Core PRs][PR].
While there are differences between the projects in terms of context and
maturity, many of the suggestions offered apply to this project.

Overall, have fun :)

[1]: http://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html
[Review]: https://github.com/jonatack/bitcoin-development/blob/master/how-to-review-bitcoin-core-prs.md
[PR]: https://github.com/jonatack/bitcoin-development/blob/master/how-to-make-bitcoin-core-prs.md
