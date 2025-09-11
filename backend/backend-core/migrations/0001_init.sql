-- Enable foreign keys for SQLite
PRAGMA foreign_keys = ON;

-- Projects table
CREATE TABLE IF NOT EXISTS projects (
    id   BLOB(16) PRIMARY KEY,  -- UUID stored as 16-byte blob
    name TEXT NOT NULL
);

-- Resources table
CREATE TABLE IF NOT EXISTS resources (
    id   BLOB(16) PRIMARY KEY,  -- UUID stored as 16-byte blob
    name TEXT NOT NULL
);

-- Assignments table
CREATE TABLE IF NOT EXISTS assignments (
    resource_id BLOB(16) NOT NULL,
    project_id  BLOB(16) NOT NULL,
    year        INTEGER NOT NULL,
    month       INTEGER NOT NULL CHECK (month BETWEEN 1 AND 12),
    PRIMARY KEY (resource_id, project_id, year, month),
    FOREIGN KEY (resource_id) REFERENCES resources(id) ON DELETE CASCADE,
    FOREIGN KEY (project_id)  REFERENCES projects(id)  ON DELETE CASCADE
);
