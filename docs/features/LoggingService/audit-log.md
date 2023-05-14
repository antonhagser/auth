# Audit Log Service Documentation

The Audit Log Service is responsible for recording and managing audit logs related to user activities and security events within the authentication system. This service helps administrators monitor and track actions taken by users or clients, and provides a method for auditing the platform's security.

## Features

-   Recording of user and client activities, such as authentication attempts, password resets, role changes, and more.
-   Filtering and searching capabilities for administrators to easily find specific events or activities.
-   Pagination support for managing large volumes of audit logs.
-   Exporting capabilities for external analysis or reporting purposes.

## REST Routes

All routes will start with /api/v0/.

## Audit Log Service

-   GET `/audit-logs`: Retrieve a list of audit logs (for clients' use)
-   GET `/audit-logs/:id`: Retrieve a specific audit log by ID (for clients' use)
-   POST `/audit-logs`: Create a new audit log entry (internal use only)
-   DELETE `/audit-logs/:id`: Delete a specific audit log entry by ID (for clients' use, with proper authorization)
-   GET `/audit-logs/export`: Export audit logs as a CSV file (for clients' use)

## Usage Example

To retrieve a list of audit logs, make a GET request to `/api/v0/audit-logs`:

```HTTP
GET /api/v0/audit-logs?page=1&limit=10&search=login&dateFrom=2023-01-01T00:00:00Z&dateTo=2023-01-31T23:59:59Z
```

This request will return a list of audit logs related to login events that occurred between January 1st and January 31st, 2023.

## Protobuf Definition

The following is an example protobuf definition for the Audit Log Service:

```protobuf
syntax = "proto3";

package auditlog;

import "google/protobuf/timestamp.proto";

message AuditLog {
  string id = 1;
  string user_id = 2;
  string client_id = 3;
  string action = 4;
  string description = 5;
  google.protobuf.Timestamp timestamp = 6;
}

service AuditLogService {
  rpc CreateAuditLog (CreateAuditLogRequest) returns (AuditLog);
}

message CreateAuditLogRequest {
  string user_id = 1;
  string client_id = 2;
  string action = 3;
  string description = 4;
}
```

## Proto Endpoints

-   CreateAuditLog: Creates a new audit log entry.

    Request:

    ```protobuf
    message CreateAuditLogRequest {
      string user_id = 1;
      string client_id = 2;
      string action = 3;
      string description = 4;
      google.protobuf.Timestamp timestamp = 6;
    }
    ```

    Response:

    ```protobuf
    message AuditLog {
      string id = 1;
      string user_id = 2;
      string client_id = 3;
      string action = 4;
      string description = 5;
      google.protobuf.Timestamp timestamp = 6;
    }
    ```

    Example Usage:

    ```bash
    $ grpcurl -plaintext -d '{"user_id": "123", "client_id": "456", "action": "login", "description": "User successfully logged in"}' localhost:50051 auditlog.AuditLogService/CreateAuditLog
    ```

    Note: The `id` field in the response will be automatically generated by the server.