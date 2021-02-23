# Hooks

The `Record` trait provides the following hooks:
- **before** hooks:
    - `before_create` : executed before document creation (`DatabaseRecord::create`)
    - `before_save` : executed before document save (`DatabaseRecord::save`)
    - `before_delete` : executed before document deletion (`DatabaseRecord::delete`)
    - `before_write` : executed before both document creation **and** save.
    - `before_all` : executed before document creation, save and deletion.
- **after** hooks:
    - `after_create` : executed after the document creation (`DatabaseRecord::create`)
    - `after_save` : executed after the document save (`DatabaseRecord::save`)
    - `after_delete` : executed after the document deletion (`DatabaseRecord::delete`)
    - `after_write` : executed after both document creation **and** save.
    - `after_all` : executed after both document creation, save and deletion.

You can register various methods in these hooks with the following syntax:
```rust
#[hook(before_create(func = "my_method"))]
```

The hooked methods can follow various patterns using the following options:
- `is_async` the method is async
- `db_access` the method uses the db access

By default both options are set to `false`.

You can combine options to have an `async` hook with db access to execute document operations automatically.
If you combine a lot of operations, like creating documents in hooks or chaining operations make sure to:
- avoid **circular operations**
- use [Transaction](./transactions.md) for safety

## Hook Patterns

### Simple hook with no options
```rust
#[hook(before_create(func = "my_method"))]
```
*my_method* can be either:
- ```rust 
  fn my_method(&self) -> Result<(), aragog::ServiceError>
  ```
- ```rust 
  fn my_method(&mut self) -> Result<(), aragog::ServiceError>
  ```

### Async hook
```rust
#[hook(before_create(func = "my_method", is_async = true))]
```
*my_method* can be either:
- ```rust 
  async fn my_method(&self) -> Result<(), aragog::ServiceError>
  ```
- ```rust 
  async fn my_method(&mut self) -> Result<(), aragog::ServiceError>
  ```

> If you use `aragog` with the `blocking` feature then this option will have no effect.


### Hook with database access
```rust
#[hook(before_create(func = "my_method", db_access = true))]
```
*my_method* can be either:
- ```rust 
  fn my_method<D>(&self, db_access: &D) -> Result<(), aragog::ServiceError> where D: aragog::DatabaseAccess
  ```
- ```rust 
  fn my_method<D>(&mut self, db_access: &D) -> Result<(), aragog::ServiceError> where D: aragog::DatabaseAccess
  ```

> If you want to use the database access, using also `is_async = true` would be recommended