# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.5.x   | :white_check_mark: |
| 0.4.x   | :white_check_mark: |
| < 0.4   | :x:                |

## Reporting a Vulnerability

We take security seriously at VantisOffice. If you discover a security vulnerability, please report it responsibly.

### How to Report

1. **DO NOT** create a public GitHub issue for security vulnerabilities
2. Email your findings to: **security@vantiscorp.com**
3. Include the following information:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 5 business days
- **Resolution Target**: Within 30 days for critical issues

### What to Expect

- We will acknowledge receipt of your report
- We will provide an estimated timeline for a fix
- We will notify you when the vulnerability is fixed
- We will credit you in the security advisory (unless you prefer anonymity)

### Scope

The following are in scope:
- All VantisOffice crates and modules
- CI/CD pipeline configurations
- Cryptographic implementations (vantis-vault, vantis-pqc)
- WASM sandbox (wasm-sandbox)
- Mobile bindings (vantis-mobile)

### Security Features

VantisOffice implements the following security measures:
- **Zero Trust Architecture**: All operations verified
- **Post-Quantum Cryptography**: Kyber KEM + Dilithium signatures
- **End-to-End Encryption**: ChaCha20-Poly1305 + AES-256-GCM
- **WASM Sandboxing**: Isolated execution environment
- **Cargo Audit**: Automated dependency vulnerability scanning
- **CodeQL Analysis**: Static code analysis for security issues
- **Snyk Integration**: Continuous vulnerability monitoring