-- Create applications table
CREATE TABLE IF NOT EXISTS applications (
    application_id UUID PRIMARY KEY,
    basic_auth_settings_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
-- Create application auth settings table
CREATE TABLE IF NOT EXISTS basic_auth_settings (
    application_id UUID PRIMARY KEY,
    min_password_length INTEGER NOT NULL DEFAULT 8,
    max_password_length INTEGER NOT NULL DEFAULT 64,
    require_lowercase BOOLEAN NOT NULL DEFAULT FALSE,
    require_uppercase BOOLEAN NOT NULL DEFAULT FALSE,
    require_numeric BOOLEAN NOT NULL DEFAULT FALSE,
    require_special BOOLEAN NOT NULL DEFAULT FALSE,
    password_history_count INTEGER NOT NULL DEFAULT 0,
    password_expiry_days INTEGER NOT NULL DEFAULT 0,
    max_failed_attempts INTEGER NOT NULL DEFAULT 0,
    lockout_duration INTEGER NOT NULL DEFAULT 0,
    require_mfa BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    FOREIGN KEY (application_id) REFERENCES applications(application_id) ON DELETE CASCADE
);
-- Foreign key applications table -> basic auth settings
ALTER TABLE applications
ADD FOREIGN KEY (basic_auth_settings_id) REFERENCES basic_auth_settings(application_id) ON DELETE CASCADE;
-- Create users table
-- TODO: Add foreign key to primary_email_id
CREATE TABLE IF NOT EXISTS users (
    user_id UUID PRIMARY KEY,
    application_id UUID NOT NULL,
    external_id VARCHAR(255) NOT NULL,
    primary_email_id UUID,
    full_name VARCHAR(255),
    display_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    FOREIGN KEY (application_id) REFERENCES applications(application_id) ON DELETE CASCADE
);
-- Create users index
CREATE INDEX IF NOT EXISTS users_application_id_idx ON users (application_id, user_id);
CREATE INDEX IF NOT EXISTS users_external_id_idx ON users (application_id, external_id);
-- Create email addresses table
CREATE TABLE IF NOT EXISTS email_addresses (
    email_id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    application_id UUID NOT NULL,
    email VARCHAR(255) NOT NULL,
    is_primary BOOLEAN NOT NULL DEFAULT FALSE,
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    verified_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (application_id) REFERENCES applications(application_id) ON DELETE CASCADE
);
-- Connect foreign key index users.primary_email_id to email_addresses.email_id
ALTER TABLE users
ADD FOREIGN KEY (primary_email_id) REFERENCES email_addresses(email_id) ON DELETE CASCADE;
-- Create basic_auths table (used for password authentication)
CREATE TABLE IF NOT EXISTS basic_auths (
    user_id UUID PRIMARY KEY,
    application_id UUID NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (application_id) REFERENCES applications(application_id) ON DELETE CASCADE
);