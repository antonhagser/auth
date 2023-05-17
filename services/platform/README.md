# Platform

The main platform, has a REST api for managing users, organizations, applications, and other settings. It is the entrypoint for the application (from a busines perspective) and is the main interface for administrators.

## Features

-   Manage applications, organizations, and users
-   Configure authentication flows
-   Manage API keys
-   Manage user groups
-   Set up SSO with SAML
-   Set up connections to external user databases (LDAP, Active Directory, etc.)
-   Manage email and SMS templates

## Environment variables

| Name | Description | Default value |
| --- | --- | --- |
| `DATABASE_URL` | Postgres database URL | nil |
