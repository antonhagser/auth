import type { V2_MetaFunction } from "@remix-run/node";
import { Link } from "@remix-run/react";

export const meta: V2_MetaFunction = () => {
    return [
        { title: "New Remix App" },
        { name: "description", content: "Welcome to Remix!" },
    ];
};

export default function Index() {
    return (
        <div className="flex justify-center mt-8">
            <Link className="text-cyan-500 ml-8 hover:underline" to="/account/login">Login</Link>
            <Link className="text-cyan-500 ml-8 hover:underline" to="/account/register">Register</Link>
        </div>
    );
}
