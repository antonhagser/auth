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

/**
 * The Email service (gRPC) handles all email related tasks.
 */
class Email implements EmailServiceServer {
    [method: string]: UntypedHandleCall;

    /**
     * Sends a verification email to the specified email address.
     *
     * @param call The gRPC call object
     * @param callback The callback function
     */
    public sendVerificationEmail(
        call: ServerUnaryCall<SendVerificationEmailRequest, SendEmailResponse>,
        callback: sendUnaryData<SendEmailResponse>
    ): void {
        (async () => {
            console.log("Received sendVerificationEmail request");

            // Get request data
            const request = call.request;
            const emailData = request.emailData;
            const emailApplication = request.emailApplication;

            // Validate request data
            if (!emailData) {
                return callback(new Error("Email data is undefined"));
            }

            if (!emailApplication) {
                return callback(new Error("Email application is undefined"));
            }

            console.log("Email data: %s", JSON.stringify(emailData));

            // Render email template to HTML
            const emailHtml = renderVerificationEmail({
                url: request.verificationURL,
                code: request.verificationCode,
            });

            // Send email
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

            // Return response
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
