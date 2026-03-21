CREATE TABLE IF NOT EXISTS question (
    id BIGSERIAL PRIMARY KEY,
    statistic_id BIGINT NOT NULL,
    body TEXT NOT NULL,
    posted_by_user_id BIGINT NOT NULL,
    upvotes BIGINT NOT NULL DEFAULT 0,
    downvotes BIGINT NOT NULL DEFAULT 0,
    created_at BIGINT NOT NULL
);
CREATE UNIQUE INDEX idx_question_statistic_id ON statistic_source (statistic_id);

CREATE TABLE IF NOT EXISTS question_vote (
    question_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    vote SMALLINT NOT NULL,

    PRIMARY KEY (question_id, user_id)
);
