import {
    ServerUnaryCall,
    UntypedHandleCall,
    sendUnaryData,
} from "@grpc/grpc-js";
import {
    GetVersionRequest,
    GetVersionResponse,
    ServiceServer,
    ServiceService,
} from "../models/messaging";

/**
 * The Service service (gRPC) handles all service related tasks such as versioning.
 */
class Service implements ServiceServer {
    [method: string]: UntypedHandleCall;

    /**
     * Returns the current version of the service.
     *
     * @param call The gRPC call object
     * @param callback The callback function
     */
    public getVersion(
        call: ServerUnaryCall<GetVersionRequest, GetVersionResponse>,
        callback: sendUnaryData<GetVersionResponse>
    ): void {
        const response: GetVersionResponse = {
            version: "1.0.0",
        };

        callback(null, response);
    }
}

export { Service, ServiceService };
