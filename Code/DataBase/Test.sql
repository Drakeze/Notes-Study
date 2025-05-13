-- Basic SQL syntax
SELECT 'Hello World' AS greeting;

-- Create a table
CREATE TABLE greetings (
    id INT PRIMARY KEY,
    message VARCHAR(50)
);

-- Insert data
INSERT INTO greetings VALUES (1, 'Hello World');