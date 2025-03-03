-- Add down migration script here
ALTER TABLE Nutabi.DataSource
    DROP COLUMN AccessKey;