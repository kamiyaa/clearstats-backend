START TRANSACTION;

-- ---------------------------
-- Add email verification code
INSERT INTO email_change_request
    (user_id, pending_email, verification_code, created_at)
VALUES
    (1, "newALICE@indaggo.com", "D3JTJ4",  1726712396)
;

COMMIT;