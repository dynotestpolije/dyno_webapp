-- Add up migration script here

CREATE TABLE IF NOT EXISTS dynos (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    info_id INTEGER DEFAULT 1,
    uuid TEXT NOT NULL,
    data_url TEXT NOT NULL DEFAULT " ",
    data_checksum TEXT NOT NULL,
    verified BOOLEAN DEFAULT 0,
    start DATETIME NOT NULL,
    stop DATETIME NOT NULL,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- CREATE TRIGGER IF NOT EXISTS generate_data_url
-- AFTER INSERT ON dynos
-- BEGIN
    -- UPDATE dynos SET data_url = '/data/dyno/' || CAST(NEW.id AS TEXT) || '-' || CAST(NEW.user_id AS TEXT) || '-' || NEW.uuid || '.bin' WHERE id = NEW.id;
-- END;
-- SELECT name FROM sqlite_master WHERE type = 'trigger' AND name = 'generate_data_url'
