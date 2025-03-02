-- Add up migration script here
CREATE TABLE IF NOT EXISTS Nutabi.DataSource (
    SourceId INTEGER NOT NULL PRIMARY KEY AUTO_INCREMENT,
    CreatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    LastUpdatedAt TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    Name VARCHAR(50) NOT NULL UNIQUE,
    Description TEXT
);

-- Add default data source
INSERT INTO Nutabi.DataSource (Name, Description) VALUES ('nutabi', 'Default data source');