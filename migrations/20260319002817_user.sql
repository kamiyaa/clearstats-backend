CREATE TABLE IF NOT EXISTS user_credential(
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(326) NOT NULL,
    password_hash VARCHAR(256) NOT NULL,
    salt VARCHAR(32) NOT NULL,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    locked BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE UNIQUE INDEX idx_user_credential_email ON user_credential (email);

CREATE TABLE IF NOT EXISTS user_profile(
    user_id BIGINT NOT NULL,

    username VARCHAR(32) NOT NULL,
    first_name VARCHAR(52) NOT NULL,
    last_name VARCHAR(52) NOT NULL,

    icon_hash VARCHAR(40),

    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,

    PRIMARY KEY (user_id)
);

CREATE UNIQUE INDEX idx_user_profile_username ON user_profile (username);

CREATE TABLE IF NOT EXISTS user_password_reset(
    user_id BIGINT NOT NULL,
    code CHAR(20) NOT NULL,
    expires_at BIGINT NOT NULL,

    PRIMARY KEY (user_id)
);

CREATE TABLE IF NOT EXISTS email_verification_code(
    email VARCHAR(324) UNIQUE NOT NULL,
    verification_code VARCHAR(8) NOT NULL,
    created_at BIGINT NOT NULL,

    PRIMARY KEY (email)
);
