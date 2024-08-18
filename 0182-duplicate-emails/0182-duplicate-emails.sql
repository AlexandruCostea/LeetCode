-- Write your PostgreSQL query statement below
SELECT email as Email
FROM ( 
    SELECT email as Email, Count(id) as count
    FROM Person
    GROUP BY email
)
WHERE count > 1;