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

class Service implements ServiceServer {
    [method: string]: UntypedHandleCall;

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
