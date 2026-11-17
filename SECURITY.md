# Security

We take the security of our software products and services seriously, which 
includes all source code repositories managed through our GitHub organizations.

If you believe you have found a security vulnerability in any of our repository 
that meets [definition of a security vulnerability][definition], please report 
it to us as described below.

## What Is a Vulnerability in This Crate

The crate defines traits and implements them by forwarding to the methods of
the standard, `hashbrown`, `indexmap`, `ascii` and `heapless` collections. Its
own code contains no `unsafe` (`#![forbid(unsafe_code)]`). Other code relies on
the documented contract of the traits when it enforces limits on data, so the
following is treated as a security vulnerability:

* Memory unsafety or undefined behaviour which can be triggered through the
  safe API of this crate.
* A fallible method which panics or aborts instead of returning its error:
  `CollectionCapacity::try_reserve` or `CollectionCapacity::try_push`.
* A method which reports a wrong size or state, so that a bound checked with it
  can be exceeded: `item_count`, `is_empty`, `byte_len`, `item_byte_len` or
  `capacity`.
* An insertion or removal which silently loses, duplicates or corrupts data in
  a way the documentation does not describe.
* A published package or a release which contains code that is not in this
  repository, and a weakness of the CI configuration which allows this.

The following behaviour is documented in the "Security considerations" section
of `README.md` and is not a vulnerability of this crate:

* The infallible methods (`push`, `with_capacity`, `with_capacity_in`,
  `reserve`, `extend`, `collect`) panic or abort when memory cannot be
  allocated, and panic when a fixed-capacity collection is full.
* The default hashers are not seeded randomly on `wasm32-unknown-unknown` and
  on bare-metal targets, which makes hash maps and hash sets with these
  hashers open to hash flooding there.
* `item_count()` takes linear time for strings.

A vulnerability in a dependency which does not involve the code of this crate
should be reported to the maintainers of that dependency and to the
[RustSec advisory database][rustsec]. Tell us as well if it affects the users
of this crate, so that the required versions can be changed.

## Reporting Security Issues

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them to the repository maintainers by sending a **GPG
encrypted e-mail** to _all maintainers of a specific repo_ using their GPG keys.

A list of repository maintainers and their keys and e-mail addresses are 
provided inside MAINTAINERS.md file.

You should receive a response within 72 hours. If for some reason you do not, 
please follow up via email to ensure we received your original message. 

Please include the requested information listed below (as much as you can 
provide) to help us better understand the nature and scope of the possible 
issue:

* Type of issue (e.g. buffer overflow, SQL injection, cross-site scripting, etc.)
* Full paths of source file(s) related to the manifestation of the issue
* The location of the affected source code (tag/branch/commit or direct URL)
* Any special configuration required to reproduce the issue
* Step-by-step instructions to reproduce the issue
* Proof-of-concept or exploit code (if possible)
* Impact of the issue, including how an attacker might exploit the issue

This information will help us triage your report more quickly.

## Preferred Languages

We prefer all communications to be in English.

## Policy

We follow the principle of [Coordinated Vulnerability Disclosure][disclosure].

[definition]: https://aka.ms/opensource/security/definition
[disclosure]: https://aka.ms/opensource/security/cvd
[rustsec]: https://rustsec.org
