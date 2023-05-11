# Services

The platform is split into multiple services, each with their own purpose. This document will give a brief overview of each service and what they do.

## MVP Services

These services are the minimum required to have a working platform.

### Authentication Service (AuthCore)

The authentication service is the main service of the platform. It handles all authentication requests and it is responsible for validating credentials and generating tokens. It's also the main source of truth for Application Accounts for the platform. It stores all the data related to Application Accounts unless the application is connected with a third-party user database such as LDAP or Active Directory.

#### Features

-   Authenticate users and generate tokens
-   Verify tokens such as access tokens and refresh tokens, email verification tokens, and password reset tokens
-   Connect with third-party user databases such as LDAP and Active Directory

### Platform Service

The dashboard is the main interface for the platform. It is an application that allows administrators to manage users, clients, and other settings.

The dashboard communicates with the authentication service to perform actions such as managing users, applications and organizations.

#### Features

-   Manage applications, organizations, and users
-   Configure authentication flows
-   Manage API keys
-   Manage user groups
-   Set up SSO with SAML
-   Set up connections to external user databases (LDAP, Active Directory, etc.)

### Logging Service

The logging service is responsible for storing logs from the authentication service and the dashboard. It is also responsible for generating reports.

The logging service would be powered by a pub/sub system. The authentication service and the dashboard would publish logs to the logging service. The logging service would then store the logs in a database and generate reports.

There would also be public endpoints for querying logs and reports.

### Messaging Service (Email/SMS)

The email and SMS service is responsible for sending emails and SMS messages. It is used by the authentication service to send emails and SMS messages to users.

## Future Services

These services are not required for the platform to work, but they would be useful to have.

### Webhook Service

The webhook service will be responsible for sending webhooks to applications when certain events happen. For example, when a user logs in, the webhook service will send a webhook to the application to notify it of the event.

This service would most likely take advantage of a similar pub/sub system as the one used by the logging service. The authentication service would publish events to the webhook service. The webhook service would then send webhooks to applications.

### Core Service

This service would be the source of truth for data such as applications, organizations, and users. It would be responsible for storing and managing this data. The authentication service would then be responsible for validating credentials and generating tokens.

### Internal Monitoring Service

This service would be responsible for monitoring the platform. It would monitor the health of the platform and send alerts when something goes wrong.

While it could possible use the Messaging Service to send alerts, it would most likely use a seperate implementation to ensure that alerts are sent even if the Messaging Service is down.
