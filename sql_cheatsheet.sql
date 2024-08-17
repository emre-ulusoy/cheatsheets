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
  WHERE id = 251; -- WHERE id NOT BETWEEN 20 and 100; -- other ways to use WHERE

SELECT employee_id AS id, employee_name AS name -- AS clause is for aliasing a piece of data in out query, only for the duration
  FROM employees;                               -- of the query.

SELECT quantity, -- We can use IIF() (ternary func) and an alias called directive to add a new calculated column to our result set
                 -- Also, we have a comma up there because IIF is the second part of the SELECT clause.
  IIF(quantity < 10, "Order more", "In Stock") AS directive
  FROM products

SELECT product_name
    FROM products
    WHERE shipment_status IN ('shipped', 'preparing', 'out of stock');


/*
  AND, OR, DISTINCT, 
*/
