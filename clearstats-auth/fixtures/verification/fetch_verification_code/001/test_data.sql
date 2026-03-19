START TRANSACTION;

-- ---------------------------

-- Add email verification code
INSERT INTO email_verification_code
    (email, verification_code, created_at)
VALUES
    ("daniel@clearstats.dev", "D3JTJ4",  1726712396)
;

COMMIT;