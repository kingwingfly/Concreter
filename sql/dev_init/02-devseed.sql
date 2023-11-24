INSERT INTO users (username) VALUES ('demo1');

INSERT INTO articles (author, title, content, field) VALUES (1000, 'hello world', 'hello world', 'science');

INSERT INTO comments (author, content, article) VALUES (1000, 'hello world', 1000);

INSERT INTO entities (name, attris) VALUES ('Genshin Impact', '{"alias": "原神", "type": "game"}');

INSERT INTO formulas (md, sym) VALUES ('y = x ^ 2', 'def formula() -> sympy.Expr:\n    x = sympy.Symbol("x")\n    return x ** 2');
