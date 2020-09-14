use aragog::query::{Query, QueryItem};

pub mod common;

#[test]
fn provides_correct_string() -> Result<(), String> {
    let query = Query::new(
        QueryItem::field("username").equals_str("felix")
    ).and(
        QueryItem::field("age").greater_than(15)
    );
    let query_str = query.render();
    common::expect_assert_eq(query_str.as_str(), r#"FILTER i.username == "felix" && i.age > 15"#)?;
    Ok(())
}