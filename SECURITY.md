# Security policy

## Supported versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | Yes       |

## Reporting a vulnerability

Email security@your-org.example with details. Do not open public issues for undisclosed vulnerabilities.

## Hardening checklist

- Enable mTLS between control plane components in production.
- Keep `agents.chaos.enabled: false` unless running isolated game days.
- Restrict NATS with authentication and TLS.
- Scope Kubernetes adapter RBAC to required namespaces only.
