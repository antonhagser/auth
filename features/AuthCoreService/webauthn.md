# WebAuthn Service Documentation

The WebAuthn Service is responsible for implementing passwordless authentication using Web Authentication (WebAuthn), a web standard for secure public-key authentication. WebAuthn enables users to authenticate with biometric or security key credentials, providing a more secure and user-friendly alternative to traditional password-based authentication.

## Features

-   Passwordless authentication using biometric or security key credentials
-   Support for multiple authenticators, such as hardware tokens or built-in platform authenticators
-   Secure public-key cryptography for user authentication
-   Improved user experience by eliminating the need for remembering and entering passwords
-   Protection against phishing, replay, and man-in-the-middle attacks

## REST Routes

All routes will start with `/api/v0/`.

## WebAuthn Service

-   POST `/auth/webauthn/register`: Start the registration process for a new WebAuthn credential.
-   POST `/auth/webauthn/register/complete`: Complete the registration process by validating and storing the new WebAuthn credential.
-   POST `/auth/webauthn/login`: Start the authentication process using a registered WebAuthn credential.
-   POST `/auth/webauthn/login/complete`: Complete the authentication process by validating the provided WebAuthn assertion.

## Usage Example

To register a new WebAuthn credential, make a POST request to `/api/v0/auth/webauthn/` register:

```HTTP
POST /api/v0/auth/webauthn/register
```

This request will return a challenge and other necessary information to be used by the client to create a new WebAuthn credential. The user will interact with their authenticator (e.g., a security key or built-in platform authenticator) to generate the credential.

Once the credential is created, complete the registration process by making a POST request to `/api/v0/auth/webauthn/register/complete` with the created credential:

```HTTP
POST /api/v0/auth/webauthn/register/complete
```

After successfully registering the WebAuthn credential, the user can authenticate using the passwordless authentication process. Start the process by making a POST request to `/api/v0/auth/webauthn/login`:

```HTTP
POST /api/v0/auth/webauthn/login
```

This request will return a challenge and other necessary information to be used by the client to generate a WebAuthn assertion. The user will interact with their authenticator to generate the assertion.

Complete the authentication process by making a POST request to `/api/v0/auth/webauthn/login/complete` with the generated assertion:

```HTTP
POST /api/v0/auth/webauthn/login/complete
```

Upon successful validation of the assertion, the user will be authenticated and granted access to the system.

## Additional Notes

-   Ensure that your application uses HTTPS, as WebAuthn requires a secure context.
-   Store the WebAuthn credentials securely, as they are the keys to user authentication.
-   Consider implementing a recovery mechanism (e.g., backup codes or alternative authentication methods) in case users lose access to their authenticator.
-   Implement proper user management and account linking to handle users with multiple WebAuthn credentials or users who have both password-based and WebAuthn credentials.
