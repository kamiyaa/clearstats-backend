INSERT INTO user_credential
    (id, email, email_verified, locked,
        salt, password_hash)
VALUES
    -- passwords are all 'password', encoded with salt: mycredentialsalt
    (1, "alice@clearstats.dev",    true,   false,
        "mycredentialsalt", "$argon2id$v=19$m=19456,t=2,p=1$bXljcmVkZW50aWFsc2FsdA$cv3Vv1J5kBhPMOBZkHCQAw"),
    (2, "bob@clearstats.dev",      true,   false,
        "mycredentialsalt", "$argon2id$v=19$m=19456,t=2,p=1$bXljcmVkZW50aWFsc2FsdA$cv3Vv1J5kBhPMOBZkHCQAw"),
    (3, "charlie@clearstats.dev",  true,   false,
        "mycredentialsalt", "$argon2id$v=19$m=19456,t=2,p=1$bXljcmVkZW50aWFsc2FsdA$cv3Vv1J5kBhPMOBZkHCQAw"),
    (4, "daniel@clearstats.dev",   false,  false,
        "mycredentialsalt", "$argon2id$v=19$m=19456,t=2,p=1$bXljcmVkZW50aWFsc2FsdA$cv3Vv1J5kBhPMOBZkHCQAw"),
    (5, "edward@clearstats.dev",    true,   false,
        "mycredentialsalt", "$argon2id$v=19$m=19456,t=2,p=1$bXljcmVkZW50aWFsc2FsdA$cv3Vv1J5kBhPMOBZkHCQAw"),
    (6, "frank@clearstats.dev",    true,   false,
        "mycredentialsalt", "$argon2id$v=19$m=19456,t=2,p=1$bXljcmVkZW50aWFsc2FsdA$cv3Vv1J5kBhPMOBZkHCQAw"),
    (7, "blackhat@clearstats.dev", true,   true,
        "mycredentialsalt", "$argon2id$v=19$m=19456,t=2,p=1$bXljcmVkZW50aWFsc2FsdA$cv3Vv1J5kBhPMOBZkHCQAw"),
    (8, "jeffbezos88@clearstats.dev",    true,   false,
        "mycredentialsalt", "$argon2id$v=19$m=19456,t=2,p=1$bXljcmVkZW50aWFsc2FsdA$cv3Vv1J5kBhPMOBZkHCQAw")
    ;

INSERT INTO user_profile
    (user_id, username, first_name, last_name, created_at, updated_at)
VALUES
    (1, "alice", "Alice", "Wonderland", 1696079820, 1696079820),
    (2, "bob", "Bob", "Builder", 1696079820, 1696079820),
    (3, "charlie", "Charlie", "Brown", 1696079820, 1696079820),
    (4, "daniel", "Daniel", "Negreanu", 1696079820, 1696079820),
    (5, "edward", "Edward", "Elric", 1696079820, 1696079820),
    (6, "frank", "Frank", "Enstein", 1696079820, 1696079820),
    (7, "blackhat", "Blackhat", "Whitehat", 1696079820, 1696079820),
    (8, "jeffbezos88", "Jeff", "Bezos", 1696079820, 1696079820)
    ;
