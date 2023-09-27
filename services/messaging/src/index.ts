import "source-map-support/register";
import 'dotenv/config'
import { Server, ServerCredentials } from "@grpc/grpc-js";

import { Service, ServiceService } from "./services/service";
import { Email, EmailServiceService } from "./services/email";

// Do not use @grpc/proto-loader
const server = new Server({
    "grpc.max_receive_message_length": -1,
    "grpc.max_send_message_length": -1,
});

function main() {
    console.log("Starting gRPC server on port 50051");
    server.addService(ServiceService, new Service());
    server.addService(EmailServiceService, new Email());
    server.bindAsync(
        "0.0.0.0:50051",
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
