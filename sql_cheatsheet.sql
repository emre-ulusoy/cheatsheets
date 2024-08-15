/* SQL
   cheatsheet */
SELECT * FROM table; -- Show everything
SELECT count(*) FROM employees; -- Get count of something

SELECT field1, field2 FROM table; -- Show one or more columns/fields
SELECT name FROM users WHERE power_level >= 9000; -- WHERE is for specifying/filtering    

CREATE TABLE users (id INTEGER, name TEXT, age INTEGER); -- Create table with the given fields
-- Constraints: PRIMARY KEY, NOT NULL, UNIQUE

INSERT into users (id, name, age) values (1, 'John Doe', 21);

ALTER TABLE employees
RENAME TO contractors; -- Rename table

ALTER TABLE contractors
RENAME COLUMN salary TO invoice; -- Rename column, can also DROP, ALTER

-- Foreign keys
CREATE TABLE employees (
id INTEGER PRIMARY KEY,
department_id INTEGER,

CONSTRAINT fk_departments -- Constraint is optional, if omitted it is auto-assigned
FOREIGN KEY (department_id)
REFERENCES other_table(other_table_id_or_key);
);

UPDATE employees -- Changing fields in specified row(s)
SET job_title = 'Chef', salary = 15
WHERE id = 251;


