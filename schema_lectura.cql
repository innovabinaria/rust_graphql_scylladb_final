
-- Cambiar al keyspace
USE my_keyspace;

-- Ver todas las filas de la tabla users
SELECT * FROM users;

-- Ver una fila específica (ajusta el id si es necesario)
SELECT * FROM users WHERE id = 1;

-- Limitar el número de filas devueltas
SELECT * FROM users LIMIT 5;
