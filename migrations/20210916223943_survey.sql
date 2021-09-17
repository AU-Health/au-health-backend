-- Add migration script here

-- Create tables to store survey content and answers.

CREATE TABLE survey (
    id uuid NOT NULL PRIMARY KEY,
    user_id uuid UNIQUE NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    FOREIGN KEY(user_id) REFERENCES user_account(id) ON DELETE CASCADE
);

CREATE TABLE response_type (
    id uuid NOT NULL PRIMARY KEY,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    type TEXT UNIQUE NOT NULL
);

CREATE TABLE question_category (
    id uuid NOT NULL PRIMARY KEY,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE question (
    id uuid NOT NULL PRIMARY KEY,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    question TEXT UNIQUE NOT NULL,
    category_id uuid NOT NULL,
    response_type_id uuid NOT NULL,
    FOREIGN KEY (category_id) REFERENCES question_category(id) ON DELETE CASCADE,
    FOREIGN KEY (response_type_id) REFERENCES response_type(id) ON DELETE CASCADE
);

CREATE TABLE answer (
    id uuid NOT NULL PRIMARY KEY,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    answer TEXT NOT NULL,
    survey_id uuid NOT NULL,
    question_id uuid NOT NULL,
    FOREIGN KEY (survey_id) REFERENCES survey(id) ON DELETE CASCADE,
    FOREIGN KEY (question_id) REFERENCES question(id) ON DELETE CASCADE
);

