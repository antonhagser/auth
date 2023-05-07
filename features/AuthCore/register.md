# Registration Service Documentation

The Registration Service is responsible for managing user registration within the system. It provides an endpoint for user registration, allowing new users to create an account and gain access to the system. This service ensures that new users provide valid and unique email addresses and conform to the system's password requirements.

## Features

-   User registration with email and password validation.
-   Email uniqueness verification to prevent duplicate accounts.
-   Password strength enforcement based on the system's password policy.
-   Optional email confirmation to verify user's email address ownership.

## REST Routes

All routes will start with `/api/v0/`.

## Registration Service

-   POST `/auth/register`: Register a new user with their email and password.
-   POST `/auth/verify/email/send`: Send a verification code to a user's email address.
-   POST `/auth/verify/email/confirm`: Confirm the provided email verification code.
-   POST `/auth/verify/sms/send`: Send a verification code to a user's phone number via SMS.
-   POST `/auth/verify/sms/confirm`: Confirm the provided SMS verification code.

## Usage Example

To register a new user, make a POST request to `/api/v0/auth/register` with the email and password:

```HTTP
POST /api/v0/auth/register
Content-Type: application/json

{
  "email": "newuser@example.com",
  "password": "Str0ngP@ssw0rd"
}
```

If the email is unique and the password meets the system's requirements, the response will indicate successful registration.

## Additional Notes
