-- Add up migration script here
ALTER TABLE Nutabi.DataSource
    ADD COLUMN AccessKey VARCHAR(44) NOT NULL UNIQUE;