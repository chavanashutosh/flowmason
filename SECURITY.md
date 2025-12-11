# Security Policy

## Supported Versions

FlowMason security updates are provided for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 2.0.x   | :white_check_mark: |
| < 2.0   | :x:                |

Only the latest minor version within a major version receives security updates. For example, if version 2.1.0 is released, version 2.0.x will continue to receive security updates, but older versions will not.

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security vulnerability in FlowMason, please report it responsibly.

### How to Report

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please report security vulnerabilities by:

1. **Opening a private security advisory** on GitHub:
   - Go to the [Security tab](https://github.com/chavanashutosh/flowmason/security) in the repository
   - Click "Report a vulnerability"
   - Fill out the security advisory form

2. **Or contact directly**:
   - Open a private issue with the "Security" label
   - Include detailed information about the vulnerability

### What to Include

When reporting a security vulnerability, please provide:

- **Description**: Clear description of the vulnerability
- **Steps to Reproduce**: Detailed steps to reproduce the issue
- **Affected Versions**: Which versions are affected
- **Impact**: Potential impact and severity
- **Proof of Concept**: If possible, provide a proof-of-concept or exploit code
- **Suggested Fix**: If you have ideas for fixing the issue

**Preferred Format**: Text-based descriptions with proof-of-concept scripts are preferred over screenshots or videos.

### What Happens Next

1. **Acknowledgment**: We will acknowledge receipt of your report within 48 hours
2. **Investigation**: We will investigate and verify the vulnerability
3. **Fix Development**: We will develop a fix for the vulnerability
4. **Release**: We will release a security update
5. **Disclosure**: After the fix is released, we may disclose the vulnerability (with your permission)

### Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Fix Timeline**: Depends on severity, typically 30-90 days

## Security Best Practices

### For Users

- **Keep Updated**: Always use the latest supported version
- **Secure Configuration**: Follow security best practices in configuration
- **Environment Variables**: Keep sensitive credentials in environment variables, not in code
- **Network Security**: If deploying publicly (with commercial license), use HTTPS
- **Access Control**: Implement proper authentication and authorization
- **Regular Updates**: Monitor for security updates and apply them promptly

### For Developers

- **Dependency Updates**: Keep dependencies up to date
- **Input Validation**: Always validate and sanitize user input
- **Error Handling**: Don't expose sensitive information in error messages
- **Authentication**: Implement secure authentication mechanisms
- **Secrets Management**: Never commit secrets or API keys to the repository
- **Code Review**: Review code for security issues before merging

## Security Considerations

### Known Security Considerations

1. **API Keys and Credentials**: 
   - Store securely in environment variables
   - Never commit to version control
   - Rotate regularly

2. **JWT Tokens**:
   - Use strong, randomly generated secrets
   - Set appropriate expiration times
   - Validate tokens properly

3. **Database Security**:
   - Use parameterized queries to prevent SQL injection
   - Implement proper access controls
   - Encrypt sensitive data at rest

4. **Network Security**:
   - Use HTTPS in production
   - Implement rate limiting
   - Validate and sanitize all inputs

5. **Dependencies**:
   - Regularly update dependencies
   - Review dependency security advisories
   - Use tools like `cargo audit` to check for vulnerabilities

### Non-Qualifying Issues

The following issues are generally **not considered security vulnerabilities**:

- **Denial of Service (DoS)**: Unless it's a trivial attack vector
- **Social Engineering**: Issues requiring user interaction or deception
- **Physical Access**: Issues requiring physical access to the system
- **Configuration Issues**: Misconfigurations that don't represent code vulnerabilities
- **Information Disclosure**: Disclosure of non-sensitive information (e.g., version numbers)
- **Self-XSS**: Cross-site scripting that only affects the user performing the action
- **Missing Security Headers**: Unless they enable a specific attack vector
- **Theoretical Attacks**: Attacks that are not practically exploitable

If you're unsure whether an issue qualifies as a security vulnerability, please report it anyway and we'll evaluate it.

## Security Updates

Security updates are released as:

- **Patch Versions**: For critical security fixes (e.g., 2.0.1 → 2.0.2)
- **Minor Versions**: For security fixes with other improvements (e.g., 2.0.x → 2.1.0)

Security advisories are published in:
- GitHub Security Advisories
- Release notes
- Project documentation

## Responsible Disclosure

We follow responsible disclosure practices:

1. **Private Reporting**: Vulnerabilities are reported privately
2. **Fix Development**: We develop fixes before public disclosure
3. **Coordinated Release**: Security updates are released with appropriate notice
4. **Credit**: We credit security researchers who report vulnerabilities (with permission)

## Security Contact

For security-related questions or concerns:

- **GitHub Security Advisory**: [Report a vulnerability](https://github.com/chavanashutosh/flowmason/security)
- **Issues**: Open a private issue with the "Security" label
- **General Questions**: Open a regular issue for security best practices questions

## Security Resources

- [Rust Security Advisory Database](https://rustsec.org/)
- [Cargo Audit](https://github.com/rustsec/rustsec/tree/main/cargo-audit)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Rust Secure Code Guidelines](https://anssi-fr.github.io/rust-guide/)

## Acknowledgments

We thank security researchers and contributors who help keep FlowMason secure by responsibly reporting vulnerabilities.

---

**Last Updated**: 2025

**Note**: This security policy may be updated periodically. Please check back for the latest information.
