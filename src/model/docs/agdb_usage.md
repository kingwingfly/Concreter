This is for quick search for the usage of [`agdb`](https://github.com/agnesoft/agdb/blob/30cd6161c9313502c29bab8a9c0b4e4c948e2460/docs/queries.md).

Two kinds of queries:
- Immutable: Select & Search
- Mutable: Insert & Remove

# DbUserValue Trait
```rust
pub trait DbUserValue: Sized {
    fn db_id(&self) -> Option<DbId>;
    fn db_keys() -> Vec<DbKey>;
    fn from_db_element(element: &DbElement) -> Result<Self, DbError>;
    fn to_db_values(&self) -> Vec<DbKeyValue>;
}
```

# QueryResult
```rust
pub struct QueryResult {
    pub result: i64,
    pub elements: Vec<DbElement>,
}
```
 `result` is a numerical identifier of a database element. Positive number means the element is a node while negative number means the elements is an edge. The value 0 is a special value signifying no valid element and is used when certain queries return data not related to any particular element, e.g. aliases.

 # DbElement
```rust
pub struct DbElement {
    pub id: DbId,
    pub values: Vec<DbKeyValue>,
}

pub struct DbKeyValue {
    pub key: DbKey,
    pub value: DbValue,
}

pub enum DbValue {
    Bytes(Vec<u8>),
    I64(i64),
    U64(u64),
    F64(DbF64),
    String(String),
    VecI64(Vec<i64>),
    VecU64(Vec<u64>),
    VecF64(Vec<DbF64>),
    VecString(Vec<String>),
}

fn bytes(&self) -> Result<&Vec<u8>, DbError>;
fn to_f64(&self) -> Result<DbF64, DbError>;
fn to_i64(&self) -> Result<i64, DbError>;
fn to_u64(&self) -> Result<u64, DbError>;
fn to_string(&self) -> String;
fn string(&self) -> Result<&String, DbError>;
fn vec_f64(&self) -> Result<&Vec<DbF64>, DbError>;
fn vec_i64(&self) -> Result<&Vec<i64>, DbError>;
fn vec_u64(&self) -> Result<&Vec<u64>, DbError>;
fn vec_string(&self) -> Result<&Vec<String>, DbError>;
```

# run a series of queries as a transaction
```rust
impl Db {
    // immutable transactions
    pub fn transaction<T, E>(&self, mut f: impl FnMut(&Transaction) -> Result<T, E>) -> Result<T, E>

    // mutable transactions
    pub fn transaction_mut<T, E: From<QueryError>>(&mut self, mut f: impl FnMut(&mut TransactionMut) -> Result<T, E>) -> Result<T, E>
}
```

# QueryIds & QueryId

```rust
pub enum QueryIds {
    Ids(Vec<QueryId>),
    Search(SearchQuery),
}

pub enum QueryId {
    Id(DbId),
    Alias(String),
}
```
You can refer to the database elements via their numerical identifier or by the string alias (name). The DbId is then just a wrapper type: pub struct DbId(pub i64). Both QueryIds and QueryId can be constructed from large number of different types like raw i64, &str, String or vectors of those etc.

# QueryValues

The `QueryValues` is a an enum type that makes a distinction between singular and multiple values like so:

```rust
pub enum QueryValues {
    Single(Vec<DbKeyValue>),
    Multi(Vec<Vec<DbKeyValue>>),
}
```
When inserting elements into the database and supplying `QueryValues::Single` all the elements will have the copy of the single set of properties associated with them. Conversely, `QueryValues::Multi` will initialize each element with a different provided set of properties but the number of inserted elements and the number of property sets must then match (it would be a query logic error if they did not match and the query would fail with such an error).

# Mutable Queries

The insert queries are used for both insert and updating data while remove queries are used to delete data from the database.

## Insert Queries

There are 4 kinds:
- insert nodes
- insert edges
- insert aliases
- insert values

### Insert Nodes

```rust
pub struct InsertNodesQuery {
    pub count: u64,
    pub values: QueryValues,
    pub aliases: Vec<String>,
}
```
Builder pattern:
```rust
QueryBuilder::insert().nodes().count(2).query();
QueryBuilder::insert().nodes().count(2).values_uniform(vec![("k", "v").into(), (1, 10).into()]).query();
QueryBuilder::insert().nodes().aliases(vec!["a", "b"]).query();
QueryBuilder::insert().nodes().aliases(vec!["a", "b"]).values(vec![vec![("k", 1).into()], vec![("k", 2).into()]]).query();
QueryBuilder::insert().nodes().aliases(vec!["a", "b"]).values_uniform(vec![("k", "v").into(), (1, 10).into()]).query();
QueryBuilder::insert().nodes().values(vec![vec![("k", 1).into()], vec![("k", 2).into()]]).query();
```
The count is the number of nodes to be inserted into the database. It can be omitted (left 0) if either values or aliases (or both) are provided. If the values is `QueryValues::Single` you must provide either count or aliases. It is a logic error if the count cannot be inferred or is set to 0. If both values QueryValues::Multi and aliases are provided their lengths must match, otherwise it will result in a logic error. Empty alias ("") are not allowed. The values can be inferred from user defined types if they implement DbUserValue trait (#derive(agdb::UserValue)). Both singular nad vectorized versions are supported.

### Insert aliases
```rust
pub struct InsertAliasesQuery {
    pub ids: QueryIds,
    pub aliases: Vec<String>,
}
```
Builder pattern:
```rust
QueryBuilder::insert().aliases("a").ids(1).query();
QueryBuilder::insert().aliases("a").ids("b").query(); // alias "b" is replaced  with "a"
QueryBuilder::insert().aliases(vec!["a", "b"]).ids(vec![1, 2]).query();
```
Inserts or updates aliases of existing nodes (and only nodes, edges cannot have aliases) through this query. It takes ids QueryIds and list of aliases as arguments. The number of aliases must match the ids (even if they are a search query). Empty alias ("") are not allowed.

Note that this query is used also for updating existing aliases. But inserting a different alias of an id that already has one the alias will be overwritten with the new one.

The result will contain:
- number of aliases inserted or updated
- empty list of elements

### Insert values
```rust
pub struct InsertValuesQuery {
    pub ids: QueryIds,
    pub values: QueryValues,
}
```
Builder pattern:
```rust
QueryBuilder::insert().element(&T { ... }).query(); //Where T: DbUserValue (i.e. #derive(UserValue))
QueryBuilder::insert().elements(&vec![T {...}, T {...}]).query(); //Where T: DbUserValue (i.e. #derive(UserValue))
QueryBuilder::insert().values(vec![vec![("k", "v").into(), (1, 10).into()], vec![("k", 2).into()]]).ids(vec![1, 2]).query();
QueryBuilder::insert().values(vec![vec![("k", "v").into(), (1, 10).into()], vec![("k", 2).into()]]).ids(QueryBuilder::search().from("a").query()).query();
QueryBuilder::insert().values_uniform(vec![("k", "v").into(), (1, 10).into()]).ids(vec![1, 2]).query();
QueryBuilder::insert().values_uniform(vec![("k", "v").into(), (1, 10).into()]).ids(QueryBuilder::search().from("a").query()).query();
```

Inserts or updates properties of existing elements. You need to specify the ids and the list of properties. The properties can be either `QueryValues::Single` that will insert the same properties to all elements identified by ids; or `QueryValues::Multi` that will insert each properties to each id of ids, the numbers should match. If the user defined type contains db_id field of type Option<DbId>, you can use the shorthand `insert().element()` / `.insert().elements()` that will infer the values and ids from your objects. All the rules as if specified manually still apply (e.g. the ids must exist in the database). The values() can be inferred from user defined types if they implement DbUserValue trait (#derive(agdb::UserValue)). Both singular nad vectorized versions are supported.

Note that this query is used also for updating existing values. By inserting the same key its old value will be overwritten with the new one.

The result will contain:

- number of key-value pairs (properties) inserted
- empty list of elements

## Remove Queries
There are 3 distinct remove queries:

- remove (elements)
- remove aliases
- remove values

### Remove elements
```rust
pub struct RemoveQuery(pub QueryIds);
```

Builder pattern:
```rust
QueryBuilder::remove().ids(1).query();
QueryBuilder::remove().ids("a").query();
QueryBuilder::remove().ids(vec![1, 2]).query();
QueryBuilder::remove().ids(vec!["a", "b"]).query();
QueryBuilder::remove().ids(QueryBuilder::search().from("a").query()).query();
```
The elements identified by QueryIds will be removed from the database if they exist. It is NOT an error if the elements to be removed do not exist in the database. All associated properties (key-value pairs) are also removed from all elements. Removing nodes will also remove all their edges (incoming and outgoing) and their properties.

The result will contain:

- negative number of elements removed (edges not explicitly listed or those listed but removed as part of one of their node's removal do not contribute to the result counter)
- empty list of elements

### Remove aliases
```rust
pub struct RemoveAliasesQuery(pub Vec<String>);
```
Builder pattern:
```rust
QueryBuilder::remove().aliases("a").query();
QueryBuilder::remove().aliases(vec!["a", "b"]).query();
```

The aliases listed will be removed from the database if they exist. It is NOT an error if the aliases do not exist in the database.

The result will contain:
- negative number of aliases removed
- empty list of elements

### Remove values
```rust
pub struct RemoveValuesQuery(pub SelectValuesQuery);
```

Builder pattern:
```rust
QueryBuilder::remove().values(vec!["k1".into(), "k2".into()]).ids(vec![1, 2]).query();
QueryBuilder::remove().values(vec!["k1".into(), "k2".into()]).ids(QueryBuilder::search().from("a").query()).query();
```
The properties (key-value pairs) identified by keys and associated with ids QueryIds will be removed from the database if they exist. It is a data error if any of the ids do not exist in the database but it is NOT an error if any of the keys does not exist or is not associated as property to any of the ids.

The result will contain:

- Number of actually removed key-value pairs
- empty list of elements

# Immutable queries
Two kind:
- select
- search

## Select queries
There are 6 select queries:
- select (elements)
- select values
- select keys
- select key count
- select aliases
- select all aliases

### Select elements
```rust
pub struct SelectQuery(pub QueryIds);
```
Builder pattern:
```rust
QueryBuilder::select().ids("a").query();
QueryBuilder::select().ids(vec![1, 2]).query();
QueryBuilder::select().ids(QueryBuilder::search().from(1).query()).query();
```

### Select values
```rust
pub struct SelectValuesQuery {
    pub keys: Vec<DbKey>,
    pub ids: QueryIds,
}
```
Builder pattern:
```rust
QueryBuilder::select().values(vec!["k".into(), "k2".into()]).ids("a").query();
QueryBuilder::select().values(vec!["k".into(), "k2".into()]).ids(vec![1, 2]).query();
QueryBuilder::select().values(vec!["k".into(), "k2".into()]).ids(QueryBuilder::search().from(1).query()).query();
```
Selects elements identified by ids or search query, and returns their properties (key-value pairs) identified by keys.

If you plan to convert the result into your user defined type(s) you should use `T::db_keys()` provided through the `DbUserValue` trait as argument to values().

### Select keys
```rust
pub struct SelectKeysQuery(pub QueryIds);
```
Builder pattern:
```rust
QueryBuilder::select().keys().ids("a").query();
QueryBuilder::select().keys().ids(vec![1, 2]).query();
QueryBuilder::select().keys().ids(QueryBuilder::search().from(1).query()).query();
```

### Select key count
```rust
pub struct SelectKeyCountQuery(pub QueryIds);
```
Builder pattern:
```rust
QueryBuilder::select().key_count().ids("a").query();
QueryBuilder::select().key_count().ids(vec![1, 2]).query();
QueryBuilder::select().key_count().ids(QueryBuilder::search().from(1).query()).query();
```

### Select aliases
```rust
pub struct SelectAliasesQuery(pub QueryIds);
```
Builder pattern:
```rust
QueryBuilder::select().aliases().ids(vec![1, 2]).query();
QueryBuilder::select().aliases().ids(QueryBuilder::search().from(1).query()).query();
```
Selects aliases of the ids QueryIds or a search.

### Select all aliases
```rust
pub struct SelectAllAliases {}
```

Builder pattern:
```rust
QueryBuilder::select().aliases().query()
```
Selects all aliases in the database.

## Search queries
Only one search query.
```rust
pub struct SearchQuery {
    pub algorithm: SearchQueryAlgorithm,
    pub origin: QueryId,
    pub destination: QueryId,
    pub limit: u64,
    pub offset: u64,
    pub order_by: Vec<DbKeyOrder>,
    pub conditions: Vec<QueryCondition>,
}

pub enum SearchQueryAlgorithm {
    BreadthFirst,
    DepthFirst,
}

pub enum DbKeyOrder {
    Asc(DbKey),
    Desc(DbKey),
}
```
The default search algorithm is `breadth first` however you can choose to use `depth first`. For path search the `A*` algorithm is used.

Builder pattern:
```rust
QueryBuilder::search().from("a").query();
QueryBuilder::search().to(1).query(); //reverse search
QueryBuilder::search().from("a").to("b").query(); //path search, A*

QueryBuilder::search().breadth_first().from("a").query(); //breadth first is the default and can be omitted however
QueryBuilder::search().depth_first().from("a").query();

//limit, offset and order_by can be applied similarly to all the search variants
QueryBuilder::search().from(1).order_by(vec![DbKeyOrder::Desc("age".into()), DbKeyOrder::Asc("name".into())]).query()
QueryBuilder::search().from(1).offset(10).query();
QueryBuilder::search().from(1).limit(5).query();
QueryBuilder::search().from(1).order_by(vec![DbKeyOrder::Desc("k".into())]).offset(10).query();
QueryBuilder::search().from(1).order_by(vec![DbKeyOrder::Desc("k".into())]).limit(5).query();
QueryBuilder::search().from(1).order_by(vec![DbKeyOrder::Desc("k".into())]).offset(10).limit(5).query();
QueryBuilder::search().from(1).offset(10).limit(5).query();
```
Specifying only `origin` (from) will result in a search along from->to edges. Specifying only `destination` (to) will result in the reverse search along the to<-from edges. When both `origin` and `destination` are specified the search algorithm becomes a path search and the algorithm used will be `A*`. Optionally you can specify a `limit` (0 = unlimited) and `offset` (0 = no offset) to the returned list of graph element ids. If specified (!= 0) the `origin` and the `destination` must exist in the database, otherwise an error will be returned. The elements can be optionally ordered with `order_by` list of keys allowing ascending/descending ordering based on multiple properties.

Finally the list of conditions that each examined graph element must satisfy to be included in the result (and subjected to the limit and offset).

NOTE: When both `origin` and `destination` are specified and the algorithm is switched to the `A*` the limit and offset are applied differently. In regular (open-ended) search (depth or width search), the search will end when the `limit` is reached, but with `A*` the `destination` must be reached first before limit and offset are applied.

### Conditions

The currently supported conditions are:

- Where
- Edge (if the element is an `edge`)
- Node (if the element is a `node`)
- Distance (if the current distance of the search satisfies the numerical comparison, each graph element away from the start increases the distance, including edges, i.e. second node from start is at distance `2`)
- EdgeCount (if the element is a node and total number of edges (in and out) satisfies the numerical comparison - self-referential edges are counted twice)
- EdgeCountFrom (if the element is a node and total number of outgoing edges satisfies the numerical comparison)
- EdgeCountTo (if the element is a node and total number of incoming edges satisfies the numerical comparison)
- Ids (if the element id is in the list)
- KeyValue (if the element's property has the `key` and its value satisfies `value` comparison)
- Keys (if the element has all the `keys` regardless of their values)
- EndWhere (closes nested list of conditions)

All conditions can be further modified as follows:

- Beyond (continues the search only beyond this element)
- Not (reverses the condition result)
- NotBeyond (stops the search beyond this element)

The conditions can be changed with logic operators:

- And (logical `and`)
- Or (logical `or`)

```Rust
pub struct QueryCondition {
    pub logic: QueryConditionLogic,
    pub modifier: QueryConditionModifier,
    pub data: QueryConditionData,
}

pub enum QueryConditionLogic {
    And,
    Or,
}

pub enum QueryConditionModifier {
    None,
    Beyond,
    Not,
    NotBeyond,
}

pub enum QueryConditionData {
    Distance(CountComparison),
    Edge,
    EdgeCount(CountComparison),
    EdgeCountFrom(CountComparison),
    EdgeCountTo(CountComparison),
    Ids(Vec<QueryId>),
    KeyValue { key: DbKey, value: Comparison },
    Keys(Vec<DbKey>),
    Node,
    Where(Vec<QueryCondition>),
}

pub enum CountComparison {
    Equal(u64),
    GreaterThan(u64),
    GreaterThanOrEqual(u64),
    LessThan(u64),
    LessThanOrEqual(u64),
    NotEqual(u64),
}

pub enum Comparison {
    Equal(DbValue),
    GreaterThan(DbValue),
    GreaterThanOrEqual(DbValue),
    LessThan(DbValue),
    LessThanOrEqual(DbValue),
    NotEqual(DbValue),
    Contains(DbValue),
}
```

Builder pattern:

```Rust
//the where_() can be applied to any of the basic search queries after order_by/offset/limit
//not() and not_beyond() can be applied to all conditions including nested where_()
QueryBuilder::search().from(1).where_().distance(CountComparison::LessThan(3)).query();
QueryBuilder::search().from(1).where_().edge().query();
QueryBuilder::search().from(1).where_().edge_count(CountComparison::GreaterThan(2))().query();
QueryBuilder::search().from(1).where_().edge_count_from(CountComparison::Equal(1))().query();
QueryBuilder::search().from(1).where_().edge_count_to(CountComparison::NotEqual(1))().query();
QueryBuilder::search().from(1).where_().node().query();
QueryBuilder::search().from(1).where_().key("k").value(Comparison::Equal(1.into())).query();
QueryBuilder::search().from(1).where_().keys(vec!["k1".into(), "k2".into()]).query();
QueryBuilder::search().from(1).where_().not().keys(vec!["k1".into(), "k2".into()]).query();
QueryBuilder::search().from(1).where_().ids(vec![1, 2]).query();
QueryBuilder::search().from(1).where_().beyond().keys(vec!["k"]).query();
QueryBuilder::search().from(1).where_().not().ids(vec![1, 2]).query();
QueryBuilder::search().from(1).where_().not_beyond().ids("a").query();
QueryBuilder::search().from(1).where_().node().or().edge().query();
QueryBuilder::search().from(1).where_().node().and().distance().query(CountComparison::GreaterThanOrEqual(3)).query();
QueryBuilder::search().from(1).where_().node().or().where_().edge().and().key("k").value(Comparison::Equal(1.into())).end_where().query();
QueryBuilder::search().from(1).where_().node().or().where_().edge().and().key("k").value(Comparison::Contains(1.into())).end_where().query();
QueryBuilder::search().from(1).where_().node().or().where_().edge().and().key("k").value(Comparison::Contains(vec![1, 2].into())).end_where().query();
```

NOTE: The use of `where_` with an underscore as the method name is necessary to avoid conflict with the Rust keyword.

The conditions are applied one at a time to each visited element and chained using logic operators `AND` and `OR`. They can be nested using `where_` and `end_where` (in place of brackets). The condition evaluator supports short-circuiting not evaluating conditions further if the logical outcome cannot change. The condition comparators are **type strict** meaning that they do not perform type conversions nor coercion (e.g. `Comparison::Equal(1_i64).compare(1_u64)` will evaluate to `false`). Slight exception to this rule is the `Comparison::Contains` as it allows vectorized version of the base type (e.g. `Comparison::Contains(vec!["bc", "ef"]).compare("abcdefg")` will evaluate to `true`).

The condition `Distance` and the condition modifiers `Beyond` and `NotBeyond` are particularly important because they can directly influence the search. The former (`Distance`) can limit the depth of the search and can help with constructing more elaborate queries (or sequence thereof) extracting only fine grained elements (e.g. nodes whose edges have particular properties or are connected to other nodes with some properties). The latter (`Beyond` and `NotBeyond`) can limit search to only certain areas of an otherwise larger graph. Its most basic usage would be with condition `ids` to flat out stop the search at certain elements or continue only beyond certain elements.

### Truth tables

The following information should help with reasoning about the query conditions. Most of it should be intuitive but there are some aspects that might not be obvious especially when combining logic operators and condition modifiers. The search is using the following `enum` when evaluating conditions:

```Rust
pub enum SearchControl {
    Continue(bool),
    Finish(bool),
    Stop(bool),
}
```

The type controls the search and the boolean value controls if the given element should be included in a search result. The `Stop` will prevent the search expanding beyond current element (stopping the search in that direction). `Finish` will immediately exit the search returning accumulated elements (ids) and is only used internally with `offset` and `limit` (NOTE: path search and `order_by` still require complete search regardless of `limit`).

Each condition contributes to the final control result as follows with the starting/default value being always `Continue(true)`:

#### And

| Left           | Right           | Result                  |
| -------------- | --------------- | ----------------------- |
| Continue(left) | Continue(right) | Continue(left && right) |
| Continue(left) | Stop(right)     | Stop(left && right)     |
| Continue(left) | Finish(right)   | Finish(left && right)   |
| Stop(left)     | Stop(right)     | Stop(left && right)     |
| Stop(left)     | Finish(right)   | Finish(left && right)   |
| Finish(left)   | Finish(right)   | Finish(left && right)   |

#### Or

| Left           | Right           | Result                    |
| -------------- | --------------- | ------------------------- |
| Continue(left) | Continue(right) | Continue(left \|\| right) |
| Continue(left) | Stop(right)     | Continue(left \|\| right) |
| Continue(left) | Finish(right)   | Continue(left \|\| right) |
| Stop(left)     | Stop(right)     | Stop(left \|\| right)     |
| Stop(left)     | Finish(right)   | Stop(left \|\| right)     |
| Finish(left)   | Finish(right)   | Finish(left \|\| right)   |

#### Modifiers

Modifiers will change the result of a condition based on the control value (the boolean) as follows:

| Modifier  | TRUE                | FALSE                  |
| --------- | ------------------- | ---------------------- |
| None      | -                   | -                      |
| Beyond    | `&& Continue(true)` | `\|\| Stop(false)`     |
| Not       | `!`                 | `!`                    |
| NotBeyond | `&& Stop(true)`     | `\|\| Continue(false)` |

#### Results

Most conditions result in `Continue(bool)` except for `distance()` and nested `where()` which can also result in `Stop(bool)`:

| Condition   | Continue | Stop |
| ----------- | -------- | ---- |
| Where       | YES      | YES  |
| Edge        | YES      | NO   |
| Node        | YES      | NO   |
| Distance    | YES      | YES  |
| EdgeCount\* | YES      | NO   |
| Ids         | YES      | NO   |
| Key(Value)  | YES      | NO   |
| Keys        | YES      | NO   |

### Paths

Path search (`from().to()`) uses A\* algorithm. Every element (node or edge) has a cost of `1` by default. If it passes all the conditions (the `SearchControl` value `true`) the cost will remain `1` and would be included in the result (if the path it is on would be selected). If it fails any of the conditions (the `SearchControl` value `false`) its cost will be `2`. This means that the algorithm will prefer paths where elements match the conditions rather than the absolutely shortest path (that can be achieved with no conditions). If the search is not to continue beyond certain element (through `beyond()`, `not_beyond()` or `distance()` conditions), its cost will be `0` and the paths it is on will no longer be considered for that search.
