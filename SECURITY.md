# Security Policy

## Reporting a vulnerability

Please do not disclose suspected security vulnerabilities in public issues,
discussions, pull requests, or commits.

Report them privately through [GitHub Security Advisories](https://github.com/Prathewsh/vscode_rust/security/advisories/new). Include enough detail for the issue to be reproduced and assessed:

- A clear description of the impact and affected component (`vscode-native`,
  `code-cli`, tunnel support, or packaging resources).
- Steps to reproduce, a proof of concept, and any prerequisites.
- The affected revision, operating system, and Rust version.
- Suggested mitigations, if you have them.

Please avoid accessing data, interrupting services, or expanding the scope of a
test beyond what is necessary to demonstrate the issue.

## What to expect

Maintainers will acknowledge a valid private report, investigate it, and work
with the reporter on a fix and coordinated disclosure when appropriate. Please
allow time for triage; do not publish details until a fix or disclosure plan has
been agreed.

## Supported code

Security fixes are considered for the current `main` branch. Older revisions
and locally modified builds may require an upgrade or reproduction on `main`
before a fix can be evaluated.

## Scope

This policy covers code and first-party resources in this repository. Issues in
third-party crates, operating systems, GPU drivers, or hosted tunnel services
may also need to be reported to their respective maintainers. A report that
affects this project because of a dependency is still welcome.
