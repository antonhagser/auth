import "source-map-support/register";
import "dotenv/config";
import { Server, ServerCredentials } from "@grpc/grpc-js";

import { Service, ServiceService } from "./services/service";
import { Email, EmailServiceService } from "./services/email";

// Configure gRPC server
const server = new Server({
    "grpc.max_receive_message_length": -1,
    "grpc.max_send_message_length": -1,
});

function main() {
    // Set up server
    const port = process.env.PORT || 50051;
    const host = process.env.HOST || "localhost";
    const url = `${host}:${port}`;

    // Start server
    console.log(`Starting gRPC server on port ${port}...`);
    server.addService(ServiceService, new Service());
    server.addService(EmailServiceService, new Email());

    // gRPC uses async methods to start the server
    server.bindAsync(
        url,
        ServerCredentials.createInsecure(),
        (err: Error | null, bindPort: number) => {
            if (err) {
                throw err;
            }

            server.start();
        }
    );
}

main();
