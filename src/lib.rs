#![allow(dead_code)]
#![allow(unused_variables)]

pub use derive::TreeDisplay;

mod ast;
mod tree_display;

#[cfg(test)]
mod tests {
  use super::*;
  use ast::*;

  fn build_complex_ast() -> Ast {
    // Helper to create spans

    let mut _s = 0;
    let mut _f = 0;

    let mut s = |start: usize, end: usize| {
      _s += 1;
      _s
    };

    let mut fresh = |ss: &str| {
      _f += 1;
      _f
    };

    // Create some identifiers
    let id = |name: &str, start: usize, end: usize| identifier(s(start, end), fresh(name));

    // Build a complex function definition with:
    // - Multiple parameters with type annotations and defaults
    // - Return type annotation
    // - Ability constraints
    // - Where clause with constraints
    // - A complex body with various expressions
    ffn(
      // fn keyword
      s(0, 2),
      // opener (
      s(3, 4),
      // Parameters
      vec![
        // param1: i32 = 42
        (
          param(
            None,
            None,
            s(4, 10), // name span
            fresh("param1"),
            s(11, 12),                // colon
            integer(s(13, 16)),       // i32 type
            Some(s(17, 18)),          // eq
            Some(integer(s(19, 21))), // 42
          ),
          None,
        ),
        // param2: string
        (
          param(
            None,
            None,
            s(23, 29),
            fresh("param2"),
            s(30, 31),
            string(s(32, 38)), // string type
            None,
            None,
          ),
          None,
        ),
        // label param3: bool
        (
          param(
            Some(s(40, 45)), // label span
            Some(fresh("label")),
            s(47, 53),
            fresh("param3"),
            s(54, 55),
            identifier(s(56, 60), fresh("bool")),
            Some(s(61, 62)),
            Some(identifier(s(63, 67), fresh("false"))),
          ),
          None,
        ),
      ],
      // closer )
      s(68, 69),
      // return type -> string
      Some(identifier(s(72, 78), fresh("string"))),
      // kw_with
      Some(s(80, 84)), // with
      // abilities
      vec![
        (identifier(s(86, 92), fresh("ability1")), None),
        (identifier(s(94, 100), fresh("ability2")), None),
      ],
      // kw_where
      Some(s(102, 107)), // where
      // constraints
      vec![
        (
          binary(
            Box::new(identifier(s(109, 115), fresh("T"))),
            Spanned {
              kind: 3,
              span: s(116, 117),
            },
            Box::new(identifier(s(118, 124), fresh("Debug"))),
          ),
          None,
        ),
        (
          binary(
            Box::new(identifier(s(126, 132), fresh("U"))),
            Spanned {
              kind: 4,
              span: s(133, 134),
            },
            Box::new(identifier(s(135, 141), fresh("Clone"))),
          ),
          None,
        ),
      ],
      // body
      Some(block(
        s(143, 145), // {
        vec![
          // let binding = 10 + 20
          (
            llet(
              s(147, 150), // let
              None,
              identifier(s(151, 158), fresh("binding")),
              None,
              s(159, 160), // =
              binary(
                Box::new(integer(s(161, 163))), // 10
                Spanned {
                  kind: 5,
                  span: s(164, 165),
                },
                Box::new(integer(s(166, 168))), // 20
              ),
            ),
            None,
          ),
          // if condition
          (
            iif(
              s(170, 172), // if
              binary(
                Box::new(identifier(s(173, 180), fresh("binding"))),
                Spanned {
                  kind: 6,
                  span: s(181, 182),
                },
                Box::new(integer(s(183, 185))), // 15
              ),
              // then block
              block(
                s(187, 189),
                vec![
                  // return binding
                  (
                    rreturn(
                      s(191, 197), // return
                      Some(identifier(s(198, 205), fresh("binding"))),
                    ),
                    None,
                  ),
                ],
                s(206, 208),
              ),
              // else block
              Some(eelse(
                s(210, 214), // else
                block(
                  s(215, 217),
                  vec![
                    // yield 42
                    (
                      yyield(
                        s(219, 224),                // yield
                        Some(integer(s(225, 227))), // 42
                      ),
                      None,
                    ),
                  ],
                  s(228, 230),
                ),
              )),
            ),
            None,
          ),
          // for loop
          (
            ffor(
              s(232, 235), // for
              identifier(s(236, 240), fresh("item")),
              s(241, 243), // in
              range(
                Some(integer(s(244, 245))), // 0
                s(246, 248),                // ..
                Some(integer(s(249, 251))), // 10
              ),
              block(
                s(252, 254),
                vec![
                  // call some_function(item)
                  (
                    call(
                      Box::new(identifier(s(256, 269), fresh("some_function"))),
                      s(270, 271),
                      vec![(
                        Arg {
                          label: None,
                          value: identifier(s(272, 276), fresh("item")),
                        },
                        None,
                      )],
                      s(277, 278),
                    ),
                    None,
                  ),
                ],
                s(279, 281),
              ),
            ),
            None,
          ),
          // while loop
          (
            wwhile(
              s(283, 288), // while
              binary(
                Box::new(identifier(s(289, 294), fresh("count"))),
                Spanned {
                  kind: 0,
                  span: s(295, 296),
                },
                Box::new(integer(s(297, 299))), // 100
              ),
              block(
                s(300, 302),
                vec![
                  // count = count + 1
                  (
                    llet(
                      s(304, 307),       // let
                      Some(s(308, 311)), // mut
                      identifier(s(312, 317), fresh("count")),
                      None,
                      s(318, 319), // =
                      binary(
                        Box::new(identifier(s(320, 325), fresh("count"))),
                        Spanned {
                          kind: 1,
                          span: s(326, 327),
                        },
                        Box::new(integer(s(328, 329))), // 1
                      ),
                    ),
                    None,
                  ),
                ],
                s(330, 332),
              ),
            ),
            None,
          ),
          // match expression
          (
            mmatch(
              s(334, 339), // match
              identifier(s(340, 346), fresh("status")),
              s(347, 348), // {
              vec![
                (
                  match_case(
                    identifier(s(349, 353), fresh("ok")),
                    None,
                    s(354, 355), // =>
                    block(
                      s(356, 358),
                      vec![(
                        rreturn(
                          s(360, 366), // return
                          Some(identifier(s(367, 370), fresh("ok"))),
                        ),
                        None,
                      )],
                      s(371, 373),
                    ),
                  ),
                  None,
                ),
                (
                  match_case(
                    identifier(s(375, 380), fresh("error")),
                    Some((
                      s(381, 385), // if
                      binary(
                        Box::new(identifier(s(386, 392), fresh("message"))),
                        Spanned {
                          kind: 3,
                          span: s(393, 395),
                        },
                        Box::new(string(s(396, 403))), // "timeout"
                      ),
                    )),
                    s(404, 405),
                    block(
                      s(406, 408),
                      vec![(
                        rreturn(
                          s(410, 416),                // return
                          Some(integer(s(417, 419))), // -1
                        ),
                        None,
                      )],
                      s(420, 422),
                    ),
                  ),
                  None,
                ),
              ],
              s(423, 424),
            ),
            None,
          ),
        ],
        s(425, 427),
      )),
    )
  }

  #[test]
  fn some_test() {
    print!(
      "-------\n{}\n-------\n",
      tree_display::tree_format(&build_complex_ast())
    );
  }
}
