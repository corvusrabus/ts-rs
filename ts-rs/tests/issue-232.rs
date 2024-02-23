#![allow(unused)]

use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
struct State {
    a: Result<EnumWithName, String>,
    b: Result<EnumWithName, String>,
}

#[derive(TS)]
#[ts(export)]
struct StateInlined {
    #[ts(inline)]
    a: Result<EnumWithName, String>,
    #[ts(inline)]
    b: Result<EnumWithName, String>,
}

#[derive(TS)]
#[ts(export)]
struct StateInlinedVec {
    #[ts(inline)]
    a: Vec<Result<EnumWithName, String>>,
    #[ts(inline)]
    b: Vec<Result<EnumWithName, String>>,
}

#[derive(TS)]
#[ts(export)]
struct EnumWithName {
    name: String,
    inner: Enum
}

#[derive(TS)]
#[ts(export)]
enum Enum {
    A,
    B
}

#[test]
fn issue_232() {
    println!("{}", StateInlinedVec::export_to_string().unwrap());
    assert_eq!(
        StateInlined::export_to_string().unwrap(),
        "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\
        import type { Enum } from \"./Enum\";\n\
        \n\
        export type StateInlined = { \
            a: { Ok : { name: string, inner: Enum, } } | { Err : string }, \
            b: { Ok : { name: string, inner: Enum, } } | { Err : string }, \
        };"
    );
    assert_eq!(
        State::export_to_string().unwrap(),
        "// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.\n\
        import type { EnumWithName } from \"./EnumWithName\";\n\
        \n\
        export type State = { \
            a: { Ok : EnumWithName } | { Err : string }, \
            b: { Ok : EnumWithName } | { Err : string }, \
        };"
    );
}
