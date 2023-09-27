import { SendMailOptions, createTransport } from "nodemailer";

const transporter = createTransport({
    service: process.env.NODEMAILER_SERVICE,
    host: process.env.NODEMAILER_HOST,
    secure: true,
    port: 465,
    auth: {
        user: process.env.NODEMAILER_EMAIL,
        pass: process.env.NODEMAILER_PASSWORD,
    },
});

export interface Data {
    from: string;
    to: string | string[];
    subject: string;
    cc: string | string[];
    bcc: string | string[];
    replyTo: string | string[];
    html: string;
}

export default async function sendEmail(
    option: SendMailOptions
): Promise<boolean> {
    console.log("Sending email to: %s", option.to);

    console.debug("Service: %s", process.env.NODEMAILER_SERVICE);
    console.debug("Host: %s", process.env.NODEMAILER_HOST);
    console.debug("Email: %s", process.env.NODEMAILER_EMAIL);

    try {
        let result = await transporter.sendMail(option);

        console.log("Mail sent: ", result.accepted.length > 0 ? true : false);

        if (result.accepted.length > 0) {
            return true;
        }

        return false;
    } catch (error) {
        console.log(error);
        return false;
    }
}
