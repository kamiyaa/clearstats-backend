CREATE TABLE IF NOT EXISTS email_change_request(
    user_id BIGINT NOT NULL,
    pending_email VARCHAR(326) NOT NULL,
    verification_code VARCHAR(8) NOT NULL,
    created_at BIGINT NOT NULL,

    PRIMARY KEY (user_id)
);
