"use client";

import Link from "next/link";
import React, { useState } from "react";
import { LockClosedIcon } from "@heroicons/react/24/solid";
import "./login.css";
import Image from "next/image";
import { Spinner } from "../components/Spinner/spinner";

const LoginPage = () => {
    const [firstName, setFirstName] = useState("");
    const [lastName, setLastName] = useState("");
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");

    const [loadingSend, setLoadingSend] = useState(false);

    const handleLogin = (e) => {
        e.preventDefault();
        setLoadingSend(true);

        setTimeout(() => {
            setLoadingSend(false);
        }, 2000);
    };

    return (
        <div className="flex h-screen w-screen items-center justify-center p-6 bg-slate-100">
            <div className="flex flex-row items-stretch justify-center bg-white rounded-md p-8 shadow">
                <div className="bg-slate-100 rounded-md p-8 flex justify-between flex-col">
                    <div className="w-7 h-7 bg-blue-600 rounded-md flex justify-center items-center">
                        <LockClosedIcon className="w-4 h-4 text-neutral-50"></LockClosedIcon>
                    </div>
                    <div className="max-w-sm">
                        <h2 className="text-2xl font-bold text-gray-900 leading-9">
                            Let us help you secure your application.
                        </h2>
                        <p className="text-gray-500 font-light text-sm mt-4">
                            Secure your application fast and easy with our
                            complete authentication solution.
                        </p>
                    </div>
                    <div className="p-6 bg-cyan-950 w-full max-w-sm rounded-xl">
                        <p className="text-slate-100 text-sm font-light font-sans">
                            Auth was a game-changer for us. Seamless, secure
                            authentication that our users trust and love.
                            It&apos;s the new gold standard for digital identity
                            solutions.
                        </p>
                        <div className="flex items-center justify-between mt-4">
                            <div className="flex items-center">
                                <Image
                                    className="w-10 h-10 rounded-full"
                                    src="/pfp.png"
                                    alt="Profile"
                                    width="100"
                                    height="100"
                                />
                                <div className="ml-2">
                                    <p className="text-slate-100 font-medium text-sm">
                                        Elsa Norwood
                                    </p>
                                    <p className="text-slate-100 font-light text-sm">
                                        CEO at Proghour
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div className="w-full lg:max-w-xl pl-8 pt-8">
                    <div>
                        <h1 className="font-semibold text-xl">Get started</h1>
                        <p className="text-gray-500 font-light text-sm mt-2">
                            Create your account to get started
                        </p>
                    </div>
                    <div className="mt-8">
                        <form onSubmit={handleLogin}>
                            <div className="mb-4 flex">
                                <div className="w-1/2 mr-2">
                                    <label
                                        className="block text-gray-600 text-sm font-medium mb-2"
                                        htmlFor="firstName"
                                    >
                                        First name
                                    </label>
                                    <input
                                        className="appearance-none block w-full px-4 py-2 mt-2 text-gray-700 bg-white border rounded-md focus:border-gray-400 focus:ring-gray-300 focus:outline-none focus:ring focus:ring-opacity-40 disabled:opacity-50 disabled:bg-gray-200 disabled:cursor-not-allowed transition-colors duration-300"
                                        id="firstName"
                                        type="text"
                                        value={firstName}
                                        onChange={(e) =>
                                            setFirstName(e.target.value)
                                        }
                                        disabled={loadingSend}
                                    />
                                </div>
                                <div className="w-1/2 ml-2">
                                    <label
                                        className="block text-gray-600 text-sm font-medium mb-2"
                                        htmlFor="lastName"
                                    >
                                        Last name
                                    </label>
                                    <input
                                        className="appearance-none block w-full px-4 py-2 mt-2 text-gray-700 bg-white border rounded-md focus:border-gray-400 focus:ring-gray-300 focus:outline-none focus:ring focus:ring-opacity-40 disabled:opacity-50 disabled:bg-gray-200 disabled:cursor-not-allowed transition-colors duration-300"
                                        id="lastName"
                                        type="text"
                                        value={lastName}
                                        onChange={(e) =>
                                            setLastName(e.target.value)
                                        }
                                        disabled={loadingSend}
                                    />
                                </div>
                            </div>
                            <div className="mb-4">
                                <label
                                    htmlFor="email"
                                    className="block text-gray-600 text-sm font-medium mb-2"
                                >
                                    Email
                                </label>
                                <input
                                    type="email"
                                    className="block w-full px-4 py-2 mt-2 text-gray-700 bg-white border rounded-md focus:border-gray-400 focus:ring-gray-300 focus:outline-none focus:ring focus:ring-opacity-40 disabled:opacity-50 disabled:bg-gray-200 disabled:cursor-not-allowed transition-colors duration-300"
                                    disabled={loadingSend}
                                />
                            </div>
                            <div className="mb-2">
                                <label
                                    htmlFor="password"
                                    className="block text-gray-600 text-sm font-medium mb-2"
                                >
                                    Password
                                </label>
                                <input
                                    type="password"
                                    className="block w-full px-4 py-2 mt-2 text-gray-700 bg-white border rounded-md focus:border-gray-400 focus:ring-gray-300 focus:outline-none focus:ring focus:ring-opacity-40 disabled:opacity-50 disabled:bg-gray-200 disabled:cursor-not-allowed transition-colors duration-300"
                                    disabled={loadingSend}
                                />
                            </div>
                            <div className="mt-6">
                                <button
                                    className="w-full px-4 py-4 tracking-wide text-white transition-colors duration-300 transform bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:bg-blue-700 flex items-center justify-center"
                                    onClick={handleLogin}
                                    disabled={loadingSend}
                                >
                                    {loadingSend ? (
                                        <Spinner
                                            size="medium"
                                            className="stroke-blue-100"
                                        ></Spinner>
                                    ) : (
                                        <span>Sign Up</span>
                                    )}
                                </button>
                            </div>
                        </form>

                        <div className="relative flex items-center justify-center w-full mt-6 border border-t">
                            <div className="absolute px-5 bg-white">Or</div>
                        </div>
                        <div className="flex mt-6 gap-x-2">
                            <button
                                type="button"
                                className="flex items-center justify-center w-full p-2 border border-gray-600 rounded-md focus:ring-2 focus:ring-offset-1 focus:ring-violet-600"
                            >
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 32 32"
                                    className="w-5 h-5 fill-current"
                                >
                                    <path d="M16.318 13.714v5.484h9.078c-0.37 2.354-2.745 6.901-9.078 6.901-5.458 0-9.917-4.521-9.917-10.099s4.458-10.099 9.917-10.099c3.109 0 5.193 1.318 6.38 2.464l4.339-4.182c-2.786-2.599-6.396-4.182-10.719-4.182-8.844 0-16 7.151-16 16s7.156 16 16 16c9.234 0 15.365-6.49 15.365-15.635 0-1.052-0.115-1.854-0.255-2.651z"></path>
                                </svg>
                            </button>
                            <button className="flex items-center justify-center w-full p-2 border border-gray-600 rounded-md focus:ring-2 focus:ring-offset-1 focus:ring-violet-600">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 32 32"
                                    className="w-5 h-5 fill-current"
                                >
                                    <path d="M16 0.396c-8.839 0-16 7.167-16 16 0 7.073 4.584 13.068 10.937 15.183 0.803 0.151 1.093-0.344 1.093-0.772 0-0.38-0.009-1.385-0.015-2.719-4.453 0.964-5.391-2.151-5.391-2.151-0.729-1.844-1.781-2.339-1.781-2.339-1.448-0.989 0.115-0.968 0.115-0.968 1.604 0.109 2.448 1.645 2.448 1.645 1.427 2.448 3.744 1.74 4.661 1.328 0.14-1.031 0.557-1.74 1.011-2.135-3.552-0.401-7.287-1.776-7.287-7.907 0-1.751 0.62-3.177 1.645-4.297-0.177-0.401-0.719-2.031 0.141-4.235 0 0 1.339-0.427 4.4 1.641 1.281-0.355 2.641-0.532 4-0.541 1.36 0.009 2.719 0.187 4 0.541 3.043-2.068 4.381-1.641 4.381-1.641 0.859 2.204 0.317 3.833 0.161 4.235 1.015 1.12 1.635 2.547 1.635 4.297 0 6.145-3.74 7.5-7.296 7.891 0.556 0.479 1.077 1.464 1.077 2.959 0 2.14-0.020 3.864-0.020 4.385 0 0.416 0.28 0.916 1.104 0.755 6.4-2.093 10.979-8.093 10.979-15.156 0-8.833-7.161-16-16-16z"></path>
                                </svg>
                            </button>
                            <button className="flex items-center justify-center w-full p-2 border border-gray-600 rounded-md focus:ring-2 focus:ring-offset-1 focus:ring-violet-600">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 32 32"
                                    className="w-5 h-5 fill-current"
                                >
                                    <path d="M31.937 6.093c-1.177 0.516-2.437 0.871-3.765 1.032 1.355-0.813 2.391-2.099 2.885-3.631-1.271 0.74-2.677 1.276-4.172 1.579-1.192-1.276-2.896-2.079-4.787-2.079-3.625 0-6.563 2.937-6.563 6.557 0 0.521 0.063 1.021 0.172 1.495-5.453-0.255-10.287-2.875-13.52-6.833-0.568 0.964-0.891 2.084-0.891 3.303 0 2.281 1.161 4.281 2.916 5.457-1.073-0.031-2.083-0.328-2.968-0.817v0.079c0 3.181 2.26 5.833 5.26 6.437-0.547 0.145-1.131 0.229-1.724 0.229-0.421 0-0.823-0.041-1.224-0.115 0.844 2.604 3.26 4.5 6.14 4.557-2.239 1.755-5.077 2.801-8.135 2.801-0.521 0-1.041-0.025-1.563-0.088 2.917 1.86 6.36 2.948 10.079 2.948 12.067 0 18.661-9.995 18.661-18.651 0-0.276 0-0.557-0.021-0.839 1.287-0.917 2.401-2.079 3.281-3.396z"></path>
                                </svg>
                            </button>
                        </div>
                    </div>
                    <div>
                        <p className="mt-4 text-sm text-center text-gray-700 pb-6 pt-4">
                            Already have an account?{" "}
                            <Link
                                href="/signin"
                                className="font-medium text-blue-600 hover:underline"
                            >
                                Sign in
                            </Link>
                        </p>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default LoginPage;
