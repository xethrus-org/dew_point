CREATE TABLE EnclosureDetails (
    species_id INTEGER PRIMARY KEY,  -- Unique identifier for the species
    species_name TEXT NOT NULL,      -- Name of the species
    enclosure_size REAL NOT NULL,    -- Size of the enclosure (could be in square meters, square feet, etc.)
    humidity REAL NOT NULL,          -- Humidity level in percentage
    temperature REAL NOT NULL        -- Temperature in degrees Celsius or Fahrenheit
);


