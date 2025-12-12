/* SQL
   cheatsheet */
SELECT * FROM table; -- Show everything
SELECT count(*) FROM employees; -- Get count of something

SELECT field1, field2 FROM table; -- Show one or more columns/fields

SELECT name FROM users WHERE power_level >= 9000; -- WHERE is for specifying/filtering    

CREATE TABLE users (id INTEGER, name TEXT, age INTEGER); -- Create table with the given fields
-- Constraints: PRIMARY KEY, NOT NULL, UNIQUE

INSERT INTO users (id, name, age) VALUES (1, 'John Doe', 21);

ALTER TABLE employees
  RENAME TO contractors; -- Rename table

ALTER TABLE contractors
  RENAME COLUMN salary TO invoice; -- Rename column, can also DROP COLUMN [name], ADD COLUMN [name] [type], 

-- Foreign keys
CREATE TABLE employees (
  id INTEGER PRIMARY KEY,
  department_id INTEGER,

  CONSTRAINT fk_departments -- Constraint is optional, if omitted it is auto-assigned
  FOREIGN KEY (department_id)
  REFERENCES other_table(key_as_appears_in_other_table)
);

UPDATE employees -- Changing fields in specified row(s)
  SET job_title = 'Chef', salary = 15
  WHERE id = 251; -- WHERE id NOT BETWEEN 20 and 100; -- other ways to use WHERE

SELECT employee_id AS id, employee_name AS name -- AS clause is for aliasing a piece of data in out query, only for the duration
  FROM employees;                               -- of the query.

SELECT quantity, -- We can use IIF() (ternary func) and an alias called directive to add a new calculated column to our result set
                 -- Also, we have a comma up there because IIF is the second part of the SELECT clause.
  IIF(quantity < 10, "Order more", "In Stock") AS directive
  FROM products;

SELECT product_name
    FROM products
    WHERE shipment_status IN ('shipped', 'preparing', 'out of stock');

SELECT * FROM products -- Using LIKE, and also % and _ (%: any length, _: single char)
  WHERE product_name LIKE '%banana%';

SELECT name, price, quantity FROM products -- ORDER BY. If we omit the DESC part, it defaults to ASC. LIMIT needs to come after
    ORDER BY quantity DESC                 -- ORDER BY, along with other such limitations that affect the result.
    LIMIT 10;

SELECT user_id, sum(amount) AS balance FROM transactions -- GROUP BY reduces the table to the entries in the group by fields, so 
  GROUP BY user_id                                       -- one entry per user in this case.
  HAVING balance > 5;   -- The HAVING clause is similar to the WHERE clause, but it operates on groups after they've been grouped,
                        -- rather than rows before they've been grouped. 

CREATE TABLE product_suppliers ( -- UNIQUE: we *can* have multiple rows with the same product_id or supplier_id, but we can't have
  product_id INTEGER,            -- two rows where *both* the product_id and supplier_id are the same.
  supplier_id INTEGER,
  UNIQUE(product_id, supplier_id) -- This is also how we implement JOINING TABLES. We link the product and supplier tables this way.
);

SELECT id, song_name, artist_id -- Subqueries. Inner calculation done first, so displating info from artists named Rick.
  FROM songs
  WHERE artist_id IN (          -- The only syntax unique to a subquery is the parentheses surrounding the nested query.
    SELECT id
    FROM artists
    WHERE artist_name LIKE 'Rick%'
  );

SELECT * FROM users    -- JOIN and ON, create rows that contains all the fields from both of the tables with country_code being in 
  INNER JOIN countries -- common.
  ON users.country_code = countries.country_code;
-- EXAMPLES:
        SELECT users.name, users.age, countries.name AS country_name FROM users -- Specifying which fields we need and renaming one
          INNER JOIN countries ON countries.country_code = users.country_code   -- field, and sorting them by the renamed field in ASC
          ORDER BY country_name;                                                -- order.

CREATE INDEX email_idx ON users(email); -- Creating an INDEX of frequently used columns to speed up reads.

/*
  AND, OR, <> `(!=)`, NOT IN (...),
  Aggregates: COUNT(), MAX(), MIN(), AVG(), MOD(num, denom), ROUND(),
  DISINCT, 
  Conditionals: IF, CASE,
  Iterative: LOOP, WHILE,
*/

