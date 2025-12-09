# Authentication

FlowMason supports two authentication methods: JWT tokens and API keys.

## JWT Authentication

### Register

Create a new user account:

```bash
POST /api/v1/auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "secure-password"
}
```

### Login

Get a JWT token:

```bash
POST /api/v1/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "secure-password"
}
```

Response:

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user_id": "user-123",
  "email": "user@example.com"
}
```

### Using JWT Token

Include the token in the Authorization header:

```bash
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

## API Key Authentication

### Create API Key

```bash
POST /api/v1/auth/api-keys
Authorization: Bearer <jwt-token>
Content-Type: application/json

{
  "name": "My API Key"
}
```

Response:

```json
{
  "id": "key-123",
  "key": "fm_abc123def456...",
  "name": "My API Key",
  "created_at": "2025-01-01T00:00:00Z"
}
```

**Important**: Save the key immediately. It won't be shown again.

### Using API Key

Include the API key in the Authorization header:

```bash
Authorization: Bearer fm_abc123def456...
```

### List API Keys

```bash
GET /api/v1/auth/api-keys
Authorization: Bearer <jwt-token>
```

### Delete API Key

```bash
DELETE /api/v1/auth/api-keys/:id
Authorization: Bearer <jwt-token>
```

## Token Expiration

JWT tokens expire after a set period. When a token expires:

1. You'll receive a `401 Unauthorized` response
2. Login again to get a new token
3. API keys don't expire but can be revoked

## Security Best Practices

1. **Never commit tokens or API keys** to version control
2. **Use environment variables** to store credentials
3. **Rotate API keys** regularly
4. **Use HTTPS** in production
5. **Limit API key permissions** when possible

## Example: cURL

```bash
# Register
curl -X POST http://localhost:3000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"password"}'

# Login
curl -X POST http://localhost:3000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"password"}'

# Use token
curl http://localhost:3000/api/v1/flows \
  -H "Authorization: Bearer <token>"
```

