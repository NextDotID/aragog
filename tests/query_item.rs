use aragog::query::{QueryItem};

pub mod common;

#[test]
fn in_str_array() -> Result<(), String> {
    let item = QueryItem::field("username").in_str_array(&["felix", "gerard"]);
    common::expect_assert_eq(format!("{}", item).as_str(), r#"username IN ["felix", "gerard"]"#)?;
    Ok(())
}

#[test]
fn not_in_str_array() -> Result<(), String> {
    let item = QueryItem::field("username").not_in_str_array(&["felix", "gerard"]);
    common::expect_assert_eq(format!("{}", item).as_str(), r#"username NOT IN ["felix", "gerard"]"#)?;
    Ok(())
}

#[test]
fn in_array() -> Result<(), String> {
    let item = QueryItem::field("age").in_array(&[13, 14, 15]);
    common::expect_assert_eq(format!("{}", item).as_str(), r#"age IN [13, 14, 15]"#)?;
    let item = QueryItem::field("price").in_array(&[13.1, 14.5, 16.13]);
    common::expect_assert_eq(format!("{}", item).as_str(), r#"price IN [13.1, 14.5, 16.13]"#)?;
    Ok(())
}

#[test]
fn not_in_array() -> Result<(), String> {
    let item = QueryItem::field("age").not_in_array(&[13, 14, 15]);
    common::expect_assert_eq(format!("{}", item).as_str(), r#"age NOT IN [13, 14, 15]"#)?;
    let item = QueryItem::field("price").not_in_array(&[13.1, 14.5, 16.13]);
    common::expect_assert_eq(format!("{}", item).as_str(), r#"price NOT IN [13.1, 14.5, 16.13]"#)?;
    Ok(())
}

#[test]
fn like() -> Result<(), String> {
    let item = QueryItem::field("last_name").like("de %");
    common::expect_assert_eq(format!("{}", item).as_str(), r#"last_name LIKE "de %""#)?;
    Ok(())
}

#[test]
fn not_like() -> Result<(), String> {
    let item = QueryItem::field("last_name").not_like("de %");
    common::expect_assert_eq(format!("{}", item).as_str(), r#"last_name NOT LIKE "de %""#)?;
    Ok(())
}

#[test]
fn matches() -> Result<(), String> {
    let item = QueryItem::field("last_name").matches(r#"^/[0.9]$"#);
    common::expect_assert_eq(format!("{}", item).as_str(), r#"last_name =~ "^/[0.9]$""#)?;
    Ok(())
}

#[test]
fn does_not_match() -> Result<(), String> {
    let item = QueryItem::field("last_name").does_not_match(r#"^/[0.9]$"#);
    common::expect_assert_eq(format!("{}", item).as_str(), r#"last_name !~ "^/[0.9]$""#)?;
    Ok(())
}

#[test]
fn greater_than() -> Result<(), String> {
    let item = QueryItem::field("age").greater_than(10);
    common::expect_assert_eq(format!("{}", item).as_str(), "age > 10")?;
    Ok(())
}

#[test]
fn greater_or_equal() -> Result<(), String> {
    let item = QueryItem::field("age").greater_or_equal(10);
    common::expect_assert_eq(format!("{}", item).as_str(), "age >= 10")?;
    Ok(())
}

#[test]
fn lesser_than() -> Result<(), String> {
    let item = QueryItem::field("age").lesser_than(10);
    common::expect_assert_eq(format!("{}", item).as_str(), "age < 10")?;
    Ok(())
}

#[test]
fn lesser_or_equal() -> Result<(), String> {
    let item = QueryItem::field("age").lesser_or_equal(10);
    common::expect_assert_eq(format!("{}", item).as_str(), "age <= 10")?;
    Ok(())
}

#[test]
fn equals() -> Result<(), String> {
    let item = QueryItem::field("age").equals(10);
    common::expect_assert_eq(format!("{}", item).as_str(), "age == 10")?;
    Ok(())
}

#[test]
fn different_than() -> Result<(), String> {
    let item = QueryItem::field("age").different_than(10);
    common::expect_assert_eq(format!("{}", item).as_str(), "age != 10")?;
    Ok(())
}


#[test]
fn equals_str() -> Result<(), String> {
    let item = QueryItem::field("name").equals_str("felix");
    common::expect_assert_eq(format!("{}", item).as_str(), r#"name == "felix""#)?;
    Ok(())
}

#[test]
fn different_than_str() -> Result<(), String> {
    let item = QueryItem::field("name").different_than_str("felix");
    common::expect_assert_eq(format!("{}", item).as_str(), r#"name != "felix""#)?;
    Ok(())
}

#[test]
fn is_null() -> Result<(), String> {
    let item = QueryItem::field("name").is_null();
    common::expect_assert_eq(format!("{}", item).as_str(), "name == null")?;
    Ok(())
}

#[test]
fn not_null() -> Result<(), String> {
    let item = QueryItem::field("name").not_null();
    common::expect_assert_eq(format!("{}", item).as_str(), "name != null")?;
    Ok(())
}

#[test]
fn is_true() -> Result<(), String> {
    let item = QueryItem::field("is_company").is_true();
    common::expect_assert_eq(format!("{}", item).as_str(), "is_company == true")?;
    Ok(())
}

#[test]
fn is_false() -> Result<(), String> {
    let item = QueryItem::field("is_company").is_false();
    common::expect_assert_eq(format!("{}", item).as_str(), "is_company == false")?;
    Ok(())
}

mod array_testing {
    use super::*;

    #[test]
    fn all() -> Result<(), String> {
        let item = QueryItem::all("emails").not_null();
        common::expect_assert_eq(format!("{}", item).as_str(), "emails ALL != null")?;
        Ok(())
    }

    #[test]
    fn none() -> Result<(), String> {
        let item = QueryItem::none("emails").is_null();
        common::expect_assert_eq(format!("{}", item).as_str(), "emails NONE == null")?;
        Ok(())
    }

    #[test]
    fn any() -> Result<(), String> {
        let item = QueryItem::any("authorizations").is_true();
        common::expect_assert_eq(format!("{}", item).as_str(), "authorizations ANY == true")?;
        Ok(())
    }

}
