To analyze and store the information about an article, a few steps are required:
- Extract the text, picture and formula in the article.
- Do NER on the text:
    - We get a json containing the entities and some unknown attributes.
    - Directly store the entities json in postgresql.
    - For each entity, store as `entity` node in agdb. Establish `Has` with the article.
    - The node's attributes(KeyValues) is the attributes of the entity.
    - For example: `{"Genshin Impact": { "alias": "原神", "type": "game" }}`, we store node "Genshin Impact" as `entity` node, whose attribute `alias` is "原神", and attribute `type` is "game".
- Process the formula:
    - We get a json containing the formula's `latex` and its python `sympy` representation.
    - Directly store the formula json in postgresql.
    - For each formula, store as `formula` node in agdb. Establish `Has` with the article.
    - The node's attributes(KeyValues) is the attributes of the formula.
    - For example: `{"E=mc^2": { "sympy": "def formula() -> sym.Expr: ..."}}`, we store node "E=mc^2" as `formula` node, whose attribute `sympy` is "def formula() -> sym.Expr: ...".


However, I don't have enough time to insert the attris in Agdb, so I just store all the information of the entities and formulas in postgresql, the nodes in agdb only contain the id of postgres.
