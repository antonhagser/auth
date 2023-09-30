import * as React from "react";
import { render } from "@react-email/render";

import {
    Body,
    Container,
    Column,
    Head,
    Heading,
    Html,
    Img,
    Link,
    Preview,
    Row,
    Section,
    Text,
} from "@react-email/components";

interface EmailProps {
    url?: string;
    code?: string;
}

/// Email template component, uses react-email to render a HTML email
export const VerifyEmail = ({ url, code }: EmailProps) => (
    <Html>
        <Head />
        <Preview>Confirm your email address</Preview>
        <Body style={main}>
            <Container style={container}>
                <Heading style={h1}>Confirm your email address</Heading>

                {/* Conditionally render the code or the URL */}
                {code ? (
                    <>
                        <Text style={heroText}>
                            Your confirmation code is below - enter it in your
                            open browser window to confirm your email address.
                        </Text>
                        <Section style={codeBox}>
                            <Text style={confirmationCodeText}>{code}</Text>
                        </Section>
                    </>
                ) : (
                    <>
                        <Text style={heroText}>
                            Click the button below to confirm your email
                            address.
                        </Text>
                        <Section style={codeBox}>
                            <Link href={url} style={confirmURL}>
                                Confirm email address
                            </Link>
                        </Section>
                    </>
                )}

                <Text style={text}>
                    If you didn't request this email, there's nothing to worry
                    about - you can safely ignore it.
                </Text>
            </Container>
        </Body>
    </Html>
);

// Styles
const main = {
    backgroundColor: "#ffffff",
    margin: "0 auto",
    fontFamily:
        "-apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif",
};

const container = {
    maxWidth: "600px",
    margin: "0 auto",
};

const logoContainer = {
    marginTop: "32px",
};

const h1 = {
    color: "#1d1c1d",
    fontSize: "36px",
    fontWeight: "700",
    margin: "30px 0",
    padding: "0",
    lineHeight: "42px",
};

const heroText = {
    fontSize: "20px",
    lineHeight: "28px",
    marginBottom: "30px",
};

const codeBox = {
    background: "rgb(245, 244, 245)",
    borderRadius: "4px",
    marginRight: "50px",
    marginBottom: "30px",
    padding: "43px 23px",
};

const confirmationCodeText = {
    fontSize: "30px",
    textAlign: "center" as const,
    verticalAlign: "middle",
};

const confirmURL = {
    color: "#fff",
    background: "#4a154b",
    padding: "10px 20px",
    borderRadius: "4px",
    fontSize: "16px",
    fontWeight: "700",
    textDecoration: "none",
};

const text = {
    color: "#000",
    fontSize: "14px",
    lineHeight: "24px",
};

/**
 * Renders the email template
 *
 * @param {string} props.url - The URL to the verification page
 * @param {string} props.code - The verification code (if no URL is provided)
 * @returns string
 */
export default function renderVerificationEmail({
    url,
    code,
}: EmailProps): string {
    return render(<VerifyEmail url={url} code={code} />);
}
