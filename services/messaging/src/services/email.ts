import {
    ServerUnaryCall,
    UntypedHandleCall,
    sendUnaryData,
} from "@grpc/grpc-js";

import {
    EmailServiceServer,
    EmailServiceService,
    SendVerificationEmailRequest,
    SendEmailResponse,
} from "../models/email";

import renderVerificationEmail from "../templates/verify";
import sendEmail from "../email/send";

class Email implements EmailServiceServer {
    [method: string]: UntypedHandleCall;

    public sendVerificationEmail(
        call: ServerUnaryCall<SendVerificationEmailRequest, SendEmailResponse>,
        callback: sendUnaryData<SendEmailResponse>
    ): void {
        (async () => {
            console.log("Received sendVerificationEmail request");

            const request = call.request;
            const emailData = request.emailData;
            const emailApplication = request.emailApplication;

            if (!emailData) {
                return callback(new Error("Email data is undefined"));
            }

            if (!emailApplication) {
                return callback(new Error("Email application is undefined"));
            }

            console.log("Email data: %s", JSON.stringify(emailData));

            console.log(
                "Request verification URL: %s",
                request.verificationURL
            );
            console.log(
                "Request verification code: %s",
                request.verificationCode
            );

            const emailHtml = renderVerificationEmail({
                url: request.verificationURL,
                code: request.verificationCode,
            });

            const subject = "Confirm your email address";
            const emailOptions = {
                from: emailData.from,
                to: emailData.to,
                subject: subject,
                cc: emailData.cc,
                bcc: emailData.bcc,
                replyTo: emailData.replyTo,
                html: emailHtml,
            };

            let result = await sendEmail(emailOptions);

            if (!result) {
                return callback(new Error("Email failed to send"));
            }

            console.log("Sending email to: %s", emailData.to);

            return callback(null, {
                emailId: "", // TODO: Implement Email IDs and logging
                message: "Email sent successfully",
            });
        })().catch((err) => {
            console.error("Error in sendVerificationEmail:", err);
            return callback(new Error("Internal server error"));
        });
    }
}

export { Email, EmailServiceService };
