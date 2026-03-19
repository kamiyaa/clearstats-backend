CREATE TABLE IF NOT EXISTS question (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    statistic_id BIGINT UNSIGNED NOT NULL,
    body TEXT NOT NULL,
    posted_by_user_id BIGINT UNSIGNED NOT NULL,
    upvotes BIGINT UNSIGNED NOT NULL DEFAULT 0,
    downvotes BIGINT UNSIGNED NOT NULL DEFAULT 0,
    created_at BIGINT UNSIGNED NOT NULL,

    PRIMARY KEY (id),
    INDEX (statistic_id)
);

CREATE TABLE IF NOT EXISTS question_vote (
    question_id BIGINT UNSIGNED NOT NULL,
    user_id BIGINT UNSIGNED NOT NULL,
    vote TINYINT NOT NULL,

    PRIMARY KEY (question_id, user_id)
);
