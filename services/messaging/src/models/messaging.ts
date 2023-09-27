/* eslint-disable */
import { ChannelCredentials, Client, makeGenericClientConstructor, Metadata } from "@grpc/grpc-js";
import type {
  CallOptions,
  ClientOptions,
  ClientUnaryCall,
  handleUnaryCall,
  ServiceError,
  UntypedServiceImplementation,
} from "@grpc/grpc-js";
import _m0 from "protobufjs/minimal";

export interface GetVersionRequest {
}

export interface GetVersionResponse {
  version: string;
}

function createBaseGetVersionRequest(): GetVersionRequest {
  return {};
}

export const GetVersionRequest = {
  encode(_: GetVersionRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GetVersionRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGetVersionRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(_: any): GetVersionRequest {
    return {};
  },

  toJSON(_: GetVersionRequest): unknown {
    const obj: any = {};
    return obj;
  },

  create<I extends Exact<DeepPartial<GetVersionRequest>, I>>(base?: I): GetVersionRequest {
    return GetVersionRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<GetVersionRequest>, I>>(_: I): GetVersionRequest {
    const message = createBaseGetVersionRequest();
    return message;
  },
};

function createBaseGetVersionResponse(): GetVersionResponse {
  return { version: "" };
}

export const GetVersionResponse = {
  encode(message: GetVersionResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.version !== "") {
      writer.uint32(10).string(message.version);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GetVersionResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGetVersionResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.version = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): GetVersionResponse {
    return { version: isSet(object.version) ? String(object.version) : "" };
  },

  toJSON(message: GetVersionResponse): unknown {
    const obj: any = {};
    if (message.version !== "") {
      obj.version = message.version;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<GetVersionResponse>, I>>(base?: I): GetVersionResponse {
    return GetVersionResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<GetVersionResponse>, I>>(object: I): GetVersionResponse {
    const message = createBaseGetVersionResponse();
    message.version = object.version ?? "";
    return message;
  },
};

export type ServiceService = typeof ServiceService;
export const ServiceService = {
  getVersion: {
    path: "/messaging.Service/GetVersion",
    requestStream: false,
    responseStream: false,
    requestSerialize: (value: GetVersionRequest) => Buffer.from(GetVersionRequest.encode(value).finish()),
    requestDeserialize: (value: Buffer) => GetVersionRequest.decode(value),
    responseSerialize: (value: GetVersionResponse) => Buffer.from(GetVersionResponse.encode(value).finish()),
    responseDeserialize: (value: Buffer) => GetVersionResponse.decode(value),
  },
} as const;

export interface ServiceServer extends UntypedServiceImplementation {
  getVersion: handleUnaryCall<GetVersionRequest, GetVersionResponse>;
}

export interface ServiceClient extends Client {
  getVersion(
    request: GetVersionRequest,
    callback: (error: ServiceError | null, response: GetVersionResponse) => void,
  ): ClientUnaryCall;
  getVersion(
    request: GetVersionRequest,
    metadata: Metadata,
    callback: (error: ServiceError | null, response: GetVersionResponse) => void,
  ): ClientUnaryCall;
  getVersion(
    request: GetVersionRequest,
    metadata: Metadata,
    options: Partial<CallOptions>,
    callback: (error: ServiceError | null, response: GetVersionResponse) => void,
  ): ClientUnaryCall;
}

export const ServiceClient = makeGenericClientConstructor(ServiceService, "messaging.Service") as unknown as {
  new (address: string, credentials: ChannelCredentials, options?: Partial<ClientOptions>): ServiceClient;
  service: typeof ServiceService;
};

type Builtin = Date | Function | Uint8Array | string | number | boolean | undefined;

type DeepPartial<T> = T extends Builtin ? T
  : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>>
  : T extends {} ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

type KeysOfUnion<T> = T extends T ? keyof T : never;
type Exact<P, I extends P> = P extends Builtin ? P
  : P & { [K in keyof P]: Exact<P[K], I[K]> } & { [K in Exclude<keyof I, KeysOfUnion<P>>]: never };

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
