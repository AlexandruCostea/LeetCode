-- Write your PostgreSQL query statement below
SELECT W1.id AS id
FROM Weather W1 INNER JOIN Weather W2
ON W1.id = W2.id + 1
WHERE W1.temperature > W2.temperature;