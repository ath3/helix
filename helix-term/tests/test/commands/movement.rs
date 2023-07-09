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
        test_with_config(
            AppBuilder::new()
                .with_file("foo.rs", None)
                .with_config(Config {
                    keys: KeymapConfig {
                        bindings: keymap.clone(),
                        ..Default::default()
                    },
                    ..helpers::test_config()
                }),
            test,
        )
        .await?;
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_move_parent_node_start() -> anyhow::Result<()> {
    let keymap = hashmap!(
        Mode::Normal => Keymap::new(keymap!({ "Normal mode"
            "A-b" => move_parent_node_start,
        })),
        Mode::Insert => Keymap::new(keymap!({ "Insert mode"
            "A-b" => move_parent_node_start,
        })),
        Mode::Select => Keymap::new(keymap!({ "Select mode"
            "A-b" => extend_parent_node_start,
        })),
    );

    let tests = vec![
        // single cursor stays single cursor, first goes to end of current
        // node, then parent
        (
            helpers::platform_line(indoc! {r##"
                fn foo() {
                    let result = if true {
                        "yes"
                    } else {
                        "no#["|]#
                    }
                }
            "##}),
            "<A-b>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        #[\"|]#no\"
                    }
                }
            "}),
        ),
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"#[\n|]#
                    }
                }
            "}),
            "<A-b>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else #[{|]#
                        \"no\"
                    }
                }
            "}),
        ),
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else #[{|]#
                        \"no\"
                    }
                }
            "}),
            "<A-b>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } #[e|]#lse {
                        \"no\"
                    }
                }
            "}),
        ),
        // select mode extends
        (
            helpers::platform_line(indoc! {r##"
                fn foo() {
                    let result = if true {
                        "yes"
                    } else {
                        #["no"|]#
                    }
                }
            "##}),
            "v<A-b><A-b>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else #[|{
                        ]#\"no\"
                    }
                }
            "}),
        ),
        (
            helpers::platform_line(indoc! {r##"
                fn foo() {
                    let result = if true {
                        "yes"
                    } else {
                        #["no"|]#
                    }
                }
            "##}),
            "v<A-b><A-b><A-b>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } #[|else {
                        ]#\"no\"
                    }
                }
            "}),
        ),
    ];
}

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
        test_with_config(
            AppBuilder::new()
                .with_file("foo.rs", None)
                .with_config(Config {
                    keys: KeymapConfig {
                        bindings: keymap.clone(),
                        ..Default::default()
                    },
                    ..helpers::test_config()
                }),
            test,
        )
        .await?;
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_supertab_move_parent_node_end() -> anyhow::Result<()> {
    let tests = vec![
        // single cursor stays single cursor, first goes to end of current
        // node, then parent
        (
            helpers::platform_line(indoc! {r##"
                fn foo() {
                    let result = if true {
                        "yes"
                    } else {
                        "no#["|]#
                    }
                }
            "##}),
            "i<tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"#[|\n]#
                    }
                }
            "}),
        ),
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"#[\n|]#
                    }
                }
            "}),
            "i<tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"
                    }#[|\n]#
                }
            "}),
        ),
        // appending to the end of a line should still look at the current
        // line, not the next one
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no#[\"|]#
                    }
                }
            "}),
            "a<tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"
                    }#[\n|]#
                }
            "}),
        ),
        // before cursor is all whitespace, so insert tab
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        #[\"no\"|]#
                    }
                }
            "}),
            "i<tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                            #[|\"no\"]#
                    }
                }
            "}),
        ),
        // if selection spans multiple lines, it should still only look at the
        // line on which the head is
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        #[\"yes\"
                    } else {
                        \"no\"|]#
                    }
                }
            "}),
            "a<tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"
                    }#[\n|]#
                }
            "}),
        ),
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        #[\"yes\"
                    } else {
                        \"no\"|]#
                    }
                }
            "}),
            "i<tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                            #[|\"yes\"
                    } else {
                        \"no\"]#
                    }
                }
            "}),
        ),
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    #[l|]#et result = if true {
                        #(\"yes\"
                    } else {
                        \"no\"|)#
                    }
                }
            "}),
            "i<tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                        #[|l]#et result = if true {
                            #(|\"yes\"
                    } else {
                        \"no\")#
                    }
                }
            "}),
        ),
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"#[\n|]#
                    } else {
                        \"no\"#(\n|)#
                    }
                }
            "}),
            "i<tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    }#[| ]#else {
                        \"no\"
                    }#(|\n)#
                }
            "}),
        ),
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        #[\"yes\"|]#
                    } else {
                        #(\"no\"|)#
                    }
                }
            "}),
            "i<tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                            #[|\"yes\"]#
                    } else {
                            #(|\"no\")#
                    }
                }
            "}),
        ),
        // if any cursors are not preceded by all whitespace, then do the
        // supertab action
        (
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        #[\"yes\"\n|]#
                    } else {
                        \"no#(\"\n|)#
                    }
                }
            "}),
            "i<tab>",
            helpers::platform_line(indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    }#[| ]#else {
                        \"no\"
                    }#(|\n)#
                }
            "}),
        ),
    ];

    for test in tests {
        test_with_config(
            AppBuilder::new()
                .with_file("foo.rs", None)
                .with_config(Config {
                    keys: KeymapConfig {
                        supertab: Some(MappableCommand::from_str("move_parent_node_end").unwrap()),
                        ..Default::default()
                    },
                    ..helpers::test_config()
                }),
            test,
        )
        .await?;
    }

    Ok(())
}
