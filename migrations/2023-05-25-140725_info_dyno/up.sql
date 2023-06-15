-- Add up migration script here


CREATE TABLE IF NOT EXISTS dyno_info (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    motor_type SMALLINT NOT NULL default 1,
    name TEXT,
    cc SMALLINT,
    cylinder SMALLINT,
    stroke SMALLINT,
    diameter_roller REAL,
    diameter_roller_beban REAL,
    diameter_gear_encoder REAL,
    diameter_gear_beban REAL,
    jarak_gear REAL,
    berat_beban REAL,
    gaya_beban REAL,
    keliling_roller REAL,

    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
