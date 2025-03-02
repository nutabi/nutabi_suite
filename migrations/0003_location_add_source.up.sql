-- Add up migration script here
ALTER TABLE Nutabi.Location
    ADD COLUMN Source INTEGER NOT NULL DEFAULT 1 AFTER RecordId,
    ADD CONSTRAINT FK_DataSource_SourceId FOREIGN KEY (Source)
        REFERENCES Nutabi.DataSource (SourceId)
        ON UPDATE CASCADE
        ON DELETE RESTRICT;