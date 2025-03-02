-- Add down migration script here
ALTER TABLE Nutabi.Location
    DROP COLUMN Source,
    DROP FOREIGN KEY FK_DataSource_SourceId;