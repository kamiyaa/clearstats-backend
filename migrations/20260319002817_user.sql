CREATE TABLE IF NOT EXISTS user_credential(
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    email VARCHAR(326) NOT NULL,
    password_hash VARCHAR(256) NOT NULL,
    salt VARCHAR(32) NOT NULL,
    email_verified TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,
    locked TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,

    PRIMARY KEY (id),
    UNIQUE INDEX (email)
);

CREATE TABLE IF NOT EXISTS user_profile(
    user_id BIGINT UNSIGNED NOT NULL,

    username VARCHAR(32) NOT NULL,
    first_name VARCHAR(52) NOT NULL,
    last_name VARCHAR(52) NOT NULL,

    icon_hash VARCHAR(40),

    created_at BIGINT UNSIGNED NOT NULL,
    updated_at BIGINT UNSIGNED NOT NULL,

    -- metrics
    lab_project_count BIGINT UNSIGNED DEFAULT 0,

    PRIMARY KEY (user_id),
    UNIQUE INDEX (username)
);

CREATE TABLE IF NOT EXISTS user_password_reset(
    user_id BIGINT UNSIGNED NOT NULL,
    code CHAR(20) NOT NULL,
    expires_at BIGINT UNSIGNED NOT NULL,

    PRIMARY KEY (user_id)
);

CREATE TABLE IF NOT EXISTS email_verification_code(
    email VARCHAR(324) UNIQUE NOT NULL,
    verification_code VARCHAR(8) NOT NULL,
    created_at BIGINT NOT NULL,

    PRIMARY KEY (email)
);
