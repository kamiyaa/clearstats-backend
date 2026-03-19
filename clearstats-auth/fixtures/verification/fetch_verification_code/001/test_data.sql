START TRANSACTION;

-- ---------------------------

-- Add email verification code
INSERT INTO email_verification_code
    (email, verification_code, created_at)
VALUES
    ("daniel@indaggo.com", "D3JTJ4",  1726712396)
;

COMMIT;