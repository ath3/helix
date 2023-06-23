use super::*;

#[tokio::test(flavor = "multi_thread")]
async fn select_all_siblings() -> anyhow::Result<()> {
    let tests = vec![
        // basic tests
        (
            helpers::platform_line(indoc! {r##"
                let foo = bar(#[a|]#, b, c);
            "##}),
            "<A-a>",
            helpers::platform_line(indoc! {r##"
                let foo = bar(#[a|]#, #(b|)#, #(c|)#);
            "##}),
        ),
        (
            helpers::platform_line(indoc! {r##"
                let a = [
                    #[1|]#,
                    2,
                    3,
                    4,
                    5,
                ];
            "##}),
            "<A-a>",
            helpers::platform_line(indoc! {r##"
                let a = [
                    #[1|]#,
                    #(2|)#,
                    #(3|)#,
                    #(4|)#,
                    #(5|)#,
                ];
            "##}),
        ),
        // direction is preserved
        (
            helpers::platform_line(indoc! {r##"
                let a = [
                    #[|1]#,
                    2,
                    3,
                    4,
                    5,
                ];
            "##}),
            "<A-a>",
            helpers::platform_line(indoc! {r##"
                let a = [
                    #[|1]#,
                    #(|2)#,
                    #(|3)#,
                    #(|4)#,
                    #(|5)#,
                ];
            "##}),
        ),
        // can't pick any more siblings - selection stays the same
        (
            helpers::platform_line(indoc! {r##"
                let a = [
                    #[1|]#,
                    #(2|)#,
                    #(3|)#,
                    #(4|)#,
                    #(5|)#,
                ];
            "##}),
            "<A-a>",
            helpers::platform_line(indoc! {r##"
                let a = [
                    #[1|]#,
                    #(2|)#,
                    #(3|)#,
                    #(4|)#,
                    #(5|)#,
                ];
            "##}),
        ),
        // each cursor does the sibling select independently
        (
            helpers::platform_line(indoc! {r##"
                let a = [
                    #[1|]#,
                    2,
                    3,
                    4,
                    5,
                ];

                let b = [
                    #("one"|)#,
                    "two",
                    "three",
                    "four",
                    "five",
                ];
            "##}),
            "<A-a>",
            helpers::platform_line(indoc! {r##"
                let a = [
                    #[1|]#,
                    #(2|)#,
                    #(3|)#,
                    #(4|)#,
                    #(5|)#,
                ];

                let b = [
                    #("one"|)#,
                    #("two"|)#,
                    #("three"|)#,
                    #("four"|)#,
                    #("five"|)#,
                ];
            "##}),
        ),
        // conflicting sibling selections get normalized. Here, the primary
        // selection would choose every list item, but because the secondary
        // range covers more than one item, the descendent is the entire list,
        // which means the sibling is the assignment. The list item ranges just
        // get normalized out since the list itself becomes selected.
        (
            helpers::platform_line(indoc! {r##"
                let a = [
                    #[1|]#,
                    2,
                    #(3,
                    4|)#,
                    5,
                ];
            "##}),
            "<A-a>",
            helpers::platform_line(indoc! {r##"
                let #(a|)# = #[[
                    1,
                    2,
                    3,
                    4,
                    5,
                ]|]#;
            "##}),
        ),
    ];

    for test in tests {
        test_with_config(AppBuilder::new().with_file("foo.rs", None), test).await?;
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn select_all_children() -> anyhow::Result<()> {
    let tests = vec![
        // basic tests
        (
            helpers::platform_line(indoc! {r##"
                let foo = bar#[(a, b, c)|]#;
            "##}),
            "<A-I>",
            helpers::platform_line(indoc! {r##"
                let foo = bar(#[a|]#, #(b|)#, #(c|)#);
            "##}),
        ),
        (
            helpers::platform_line(indoc! {r##"
                let a = #[[
                    1,
                    2,
                    3,
                    4,
                    5,
                ]|]#;
            "##}),
            "<A-I>",
            helpers::platform_line(indoc! {r##"
                let a = [
                    #[1|]#,
                    #(2|)#,
                    #(3|)#,
                    #(4|)#,
                    #(5|)#,
                ];
            "##}),
        ),
        // direction is preserved
        (
            helpers::platform_line(indoc! {r##"
                let a = #[|[
                    1,
                    2,
                    3,
                    4,
                    5,
                ]]#;
            "##}),
            "<A-I>",
            helpers::platform_line(indoc! {r##"
                let a = [
                    #[|1]#,
                    #(|2)#,
                    #(|3)#,
                    #(|4)#,
                    #(|5)#,
                ];
            "##}),
        ),
        // can't pick any more children - selection stays the same
        (
            helpers::platform_line(indoc! {r##"
                let a = [
                    #[1|]#,
                    #(2|)#,
                    #(3|)#,
                    #(4|)#,
                    #(5|)#,
                ];
            "##}),
            "<A-I>",
            helpers::platform_line(indoc! {r##"
                let a = [
                    #[1|]#,
                    #(2|)#,
                    #(3|)#,
                    #(4|)#,
                    #(5|)#,
                ];
            "##}),
        ),
        // each cursor does the sibling select independently
        (
            helpers::platform_line(indoc! {r##"
                let a = #[|[
                    1,
                    2,
                    3,
                    4,
                    5,
                ]]#;

                let b = #([
                    "one",
                    "two",
                    "three",
                    "four",
                    "five",
                ]|)#;
            "##}),
            "<A-I>",
            helpers::platform_line(indoc! {r##"
                let a = [
                    #[|1]#,
                    #(|2)#,
                    #(|3)#,
                    #(|4)#,
                    #(|5)#,
                ];

                let b = [
                    #("one"|)#,
                    #("two"|)#,
                    #("three"|)#,
                    #("four"|)#,
                    #("five"|)#,
                ];
            "##}),
        ),
    ];

    for test in tests {
        test_with_config(AppBuilder::new().with_file("foo.rs", None), test).await?;
    }

    Ok(())
}
