import * as React from "react";
import { useState, ReactNode, createContext, useContext } from "react";

type AuthContextType = {
    token: string | null;
    setToken: (token: string) => void;
};

export const AuthContext = createContext<AuthContextType | undefined>(
    undefined
);

export const useAuth = () => {
    const context = useContext(AuthContext);
    if (!context) {
        throw new Error("useAuth must be used within an AuthProvider");
    }
    return context;
};

type AuthProviderProps = {
    children: ReactNode;
};

export const AuthProvider = ({ children }: AuthProviderProps) => {
    const [token, setToken] = useState<string | null>(null);

    // TODO: Add logic to acquire tokens, refresh tokens, etc.

    return (
        <AuthContext.Provider value={{ token, setToken }}>
            {children}
        </AuthContext.Provider>
    );
};
