# Authentication Usage Guide

## Overview

The authentication system uses **HTTP-only cookie**-based sessions for security:

1. User creates a household or logs in with password
2. Server sets `session` cookie (httpOnly, Secure, SameSite=Strict)
3. Client automatically includes cookie in subsequent requests
4. Server validates session before processing protected routes

## Session Storage

- **Cookie**: `Set-Cookie: session=...` (httpOnly, cannot be accessed via JavaScript)
- **Duration**: 8 hours, then automatically invalidated
- **Scope**: Domain-scoped, automatic CSRF protection via SameSite=Strict

## Authentication Middleware

All protected routes use `auth_middleware` automatically:
- Extracts session from httpOnly cookie
- Validates HMAC signature
- Checks session expiration
- Verifies household ownership
- Injects `AuthSession` into handler context

No manual validation needed in handlers.

## API Endpoints

### Public Endpoints (No Auth Required)
- `POST /api/households` - Create household
- `POST /api/households/{household_id}/login` - Login to household
- `GET /health` - Health check

### Protected Endpoints (Auth Required)
- `POST /api/households/{household_id}/logout` - Logout
- `GET/POST /api/households/{household_id}/transactions` - Transactions
- `GET/POST /api/households/{household_id}/categories` - Categories
- `GET/POST /api/households/{household_id}/goals` - Goals

## Example Requests

### Create Household
```bash
curl -X POST http://localhost:3000/api/households \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Smith Family",
    "password": "secure-password-123"
  }'
```

Response:
```json
{
  "id": "hh-uuid",
  "name": "Smith Family",
  "session_id": "sess-uuid",
  "expires_at": "2026-04-12T20:38:00.000Z"
}
```

### Login
```bash
curl -X POST http://localhost:3000/api/households/hh-uuid/login \
  -H "Content-Type: application/json" \
  -d '{"password": "secure-password-123"}'
```

### Access Protected Resource
```bash
# Use --cookie-jar to save cookies, -b to send them
curl -X GET http://localhost:3000/api/households/hh-uuid/transactions \
  -b cookies.txt
```

Or in JavaScript (fetch auto-includes cookies):
```javascript
fetch('/api/households/hh-uuid/transactions', {
  credentials: 'include',  // IMPORTANT: include cookies
})
```

## Session Validation Flow

1. **Token Extraction**: Get session_id from Authorization header or query parameter
2. **Database Lookup**: Query sessions table for matching session_id
3. **Expiration Check**: Verify session hasn't expired (compares current time with expires_at)
4. **Ownership Verification**: Ensure session.household_id matches request's household_id
5. **Allow/Deny**: Process request if all checks pass, reject with 401 Unauthorized otherwise

## Security Features

- ✅ Passwords hashed with Argon2 (min 12 chars, uppercase/lowercase/digits required)
- ✅ Sessions stored server-side (no JWT)
- ✅ 8-hour expiration TTL
- ✅ Household ownership verification
- ✅ HTTP-only cookies (XSS-protected, cannot be stolen via JavaScript)
- ✅ Secure flag (HTTPS only in production)
- ✅ SameSite=Strict (CSRF protection)
- ✅ HMAC-signed session tokens (cryptographic validation)

## Token Lifespan

- Sessions expire after **8 hours**
- Each login generates a new session_id
- Logout immediately invalidates the session_id
- Expired sessions are purged from database on validation attempt
