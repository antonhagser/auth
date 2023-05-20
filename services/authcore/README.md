# AuthCore service

## Environment variables

| Name | Description | Default value |
| --- | --- | --- |
| `DATABASE_URL` | Postgres database URL | nil |

## Microservice stratergy

This service is a microservice that provides authentication and authorization for other services. It is designed to be stateless and can be scaled horizontally.

To make sure that the service is stateless, it uses the following strategies:

-   All data is stored in the database.
-   Minimum communication with other services (only when necessary; email verification).
-   Data replication, Application exists in both AuthCore and Platform services to minimize communication between services.
    -   Application is updated in AuthCore service over gRPC when it is updated in Platform service.
