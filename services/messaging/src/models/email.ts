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

export interface EmailData {
  from: string;
  to: string[];
  cc: string[];
  bcc: string[];
  replyTo: string;
}

export interface Attachment {
  filename: string;
  data: Buffer;
  /** e.g., "image/jpeg" */
  mimetype: string;
}

export interface EmailApplication {
  name: string;
}

export interface SendVerificationEmailRequest {
  verificationURL?: string | undefined;
  verificationCode?: string | undefined;
  emailData?: EmailData | undefined;
  emailApplication?: EmailApplication | undefined;
}

export interface SendEmailResponse {
  /** e.g., "Email sent successfully" */
  message: string;
  /** ID or reference for the sent email */
  emailId: string;
}

function createBaseEmailData(): EmailData {
  return { from: "", to: [], cc: [], bcc: [], replyTo: "" };
}

export const EmailData = {
  encode(message: EmailData, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.from !== "") {
      writer.uint32(10).string(message.from);
    }
    for (const v of message.to) {
      writer.uint32(18).string(v!);
    }
    for (const v of message.cc) {
      writer.uint32(58).string(v!);
    }
    for (const v of message.bcc) {
      writer.uint32(66).string(v!);
    }
    if (message.replyTo !== "") {
      writer.uint32(74).string(message.replyTo);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): EmailData {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseEmailData();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.from = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.to.push(reader.string());
          continue;
        case 7:
          if (tag !== 58) {
            break;
          }

          message.cc.push(reader.string());
          continue;
        case 8:
          if (tag !== 66) {
            break;
          }

          message.bcc.push(reader.string());
          continue;
        case 9:
          if (tag !== 74) {
            break;
          }

          message.replyTo = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): EmailData {
    return {
      from: isSet(object.from) ? String(object.from) : "",
      to: Array.isArray(object?.to) ? object.to.map((e: any) => String(e)) : [],
      cc: Array.isArray(object?.cc) ? object.cc.map((e: any) => String(e)) : [],
      bcc: Array.isArray(object?.bcc) ? object.bcc.map((e: any) => String(e)) : [],
      replyTo: isSet(object.replyTo) ? String(object.replyTo) : "",
    };
  },

  toJSON(message: EmailData): unknown {
    const obj: any = {};
    if (message.from !== "") {
      obj.from = message.from;
    }
    if (message.to?.length) {
      obj.to = message.to;
    }
    if (message.cc?.length) {
      obj.cc = message.cc;
    }
    if (message.bcc?.length) {
      obj.bcc = message.bcc;
    }
    if (message.replyTo !== "") {
      obj.replyTo = message.replyTo;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<EmailData>, I>>(base?: I): EmailData {
    return EmailData.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<EmailData>, I>>(object: I): EmailData {
    const message = createBaseEmailData();
    message.from = object.from ?? "";
    message.to = object.to?.map((e) => e) || [];
    message.cc = object.cc?.map((e) => e) || [];
    message.bcc = object.bcc?.map((e) => e) || [];
    message.replyTo = object.replyTo ?? "";
    return message;
  },
};

function createBaseAttachment(): Attachment {
  return { filename: "", data: Buffer.alloc(0), mimetype: "" };
}

export const Attachment = {
  encode(message: Attachment, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.filename !== "") {
      writer.uint32(10).string(message.filename);
    }
    if (message.data.length !== 0) {
      writer.uint32(18).bytes(message.data);
    }
    if (message.mimetype !== "") {
      writer.uint32(26).string(message.mimetype);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Attachment {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAttachment();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.filename = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.data = reader.bytes() as Buffer;
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.mimetype = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): Attachment {
    return {
      filename: isSet(object.filename) ? String(object.filename) : "",
      data: isSet(object.data) ? Buffer.from(bytesFromBase64(object.data)) : Buffer.alloc(0),
      mimetype: isSet(object.mimetype) ? String(object.mimetype) : "",
    };
  },

  toJSON(message: Attachment): unknown {
    const obj: any = {};
    if (message.filename !== "") {
      obj.filename = message.filename;
    }
    if (message.data.length !== 0) {
      obj.data = base64FromBytes(message.data);
    }
    if (message.mimetype !== "") {
      obj.mimetype = message.mimetype;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<Attachment>, I>>(base?: I): Attachment {
    return Attachment.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<Attachment>, I>>(object: I): Attachment {
    const message = createBaseAttachment();
    message.filename = object.filename ?? "";
    message.data = object.data ?? Buffer.alloc(0);
    message.mimetype = object.mimetype ?? "";
    return message;
  },
};

function createBaseEmailApplication(): EmailApplication {
  return { name: "" };
}

export const EmailApplication = {
  encode(message: EmailApplication, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): EmailApplication {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseEmailApplication();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.name = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): EmailApplication {
    return { name: isSet(object.name) ? String(object.name) : "" };
  },

  toJSON(message: EmailApplication): unknown {
    const obj: any = {};
    if (message.name !== "") {
      obj.name = message.name;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<EmailApplication>, I>>(base?: I): EmailApplication {
    return EmailApplication.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<EmailApplication>, I>>(object: I): EmailApplication {
    const message = createBaseEmailApplication();
    message.name = object.name ?? "";
    return message;
  },
};

function createBaseSendVerificationEmailRequest(): SendVerificationEmailRequest {
  return { verificationURL: undefined, verificationCode: undefined, emailData: undefined, emailApplication: undefined };
}

export const SendVerificationEmailRequest = {
  encode(message: SendVerificationEmailRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.verificationURL !== undefined) {
      writer.uint32(18).string(message.verificationURL);
    }
    if (message.verificationCode !== undefined) {
      writer.uint32(26).string(message.verificationCode);
    }
    if (message.emailData !== undefined) {
      EmailData.encode(message.emailData, writer.uint32(34).fork()).ldelim();
    }
    if (message.emailApplication !== undefined) {
      EmailApplication.encode(message.emailApplication, writer.uint32(42).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SendVerificationEmailRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSendVerificationEmailRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 2:
          if (tag !== 18) {
            break;
          }

          message.verificationURL = reader.string();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.verificationCode = reader.string();
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.emailData = EmailData.decode(reader, reader.uint32());
          continue;
        case 5:
          if (tag !== 42) {
            break;
          }

          message.emailApplication = EmailApplication.decode(reader, reader.uint32());
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): SendVerificationEmailRequest {
    return {
      verificationURL: isSet(object.verificationURL) ? String(object.verificationURL) : undefined,
      verificationCode: isSet(object.verificationCode) ? String(object.verificationCode) : undefined,
      emailData: isSet(object.emailData) ? EmailData.fromJSON(object.emailData) : undefined,
      emailApplication: isSet(object.emailApplication) ? EmailApplication.fromJSON(object.emailApplication) : undefined,
    };
  },

  toJSON(message: SendVerificationEmailRequest): unknown {
    const obj: any = {};
    if (message.verificationURL !== undefined) {
      obj.verificationURL = message.verificationURL;
    }
    if (message.verificationCode !== undefined) {
      obj.verificationCode = message.verificationCode;
    }
    if (message.emailData !== undefined) {
      obj.emailData = EmailData.toJSON(message.emailData);
    }
    if (message.emailApplication !== undefined) {
      obj.emailApplication = EmailApplication.toJSON(message.emailApplication);
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<SendVerificationEmailRequest>, I>>(base?: I): SendVerificationEmailRequest {
    return SendVerificationEmailRequest.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<SendVerificationEmailRequest>, I>>(object: I): SendVerificationEmailRequest {
    const message = createBaseSendVerificationEmailRequest();
    message.verificationURL = object.verificationURL ?? undefined;
    message.verificationCode = object.verificationCode ?? undefined;
    message.emailData = (object.emailData !== undefined && object.emailData !== null)
      ? EmailData.fromPartial(object.emailData)
      : undefined;
    message.emailApplication = (object.emailApplication !== undefined && object.emailApplication !== null)
      ? EmailApplication.fromPartial(object.emailApplication)
      : undefined;
    return message;
  },
};

function createBaseSendEmailResponse(): SendEmailResponse {
  return { message: "", emailId: "" };
}

export const SendEmailResponse = {
  encode(message: SendEmailResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.message !== "") {
      writer.uint32(10).string(message.message);
    }
    if (message.emailId !== "") {
      writer.uint32(18).string(message.emailId);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SendEmailResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSendEmailResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.message = reader.string();
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.emailId = reader.string();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  fromJSON(object: any): SendEmailResponse {
    return {
      message: isSet(object.message) ? String(object.message) : "",
      emailId: isSet(object.emailId) ? String(object.emailId) : "",
    };
  },

  toJSON(message: SendEmailResponse): unknown {
    const obj: any = {};
    if (message.message !== "") {
      obj.message = message.message;
    }
    if (message.emailId !== "") {
      obj.emailId = message.emailId;
    }
    return obj;
  },

  create<I extends Exact<DeepPartial<SendEmailResponse>, I>>(base?: I): SendEmailResponse {
    return SendEmailResponse.fromPartial(base ?? ({} as any));
  },
  fromPartial<I extends Exact<DeepPartial<SendEmailResponse>, I>>(object: I): SendEmailResponse {
    const message = createBaseSendEmailResponse();
    message.message = object.message ?? "";
    message.emailId = object.emailId ?? "";
    return message;
  },
};

export type EmailServiceService = typeof EmailServiceService;
export const EmailServiceService = {
  sendVerificationEmail: {
    path: "/messaging.email.EmailService/SendVerificationEmail",
    requestStream: false,
    responseStream: false,
    requestSerialize: (value: SendVerificationEmailRequest) =>
      Buffer.from(SendVerificationEmailRequest.encode(value).finish()),
    requestDeserialize: (value: Buffer) => SendVerificationEmailRequest.decode(value),
    responseSerialize: (value: SendEmailResponse) => Buffer.from(SendEmailResponse.encode(value).finish()),
    responseDeserialize: (value: Buffer) => SendEmailResponse.decode(value),
  },
} as const;

export interface EmailServiceServer extends UntypedServiceImplementation {
  sendVerificationEmail: handleUnaryCall<SendVerificationEmailRequest, SendEmailResponse>;
}

export interface EmailServiceClient extends Client {
  sendVerificationEmail(
    request: SendVerificationEmailRequest,
    callback: (error: ServiceError | null, response: SendEmailResponse) => void,
  ): ClientUnaryCall;
  sendVerificationEmail(
    request: SendVerificationEmailRequest,
    metadata: Metadata,
    callback: (error: ServiceError | null, response: SendEmailResponse) => void,
  ): ClientUnaryCall;
  sendVerificationEmail(
    request: SendVerificationEmailRequest,
    metadata: Metadata,
    options: Partial<CallOptions>,
    callback: (error: ServiceError | null, response: SendEmailResponse) => void,
  ): ClientUnaryCall;
}

export const EmailServiceClient = makeGenericClientConstructor(
  EmailServiceService,
  "messaging.email.EmailService",
) as unknown as {
  new (address: string, credentials: ChannelCredentials, options?: Partial<ClientOptions>): EmailServiceClient;
  service: typeof EmailServiceService;
};

declare const self: any | undefined;
declare const window: any | undefined;
declare const global: any | undefined;
const tsProtoGlobalThis: any = (() => {
  if (typeof globalThis !== "undefined") {
    return globalThis;
  }
  if (typeof self !== "undefined") {
    return self;
  }
  if (typeof window !== "undefined") {
    return window;
  }
  if (typeof global !== "undefined") {
    return global;
  }
  throw "Unable to locate global object";
})();

function bytesFromBase64(b64: string): Uint8Array {
  if (tsProtoGlobalThis.Buffer) {
    return Uint8Array.from(tsProtoGlobalThis.Buffer.from(b64, "base64"));
  } else {
    const bin = tsProtoGlobalThis.atob(b64);
    const arr = new Uint8Array(bin.length);
    for (let i = 0; i < bin.length; ++i) {
      arr[i] = bin.charCodeAt(i);
    }
    return arr;
  }
}

function base64FromBytes(arr: Uint8Array): string {
  if (tsProtoGlobalThis.Buffer) {
    return tsProtoGlobalThis.Buffer.from(arr).toString("base64");
  } else {
    const bin: string[] = [];
    arr.forEach((byte) => {
      bin.push(String.fromCharCode(byte));
    });
    return tsProtoGlobalThis.btoa(bin.join(""));
  }
}

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
