# Security Policy

## Reporting a Vulnerability

The security of Zterm is taken seriously. If you believe you have found a security vulnerability in this project, **please do not open a public GitHub issue**. Public disclosure of a vulnerability before it has been assessed and addressed puts all users at risk.

Instead, please report security vulnerabilities by opening a **GitHub Security Advisory** on this repository:

1. Navigate to the **Security** tab of this repository.
2. Click **"Advisories"** in the left sidebar.
3. Click **"New draft security advisory"**.
4. Fill in the details: a description of the vulnerability, steps to reproduce, potential impact, and any suggested mitigations you have in mind.

This creates a private draft visible only to the repository maintainers, allowing us to assess and address the issue before any public disclosure.

## What to Include in Your Report

To help us understand and reproduce the issue as quickly as possible, please include:

- A clear description of the vulnerability and its potential impact.
- Step-by-step instructions to reproduce the issue.
- The version or commit of Zterm where you observed the behaviour.
- Your operating system and Rust toolchain version, if relevant.
- Any proof-of-concept code, screenshots, or logs that illustrate the issue.
- Your suggested fix or mitigation, if you have one.

## Coordinated Disclosure

We follow a coordinated disclosure process:

1. You report the vulnerability privately via a GitHub Security Advisory.
2. We acknowledge receipt and begin assessment, typically within a few business days.
3. We work with you to understand the scope, develop a fix, and agree on a disclosure timeline.
4. Once a fix is ready and released, we publish a public security advisory crediting your report (unless you prefer to remain anonymous).

We ask that you give us a reasonable amount of time to address the vulnerability before making any public disclosure. In return, we commit to keeping you informed throughout the process.

## Scope

This security policy applies to the Zterm client codebase in this repository. It does not cover third-party dependencies — if you discover a vulnerability in an upstream dependency (e.g., Tokio, Alacritty, Hyper), please report it directly to the relevant upstream project.
