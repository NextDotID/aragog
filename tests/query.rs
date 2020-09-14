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

#[test]
fn succeeds_on_complex_queries() -> Result<(), String> {
    let query = Query::new(
        QueryItem::field("company_name").not_like("%google%")
    ).and(
        QueryItem::field("company_age").greater_than(15)
    ).or(
        QueryItem::any("emails").like("%gmail.com")
    ).and(
        QueryItem::field("roles").in_str_array(&["SHIPPER", "FORWARDER"])
    );
    let query_str = query.render();
    common::expect_assert_eq(
        query_str.as_str(),
        r#"FILTER i.company_name NOT LIKE "%google%" && i.company_age > 15 || i.emails ANY LIKE "%gmail.com" && i.roles IN ["SHIPPER", "FORWARDER"]"#)?;
    Ok(())
}