#[macro_use] extern crate aragog;

use aragog::query::{Query, Comparison, Filter, SortDirection};

pub mod common;

mod comparison {
    use super::*;

    #[test]
    fn in_str_array() -> Result<(), String> {
        let item = Comparison::field("username").in_str_array(&["felix", "gerard"]);
        common::expect_assert_eq(format!("{}", item).as_str(), r#"i.username IN ["felix", "gerard"]"#)?;
        Ok(())
    }

    #[test]
    fn not_in_str_array() -> Result<(), String> {
        let item = Comparison::field("username").not_in_str_array(&["felix", "gerard"]);
        common::expect_assert_eq(format!("{}", item).as_str(), r#"i.username NOT IN ["felix", "gerard"]"#)?;
        Ok(())
    }

    #[test]
    fn in_array() -> Result<(), String> {
        let item = Comparison::field("age").in_array(&[13, 14, 15]);
        common::expect_assert_eq(format!("{}", item).as_str(), r#"i.age IN [13, 14, 15]"#)?;
        let item = Comparison::field("price").in_array(&[13.1, 14.5, 16.13]);
        common::expect_assert_eq(format!("{}", item).as_str(), r#"i.price IN [13.1, 14.5, 16.13]"#)?;
        Ok(())
    }

    #[test]
    fn not_in_array() -> Result<(), String> {
        let item = Comparison::field("age").not_in_array(&[13, 14, 15]);
        common::expect_assert_eq(format!("{}", item).as_str(), r#"i.age NOT IN [13, 14, 15]"#)?;
        let item = Comparison::field("price").not_in_array(&[13.1, 14.5, 16.13]);
        common::expect_assert_eq(format!("{}", item).as_str(), r#"i.price NOT IN [13.1, 14.5, 16.13]"#)?;
        Ok(())
    }

    #[test]
    fn like() -> Result<(), String> {
        let item = Comparison::field("last_name").like("de %");
        common::expect_assert_eq(format!("{}", item).as_str(), r#"i.last_name LIKE "de %""#)?;
        Ok(())
    }

    #[test]
    fn not_like() -> Result<(), String> {
        let item = Comparison::field("last_name").not_like("de %");
        common::expect_assert_eq(format!("{}", item).as_str(), r#"i.last_name NOT LIKE "de %""#)?;
        Ok(())
    }

    #[test]
    fn matches() -> Result<(), String> {
        let item = Comparison::field("last_name").matches(r#"^/[0.9]$"#);
        common::expect_assert_eq(format!("{}", item).as_str(), r#"i.last_name =~ "^/[0.9]$""#)?;
        Ok(())
    }

    #[test]
    fn does_not_match() -> Result<(), String> {
        let item = Comparison::field("last_name").does_not_match(r#"^/[0.9]$"#);
        common::expect_assert_eq(format!("{}", item).as_str(), r#"i.last_name !~ "^/[0.9]$""#)?;
        Ok(())
    }

    #[test]
    fn greater_than() -> Result<(), String> {
        let item = Comparison::field("age").greater_than(10);
        common::expect_assert_eq(format!("{}", item).as_str(), "i.age > 10")?;
        Ok(())
    }

    #[test]
    fn greater_or_equal() -> Result<(), String> {
        let item = Comparison::field("age").greater_or_equal(10);
        common::expect_assert_eq(format!("{}", item).as_str(), "i.age >= 10")?;
        Ok(())
    }

    #[test]
    fn lesser_than() -> Result<(), String> {
        let item = Comparison::field("age").lesser_than(10);
        common::expect_assert_eq(format!("{}", item).as_str(), "i.age < 10")?;
        Ok(())
    }

    #[test]
    fn lesser_or_equal() -> Result<(), String> {
        let item = Comparison::field("age").lesser_or_equal(10);
        common::expect_assert_eq(format!("{}", item).as_str(), "i.age <= 10")?;
        Ok(())
    }

    #[test]
    fn equals() -> Result<(), String> {
        let item = Comparison::field("age").equals(10);
        common::expect_assert_eq(format!("{}", item).as_str(), "i.age == 10")?;
        Ok(())
    }

    #[test]
    fn different_than() -> Result<(), String> {
        let item = Comparison::field("age").different_than(10);
        common::expect_assert_eq(format!("{}", item).as_str(), "i.age != 10")?;
        Ok(())
    }


    #[test]
    fn equals_str() -> Result<(), String> {
        let item = Comparison::field("name").equals_str("felix");
        common::expect_assert_eq(format!("{}", item).as_str(), r#"i.name == "felix""#)?;
        Ok(())
    }

    #[test]
    fn different_than_str() -> Result<(), String> {
        let item = Comparison::field("name").different_than_str("felix");
        common::expect_assert_eq(format!("{}", item).as_str(), r#"i.name != "felix""#)?;
        Ok(())
    }

    #[test]
    fn is_null() -> Result<(), String> {
        let item = Comparison::field("name").is_null();
        common::expect_assert_eq(format!("{}", item).as_str(), "i.name == null")?;
        Ok(())
    }

    #[test]
    fn not_null() -> Result<(), String> {
        let item = Comparison::field("name").not_null();
        common::expect_assert_eq(format!("{}", item).as_str(), "i.name != null")?;
        Ok(())
    }

    #[test]
    fn is_true() -> Result<(), String> {
        let item = Comparison::field("is_company").is_true();
        common::expect_assert_eq(format!("{}", item).as_str(), "i.is_company == true")?;
        Ok(())
    }

    #[test]
    fn is_false() -> Result<(), String> {
        let item = Comparison::field("is_company").is_false();
        common::expect_assert_eq(format!("{}", item).as_str(), "i.is_company == false")?;
        Ok(())
    }

    mod array_testing {
        use super::*;

        #[test]
        fn all() -> Result<(), String> {
            let item = Comparison::all("emails").not_null();
            common::expect_assert_eq(format!("{}", item).as_str(), "i.emails ALL != null")?;
            let item = compare!(all "emails").not_null();
            common::expect_assert_eq(format!("{}", item).as_str(), "i.emails ALL != null")?;
            Ok(())
        }

        #[test]
        fn none() -> Result<(), String> {
            let item = Comparison::none("emails").is_null();
            common::expect_assert_eq(format!("{}", item).as_str(), "i.emails NONE == null")?;
            let item = compare!(none "emails").is_null();
            common::expect_assert_eq(format!("{}", item).as_str(), "i.emails NONE == null")?;
            Ok(())
        }

        #[test]
        fn any() -> Result<(), String> {
            let item = Comparison::any("authorizations").is_true();
            common::expect_assert_eq(format!("{}", item).as_str(), "i.authorizations ANY == true")?;
            let item = compare!(any "authorizations").is_true();
            common::expect_assert_eq(format!("{}", item).as_str(), "i.authorizations ANY == true")?;
            Ok(())
        }

    }

}

mod filter {
    use super::*;
    use aragog::query::Filter;

    #[test]
    fn provides_correct_string() -> Result<(), String> {
        let filter = Filter::new(
            Comparison::field("username").equals_str("felix")
        ).and(
            Comparison::field("age").greater_than(15)
        );
        let filter_str = filter.render();
        common::expect_assert_eq(filter_str.as_str(), r#"FILTER i.username == "felix" && i.age > 15"#)?;
        Ok(())
    }

    #[test]
    fn succeeds_on_complex_queries() -> Result<(), String> {
        let filter = Filter::new(
            Comparison::field("company_name").not_like("%google%")
        ).and(
            Comparison::field("company_age").greater_than(15)
        ).or(
            Comparison::any("emails").like("%gmail.com")
        ).and(
            Comparison::field("roles").in_str_array(&["SHIPPER", "FORWARDER"])
        );
        let filter_str = filter.render();
        common::expect_assert_eq(
            filter_str.as_str(),
            r#"FILTER i.company_name NOT LIKE "%google%" && i.company_age > 15 || i.emails ANY LIKE "%gmail.com" && i.roles IN ["SHIPPER", "FORWARDER"]"#)?;
        Ok(())
    }
}

#[test]
fn complex_query_works() -> Result<(), String> {
    let query = Query::new("Companies")
        .filter(Filter::new(Comparison::any("emails").like("%gmail.com")))
        .sort("company_name", None)
        .sort("company_age", Some(SortDirection::Desc))
        .limit(5, None)
        .distinct();
    common::expect_assert_eq(
        query.render().as_str(),
        r#"FOR i in Companies FILTER i.emails ANY LIKE "%gmail.com" SORT i.company_name, i.company_age DESC LIMIT 5 DISTINCT return i"#
    )?;
    Ok(())
}

#[test]
fn macros_work() -> Result<(), String> {
    let query = query!("Companies")
        .filter(Filter::new(compare!(any "emails").like("%gmail.com")))
        .sort("company_name", Some(SortDirection::Desc))
        .sort("company_age", None)
        .limit(5, None)
        .distinct();
    common::expect_assert_eq(
        query.render().as_str(),
        r#"FOR i in Companies FILTER i.emails ANY LIKE "%gmail.com" SORT i.company_name DESC, i.company_age LIMIT 5 DISTINCT return i"#
    )?;
    Ok(())
}

#[test]
fn empty_query_works() -> Result<(), String> {
    let query = Query::new("Companies");
    common::expect_assert_eq(query.render().as_str(), "FOR i in Companies return i")?;
    Ok(())
}