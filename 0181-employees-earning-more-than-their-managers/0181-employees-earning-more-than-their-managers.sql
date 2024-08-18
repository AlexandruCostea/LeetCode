-- Write your PostgreSQL query statement below
SELECT name as Employee FROM Employee E
WHERE E.salary > 
(SELECT salary FROM Employee WHERE id = E.managerId);