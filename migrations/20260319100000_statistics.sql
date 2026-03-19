CREATE TABLE IF NOT EXISTS author (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(200) NOT NULL,
    bio TEXT,
    avatar_url VARCHAR(500),
    affiliation VARCHAR(200),

    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS statistic (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    title VARCHAR(500) NOT NULL,
    description TEXT NOT NULL,
    posted_by_user_id BIGINT UNSIGNED NOT NULL,
    upvotes BIGINT UNSIGNED NOT NULL DEFAULT 0,
    downvotes BIGINT UNSIGNED NOT NULL DEFAULT 0,
    question_count BIGINT UNSIGNED NOT NULL DEFAULT 0,
    created_at BIGINT UNSIGNED NOT NULL,

    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS statistic_tag (
    statistic_id BIGINT UNSIGNED NOT NULL,
    tag VARCHAR(100) NOT NULL,

    PRIMARY KEY (statistic_id, tag)
);

CREATE TABLE IF NOT EXISTS statistic_source (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    statistic_id BIGINT UNSIGNED NOT NULL,
    url VARCHAR(1000) NOT NULL,
    title VARCHAR(500),

    PRIMARY KEY (id),
    INDEX (statistic_id)
);

CREATE TABLE IF NOT EXISTS statistic_attachment (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    statistic_id BIGINT UNSIGNED NOT NULL,
    url VARCHAR(1000) NOT NULL,
    filename VARCHAR(300) NOT NULL,

    PRIMARY KEY (id),
    INDEX (statistic_id)
);

CREATE TABLE IF NOT EXISTS statistic_author (
    statistic_id BIGINT UNSIGNED NOT NULL,
    author_id BIGINT UNSIGNED NOT NULL,

    PRIMARY KEY (statistic_id, author_id)
);

CREATE TABLE IF NOT EXISTS statistic_vote (
    statistic_id BIGINT UNSIGNED NOT NULL,
    user_id BIGINT UNSIGNED NOT NULL,
    vote TINYINT NOT NULL,

    PRIMARY KEY (statistic_id, user_id)
);

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
