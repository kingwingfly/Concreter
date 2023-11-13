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
```rsut
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
