CREATE TABLE IF NOT EXISTS clipboard_history (
    id INTEGER PRIMARY KEY NOT NULL,
    typ INTEGER NOT NULL,
    text TEXT,
    width INTEGER,
    height INTEGER,
    bytes BLOB
);

CREATE TABLE IF NOT EXISTS application_meta (
    version INTEGER NOT NULL,
    preference TEXT NOT NULL
);
