CREATE TABLE IF NOT EXISTS idioms_tbl (
    _id serial PRIMARY KEY,
    id VARCHAR(100) NOT NULL,
    idiom_eng VARCHAR(100) NOT NULL,
    idiom_hin VARCHAR(100) NOT NULL,
    UNIQUE (id)
);

CREATE TABLE IF NOT EXISTS req_tbl (
    _id serial PRIMARY KEY,
    req_user VARCHAR(100) NOT NULL,
    is_read BOOLEAN NOT NULL,
    idiom VARCHAR(100) REFERENCES idioms_tbl (id)
);
