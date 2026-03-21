CREATE TABLE IF NOT EXISTS author (
    id SERIAL PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    bio TEXT,
    avatar_url VARCHAR(500),
    affiliation VARCHAR(200)
);

CREATE TABLE IF NOT EXISTS statistic (
    id SERIAL PRIMARY KEY,
    title VARCHAR(500) NOT NULL,
    description TEXT NOT NULL,
    posted_by_user_id BIGINT NOT NULL,
    upvotes BIGINT NOT NULL DEFAULT 0,
    downvotes BIGINT NOT NULL DEFAULT 0,
    question_count BIGINT NOT NULL DEFAULT 0,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS statistic_tag (
    statistic_id BIGINT NOT NULL,
    tag VARCHAR(100) NOT NULL,

    PRIMARY KEY (statistic_id, tag)
);

CREATE TABLE IF NOT EXISTS statistic_source (
    id SERIAL PRIMARY KEY,
    statistic_id BIGINT NOT NULL,
    url VARCHAR(1000) NOT NULL,
    title VARCHAR(500)
);

CREATE UNIQUE INDEX idx_statistic_source_statistic_id ON statistic_source (statistic_id);

CREATE TABLE IF NOT EXISTS statistic_attachment (
    id SERIAL PRIMARY KEY,
    statistic_id BIGINT NOT NULL,
    url VARCHAR(1000) NOT NULL,
    filename VARCHAR(300) NOT NULL
);

CREATE UNIQUE INDEX idx_statistic_attachment_statistic_id ON statistic_source (statistic_id);

CREATE TABLE IF NOT EXISTS statistic_author (
    statistic_id BIGINT NOT NULL,
    author_id BIGINT NOT NULL,

    PRIMARY KEY (statistic_id, author_id)
);

CREATE TABLE IF NOT EXISTS statistic_vote (
    statistic_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    vote SMALLINT NOT NULL,

    PRIMARY KEY (statistic_id, user_id)
);
