SET @user_id1   = 8;
SET @code       = 'RBRDOE';
SET @user_id2   = 1;
SET @code2      = 'ABC123';
SET @initial_id = 1000;

-- ---------------------------

-- Add reset code
INSERT INTO user_password_reset
    (user_id, code, expires_at)
VALUES
    (@user_id1, @code,  9728068553),
    (@user_id2, @code2,  9728068553)
;