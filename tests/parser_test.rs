use jackc::jack::*;
use std::path::PathBuf;
use std::{env, fs};

fn read_test_file(filename: &str) -> String {
    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let path = dir.join("tests").join("testdata").join(filename);
    fs::read_to_string(path).expect("couldn't load test file")
}

#[test]
fn array_test_jack_test() {
    let source = read_test_file("ArrayTest.jack");
    let tokenizer = Tokenizer::new(&source);
    let mut parser = Parser::new(tokenizer);

    assert_eq!(
        parser.parse(),
        Some(Class {
            name: "Main".into(),
            vars: vec![],
            subs: vec![Subroutine {
                kind: SubroutineKind::Function,
                typ: SubroutineType::Void,
                name: "main".into(),
                params: vec![],
                body: SubroutineBody {
                    vars: vec![
                        LocalVars {
                            typ: VarType::ClassName("Array".into()),
                            names: vec!["a".into()],
                        },
                        LocalVars {
                            typ: VarType::Int,
                            names: vec!["length".into()],
                        },
                        LocalVars {
                            typ: VarType::Int,
                            names: vec!["i".into(), "sum".into()],
                        },
                    ],
                    statements: vec![
                        Statement::Let {
                            lhs: "length".into(),
                            index: None,
                            rhs: Expr::Term(Term::SubroutineCall(SubroutineCall {
                                receiver: Some("Keyboard".into()),
                                subroutine: "readInt".into(),
                                args: vec![Expr::Term(Term::StrConst("HOW MANY NUMBERS? ".into()))],
                            })),
                        },
                        Statement::Let {
                            lhs: "a".into(),
                            index: None,
                            rhs: Expr::Term(Term::SubroutineCall(SubroutineCall {
                                receiver: Some("Array".into()),
                                subroutine: "new".into(),
                                args: vec![Expr::Term(Term::Var("length".into()))],
                            })),
                        },
                        Statement::Let {
                            lhs: "i".into(),
                            index: None,
                            rhs: Expr::Term(Term::IntConst(0)),
                        },
                        Statement::While {
                            condition: Expr::Binary(
                                BinaryOp::LessThan,
                                Term::Var("i".into()),
                                Box::new(Expr::Term(Term::Var("length".into())))
                            ),
                            body: vec![
                                Statement::Let {
                                    lhs: "a".into(),
                                    index: Some(Expr::Term(Term::Var("i".into()))),
                                    rhs: Expr::Term(Term::SubroutineCall(SubroutineCall {
                                        receiver: Some("Keyboard".into()),
                                        subroutine: "readInt".into(),
                                        args: vec![Expr::Term(Term::StrConst(
                                            "ENTER THE NEXT NUMBER: ".into()
                                        ))],
                                    })),
                                },
                                Statement::Let {
                                    lhs: "i".into(),
                                    index: None,
                                    rhs: Expr::Binary(
                                        BinaryOp::Add,
                                        Term::Var("i".into()),
                                        Box::new(Expr::Term(Term::IntConst(1)))
                                    ),
                                }
                            ],
                        },
                        Statement::Let {
                            lhs: "i".into(),
                            index: None,
                            rhs: Expr::Term(Term::IntConst(0)),
                        },
                        Statement::Let {
                            lhs: "sum".into(),
                            index: None,
                            rhs: Expr::Term(Term::IntConst(0)),
                        },
                        Statement::While {
                            condition: Expr::Binary(
                                BinaryOp::LessThan,
                                Term::Var("i".into()),
                                Box::new(Expr::Term(Term::Var("length".into())))
                            ),
                            body: vec![
                                Statement::Let {
                                    lhs: "sum".into(),
                                    index: None,
                                    rhs: Expr::Binary(
                                        BinaryOp::Add,
                                        Term::Var("sum".into()),
                                        Box::new(Expr::Term(Term::IndexedVar(
                                            "a".into(),
                                            Box::new(Expr::Term(Term::Var("i".into())))
                                        )))
                                    ),
                                },
                                Statement::Let {
                                    lhs: "i".into(),
                                    index: None,
                                    rhs: Expr::Binary(
                                        BinaryOp::Add,
                                        Term::Var("i".into()),
                                        Box::new(Expr::Term(Term::IntConst(1)))
                                    ),
                                }
                            ],
                        },
                        Statement::Do(SubroutineCall {
                            receiver: Some("Output".into()),
                            subroutine: "printString".into(),
                            args: vec![Expr::Term(Term::StrConst("THE AVERAGE IS: ".into()))],
                        }),
                        Statement::Do(SubroutineCall {
                            receiver: Some("Output".into()),
                            subroutine: "printInt".into(),
                            args: vec![Expr::Binary(
                                BinaryOp::Divide,
                                Term::Var("sum".into()),
                                Box::new(Expr::Term(Term::Var("length".into())))
                            )],
                        }),
                        Statement::Do(SubroutineCall {
                            receiver: Some("Output".into()),
                            subroutine: "println".into(),
                            args: vec![],
                        }),
                        Statement::Return(None),
                    ],
                },
            }],
        })
    );
}

#[test]
fn main_jack_test() {
    let source = read_test_file("Main.jack");
    let tokenizer = Tokenizer::new(&source);
    let mut parser = Parser::new(tokenizer);

    assert_eq!(
        parser.parse(),
        Some(Class {
            name: "Main".into(),
            vars: vec![ClassVars {
                kind: ClassVarKind::Static,
                typ: VarType::Boolean,
                names: vec!["test".into()],
            }],
            subs: vec![
                Subroutine {
                    kind: SubroutineKind::Function,
                    typ: SubroutineType::Void,
                    name: "main".into(),
                    params: vec![],
                    body: SubroutineBody {
                        vars: vec![LocalVars {
                            typ: VarType::ClassName("SquareGame".into()),
                            names: vec!["game".into()]
                        }],
                        statements: vec![
                            Statement::Let {
                                lhs: "game".into(),
                                index: None,
                                rhs: Expr::Term(Term::SubroutineCall(SubroutineCall {
                                    receiver: Some("SquareGame".into()),
                                    subroutine: "new".into(),
                                    args: vec![],
                                }))
                            },
                            Statement::Do(SubroutineCall {
                                receiver: Some("game".into()),
                                subroutine: "run".into(),
                                args: vec![],
                            }),
                            Statement::Do(SubroutineCall {
                                receiver: Some("game".into()),
                                subroutine: "dispose".into(),
                                args: vec![],
                            }),
                            Statement::Return(None),
                        ]
                    }
                },
                Subroutine {
                    kind: SubroutineKind::Function,
                    typ: SubroutineType::Void,
                    name: "test".into(),
                    params: vec![],
                    body: SubroutineBody {
                        vars: vec![
                            LocalVars {
                                typ: VarType::Int,
                                names: vec!["i".into(), "j".into()],
                            },
                            LocalVars {
                                typ: VarType::ClassName("String".into()),
                                names: vec!["s".into()],
                            },
                            LocalVars {
                                typ: VarType::ClassName("Array".into()),
                                names: vec!["a".into()],
                            },
                        ],
                        statements: vec![
                            Statement::If {
                                condition: Expr::Term(Term::KeywordConst(KeywordConst::False)),
                                if_body: vec![
                                    Statement::Let {
                                        lhs: "s".into(),
                                        index: None,
                                        rhs: Expr::Term(Term::StrConst("string constant".into())),
                                    },
                                    Statement::Let {
                                        lhs: "s".into(),
                                        index: None,
                                        rhs: Expr::Term(Term::KeywordConst(KeywordConst::Null)),
                                    },
                                    Statement::Let {
                                        lhs: "a".into(),
                                        index: Some(Expr::Term(Term::IntConst(1))),
                                        rhs: Expr::Term(Term::IndexedVar(
                                            "a".into(),
                                            Box::new(Expr::Term(Term::IntConst(2)))
                                        ),)
                                    }
                                ],
                                else_body: Some(vec![
                                    Statement::Let {
                                        lhs: "i".into(),
                                        index: None,
                                        rhs: Expr::Binary(
                                            BinaryOp::Multiply,
                                            Term::Var("i".into()),
                                            Box::new(Expr::Term(Term::Bracketed(Box::new(
                                                Expr::Term(Term::Unary(
                                                    UnaryOp::Minus,
                                                    Box::new(Term::Var("j".into()))
                                                ))
                                            ))))
                                        ),
                                    },
                                    Statement::Let {
                                        lhs: "j".into(),
                                        index: None,
                                        rhs: Expr::Binary(
                                            BinaryOp::Divide,
                                            Term::Var("j".into()),
                                            Box::new(Expr::Term(Term::Bracketed(Box::new(
                                                Expr::Term(Term::Unary(
                                                    UnaryOp::Minus,
                                                    Box::new(Term::IntConst(2))
                                                ))
                                            ))))
                                        )
                                    },
                                    Statement::Let {
                                        lhs: "i".into(),
                                        index: None,
                                        rhs: Expr::Binary(
                                            BinaryOp::Or,
                                            Term::Var("i".into()),
                                            Box::new(Expr::Term(Term::Var("j".into())))
                                        )
                                    }
                                ]),
                            },
                            Statement::Return(None)
                        ],
                    }
                }
            ],
        })
    );
}

#[test]
fn square_jack_test() {
    let source = read_test_file("Square.jack");
    let tokenizer = Tokenizer::new(&source);
    let mut parser = Parser::new(tokenizer);

    assert_eq!(
        parser.parse(),
        Some(Class {
            name: "Square".into(),
            vars: vec![
                ClassVars {
                    kind: ClassVarKind::Field,
                    typ: VarType::Int,
                    names: vec!["x".into(), "y".into()],
                },
                ClassVars {
                    kind: ClassVarKind::Field,
                    typ: VarType::Int,
                    names: vec!["size".into()],
                }
            ],
            subs: vec![
                Subroutine {
                    kind: SubroutineKind::Constructor,
                    typ: SubroutineType::NonVoid(VarType::ClassName("Square".into())),
                    name: "new".into(),
                    params: vec![
                        Param {
                            typ: VarType::Int,
                            name: "Ax".into(),
                        },
                        Param {
                            typ: VarType::Int,
                            name: "Ay".into(),
                        },
                        Param {
                            typ: VarType::Int,
                            name: "Asize".into(),
                        }
                    ],
                    body: SubroutineBody {
                        vars: vec![],
                        statements: vec![
                            Statement::Let {
                                lhs: "x".into(),
                                index: None,
                                rhs: Expr::Term(Term::Var("Ax".into())),
                            },
                            Statement::Let {
                                lhs: "y".into(),
                                index: None,
                                rhs: Expr::Term(Term::Var("Ay".into())),
                            },
                            Statement::Let {
                                lhs: "size".into(),
                                index: None,
                                rhs: Expr::Term(Term::Var("Asize".into())),
                            },
                            Statement::Do(SubroutineCall {
                                receiver: None,
                                subroutine: "draw".into(),
                                args: vec![],
                            }),
                            Statement::Return(Some(Expr::Term(Term::KeywordConst(
                                KeywordConst::This
                            )))),
                        ],
                    },
                },
                Subroutine {
                    kind: SubroutineKind::Method,
                    typ: SubroutineType::Void,
                    name: "dispose".into(),
                    params: vec![],
                    body: SubroutineBody {
                        vars: vec![],
                        statements: vec![
                            Statement::Do(SubroutineCall {
                                receiver: Some("Memory".into()),
                                subroutine: "deAlloc".into(),
                                args: vec![Expr::Term(Term::KeywordConst(KeywordConst::This))],
                            }),
                            Statement::Return(None)
                        ],
                    },
                },
                Subroutine {
                    kind: SubroutineKind::Method,
                    typ: SubroutineType::Void,
                    name: "draw".into(),
                    params: vec![],
                    body: SubroutineBody {
                        vars: vec![],
                        statements: vec![
                            Statement::Do(SubroutineCall {
                                receiver: Some("Screen".into()),
                                subroutine: "setColor".into(),
                                args: vec![Expr::Term(Term::KeywordConst(KeywordConst::True))],
                            }),
                            Statement::Do(SubroutineCall {
                                receiver: Some("Screen".into()),
                                subroutine: "drawRectangle".into(),
                                args: vec![
                                    Expr::Term(Term::Var("x".into())),
                                    Expr::Term(Term::Var("y".into())),
                                    Expr::Binary(
                                        BinaryOp::Add,
                                        Term::Var("x".into()),
                                        Box::new(Expr::Term(Term::Var("size".into())))
                                    ),
                                    Expr::Binary(
                                        BinaryOp::Add,
                                        Term::Var("y".into()),
                                        Box::new(Expr::Term(Term::Var("size".into())))
                                    ),
                                ],
                            }),
                            Statement::Return(None),
                        ],
                    },
                },
                Subroutine {
                    kind: SubroutineKind::Method,
                    typ: SubroutineType::Void,
                    name: "erase".into(),
                    params: vec![],
                    body: SubroutineBody {
                        vars: vec![],
                        statements: vec![
                            Statement::Do(SubroutineCall {
                                receiver: Some("Screen".into()),
                                subroutine: "setColor".into(),
                                args: vec![Expr::Term(Term::KeywordConst(KeywordConst::False))],
                            }),
                            Statement::Do(SubroutineCall {
                                receiver: Some("Screen".into()),
                                subroutine: "drawRectangle".into(),
                                args: vec![
                                    Expr::Term(Term::Var("x".into())),
                                    Expr::Term(Term::Var("y".into())),
                                    Expr::Binary(
                                        BinaryOp::Add,
                                        Term::Var("x".into()),
                                        Box::new(Expr::Term(Term::Var("size".into())))
                                    ),
                                    Expr::Binary(
                                        BinaryOp::Add,
                                        Term::Var("y".into()),
                                        Box::new(Expr::Term(Term::Var("size".into())))
                                    ),
                                ],
                            }),
                            Statement::Return(None),
                        ],
                    },
                },
                Subroutine {
                    kind: SubroutineKind::Method,
                    typ: SubroutineType::Void,
                    name: "incSize".into(),
                    params: vec![],
                    body: SubroutineBody {
                        vars: vec![],
                        statements: vec![
                            Statement::If {
                                condition: Expr::Binary(
                                    BinaryOp::And,
                                    Term::Bracketed(Box::new(Expr::Binary(
                                        BinaryOp::LessThan,
                                        Term::Bracketed(Box::new(Expr::Binary(
                                            BinaryOp::Add,
                                            Term::Var("y".into()),
                                            Box::new(Expr::Term(Term::Var("size".into())))
                                        ))),
                                        Box::new(Expr::Term(Term::IntConst(254)))
                                    ))),
                                    Box::new(Expr::Term(Term::Bracketed(Box::new(Expr::Binary(
                                        BinaryOp::LessThan,
                                        Term::Bracketed(Box::new(Expr::Binary(
                                            BinaryOp::Add,
                                            Term::Var("x".into()),
                                            Box::new(Expr::Term(Term::Var("size".into())))
                                        ))),
                                        Box::new(Expr::Term(Term::IntConst(510)))
                                    ))))),
                                ),
                                if_body: vec![
                                    Statement::Do(SubroutineCall {
                                        receiver: None,
                                        subroutine: "erase".into(),
                                        args: vec![],
                                    }),
                                    Statement::Let {
                                        lhs: "size".into(),
                                        index: None,
                                        rhs: Expr::Binary(
                                            BinaryOp::Add,
                                            Term::Var("size".into()),
                                            Box::new(Expr::Term(Term::IntConst(2)))
                                        ),
                                    },
                                    Statement::Do(SubroutineCall {
                                        receiver: None,
                                        subroutine: "draw".into(),
                                        args: vec![],
                                    }),
                                ],
                                else_body: None,
                            },
                            Statement::Return(None),
                        ],
                    },
                },
                Subroutine {
                    kind: SubroutineKind::Method,
                    typ: SubroutineType::Void,
                    name: "decSize".into(),
                    params: vec![],
                    body: SubroutineBody {
                        vars: vec![],
                        statements: vec![
                            Statement::If {
                                condition: Expr::Binary(
                                    BinaryOp::GreaterThan,
                                    Term::Var("size".into()),
                                    Box::new(Expr::Term(Term::IntConst(2)))
                                ),
                                if_body: vec![
                                    Statement::Do(SubroutineCall {
                                        receiver: None,
                                        subroutine: "erase".into(),
                                        args: vec![],
                                    }),
                                    Statement::Let {
                                        lhs: "size".into(),
                                        index: None,
                                        rhs: Expr::Binary(
                                            BinaryOp::Subtract,
                                            Term::Var("size".into()),
                                            Box::new(Expr::Term(Term::IntConst(2)))
                                        ),
                                    },
                                    Statement::Do(SubroutineCall {
                                        receiver: None,
                                        subroutine: "draw".into(),
                                        args: vec![],
                                    }),
                                ],
                                else_body: None,
                            },
                            Statement::Return(None),
                        ],
                    },
                },
                Subroutine {
                    kind: SubroutineKind::Method,
                    typ: SubroutineType::Void,
                    name: "moveUp".into(),
                    params: vec![],
                    body: SubroutineBody {
                        vars: vec![],
                        statements: vec![
                            Statement::If {
                                condition: Expr::Binary(
                                    BinaryOp::GreaterThan,
                                    Term::Var("y".into()),
                                    Box::new(Expr::Term(Term::IntConst(1)))
                                ),
                                if_body: vec![
                                    Statement::Do(SubroutineCall {
                                        receiver: Some("Screen".into()),
                                        subroutine: "setColor".into(),
                                        args: vec![Expr::Term(Term::KeywordConst(
                                            KeywordConst::False
                                        ))],
                                    }),
                                    Statement::Do(SubroutineCall {
                                        receiver: Some("Screen".into()),
                                        subroutine: "drawRectangle".into(),
                                        args: vec![
                                            Expr::Term(Term::Var("x".into())),
                                            Expr::Binary(
                                                BinaryOp::Subtract,
                                                Term::Bracketed(Box::new(Expr::Binary(
                                                    BinaryOp::Add,
                                                    Term::Var("y".into()),
                                                    Box::new(Expr::Term(Term::Var("size".into())))
                                                ))),
                                                Box::new(Expr::Term(Term::IntConst(1)))
                                            ),
                                            Expr::Binary(
                                                BinaryOp::Add,
                                                Term::Var("x".into()),
                                                Box::new(Expr::Term(Term::Var("size".into())))
                                            ),
                                            Expr::Binary(
                                                BinaryOp::Add,
                                                Term::Var("y".into()),
                                                Box::new(Expr::Term(Term::Var("size".into())))
                                            ),
                                        ],
                                    }),
                                    Statement::Let {
                                        lhs: "y".into(),
                                        index: None,
                                        rhs: Expr::Binary(
                                            BinaryOp::Subtract,
                                            Term::Var("y".into()),
                                            Box::new(Expr::Term(Term::IntConst(2)))
                                        ),
                                    },
                                    Statement::Do(SubroutineCall {
                                        receiver: Some("Screen".into()),
                                        subroutine: "setColor".into(),
                                        args: vec![Expr::Term(Term::KeywordConst(
                                            KeywordConst::True
                                        ))],
                                    }),
                                    Statement::Do(SubroutineCall {
                                        receiver: Some("Screen".into()),
                                        subroutine: "drawRectangle".into(),
                                        args: vec![
                                            Expr::Term(Term::Var("x".into())),
                                            Expr::Term(Term::Var("y".into())),
                                            Expr::Binary(
                                                BinaryOp::Add,
                                                Term::Var("x".into()),
                                                Box::new(Expr::Term(Term::Var("size".into())))
                                            ),
                                            Expr::Binary(
                                                BinaryOp::Add,
                                                Term::Var("y".into()),
                                                Box::new(Expr::Term(Term::IntConst(1)))
                                            ),
                                        ],
                                    }),
                                ],
                                else_body: None,
                            },
                            Statement::Return(None),
                        ],
                    },
                },
                Subroutine {
                    kind: SubroutineKind::Method,
                    typ: SubroutineType::Void,
                    name: "moveDown".into(),
                    params: vec![],
                    body: SubroutineBody {
                        vars: vec![],
                        statements: vec![
                            Statement::If {
                                condition: Expr::Binary(
                                    BinaryOp::LessThan,
                                    Term::Bracketed(Box::new(Expr::Binary(
                                        BinaryOp::Add,
                                        Term::Var("y".into()),
                                        Box::new(Expr::Term(Term::Var("size".into())))
                                    ))),
                                    Box::new(Expr::Term(Term::IntConst(254)))
                                ),
                                if_body: vec![
                                    Statement::Do(SubroutineCall {
                                        receiver: Some("Screen".into()),
                                        subroutine: "setColor".into(),
                                        args: vec![Expr::Term(Term::KeywordConst(
                                            KeywordConst::False
                                        ))],
                                    }),
                                    Statement::Do(SubroutineCall {
                                        receiver: Some("Screen".into()),
                                        subroutine: "drawRectangle".into(),
                                        args: vec![
                                            Expr::Term(Term::Var("x".into())),
                                            Expr::Term(Term::Var("y".into())),
                                            Expr::Binary(
                                                BinaryOp::Add,
                                                Term::Var("x".into()),
                                                Box::new(Expr::Term(Term::Var("size".into())))
                                            ),
                                            Expr::Binary(
                                                BinaryOp::Add,
                                                Term::Var("y".into()),
                                                Box::new(Expr::Term(Term::IntConst(1)))
                                            ),
                                        ],
                                    }),
                                    Statement::Let {
                                        lhs: "y".into(),
                                        index: None,
                                        rhs: Expr::Binary(
                                            BinaryOp::Add,
                                            Term::Var("y".into()),
                                            Box::new(Expr::Term(Term::IntConst(2)))
                                        ),
                                    },
                                    Statement::Do(SubroutineCall {
                                        receiver: Some("Screen".into()),
                                        subroutine: "setColor".into(),
                                        args: vec![Expr::Term(Term::KeywordConst(
                                            KeywordConst::True
                                        ))],
                                    }),
                                    Statement::Do(SubroutineCall {
                                        receiver: Some("Screen".into()),
                                        subroutine: "drawRectangle".into(),
                                        args: vec![
                                            Expr::Term(Term::Var("x".into())),
                                            Expr::Binary(
                                                BinaryOp::Subtract,
                                                Term::Bracketed(Box::new(Expr::Binary(
                                                    BinaryOp::Add,
                                                    Term::Var("y".into()),
                                                    Box::new(Expr::Term(Term::Var("size".into())))
                                                ))),
                                                Box::new(Expr::Term(Term::IntConst(1)))
                                            ),
                                            Expr::Binary(
                                                BinaryOp::Add,
                                                Term::Var("x".into()),
                                                Box::new(Expr::Term(Term::Var("size".into())))
                                            ),
                                            Expr::Binary(
                                                BinaryOp::Add,
                                                Term::Var("y".into()),
                                                Box::new(Expr::Term(Term::Var("size".into())))
                                            ),
                                        ],
                                    }),
                                ],
                                else_body: None,
                            },
                            Statement::Return(None),
                        ],
                    },
                },
                Subroutine {
                    kind: SubroutineKind::Method,
                    typ: SubroutineType::Void,
                    name: "moveLeft".into(),
                    params: vec![],
                    body: SubroutineBody {
                        vars: vec![],
                        statements: vec![
                            Statement::If {
                                condition: Expr::Binary(
                                    BinaryOp::GreaterThan,
                                    Term::Var("x".into()),
                                    Box::new(Expr::Term(Term::IntConst(1)))
                                ),
                                if_body: vec![
                                    Statement::Do(SubroutineCall {
                                        receiver: Some("Screen".into()),
                                        subroutine: "setColor".into(),
                                        args: vec![Expr::Term(Term::KeywordConst(
                                            KeywordConst::False
                                        ))],
                                    }),
                                    Statement::Do(SubroutineCall {
                                        receiver: Some("Screen".into()),
                                        subroutine: "drawRectangle".into(),
                                        args: vec![
                                            Expr::Binary(
                                                BinaryOp::Subtract,
                                                Term::Bracketed(Box::new(Expr::Binary(
                                                    BinaryOp::Add,
                                                    Term::Var("x".into()),
                                                    Box::new(Expr::Term(Term::Var("size".into())))
                                                ))),
                                                Box::new(Expr::Term(Term::IntConst(1)))
                                            ),
                                            Expr::Term(Term::Var("y".into())),
                                            Expr::Binary(
                                                BinaryOp::Add,
                                                Term::Var("x".into()),
                                                Box::new(Expr::Term(Term::Var("size".into())))
                                            ),
                                            Expr::Binary(
                                                BinaryOp::Add,
                                                Term::Var("y".into()),
                                                Box::new(Expr::Term(Term::Var("size".into())))
                                            ),
                                        ],
                                    }),
                                    Statement::Let {
                                        lhs: "x".into(),
                                        index: None,
                                        rhs: Expr::Binary(
                                            BinaryOp::Subtract,
                                            Term::Var("x".into()),
                                            Box::new(Expr::Term(Term::IntConst(2)))
                                        ),
                                    },
                                    Statement::Do(SubroutineCall {
                                        receiver: Some("Screen".into()),
                                        subroutine: "setColor".into(),
                                        args: vec![Expr::Term(Term::KeywordConst(
                                            KeywordConst::True
                                        ))],
                                    }),
                                    Statement::Do(SubroutineCall {
                                        receiver: Some("Screen".into()),
                                        subroutine: "drawRectangle".into(),
                                        args: vec![
                                            Expr::Term(Term::Var("x".into())),
                                            Expr::Term(Term::Var("y".into())),
                                            Expr::Binary(
                                                BinaryOp::Add,
                                                Term::Var("x".into()),
                                                Box::new(Expr::Term(Term::IntConst(1)))
                                            ),
                                            Expr::Binary(
                                                BinaryOp::Add,
                                                Term::Var("y".into()),
                                                Box::new(Expr::Term(Term::Var("size".into())))
                                            ),
                                        ],
                                    }),
                                ],
                                else_body: None,
                            },
                            Statement::Return(None),
                        ],
                    },
                },
                Subroutine {
                    kind: SubroutineKind::Method,
                    typ: SubroutineType::Void,
                    name: "moveRight".into(),
                    params: vec![],
                    body: SubroutineBody {
                        vars: vec![],
                        statements: vec![
                            Statement::If {
                                condition: Expr::Binary(
                                    BinaryOp::LessThan,
                                    Term::Bracketed(Box::new(Expr::Binary(
                                        BinaryOp::Add,
                                        Term::Var("x".into()),
                                        Box::new(Expr::Term(Term::Var("size".into())))
                                    ))),
                                    Box::new(Expr::Term(Term::IntConst(510)))
                                ),
                                if_body: vec![
                                    Statement::Do(SubroutineCall {
                                        receiver: Some("Screen".into()),
                                        subroutine: "setColor".into(),
                                        args: vec![Expr::Term(Term::KeywordConst(
                                            KeywordConst::False
                                        ))],
                                    }),
                                    Statement::Do(SubroutineCall {
                                        receiver: Some("Screen".into()),
                                        subroutine: "drawRectangle".into(),
                                        args: vec![
                                            Expr::Term(Term::Var("x".into())),
                                            Expr::Term(Term::Var("y".into())),
                                            Expr::Binary(
                                                BinaryOp::Add,
                                                Term::Var("x".into()),
                                                Box::new(Expr::Term(Term::IntConst(1)))
                                            ),
                                            Expr::Binary(
                                                BinaryOp::Add,
                                                Term::Var("y".into()),
                                                Box::new(Expr::Term(Term::Var("size".into())))
                                            ),
                                        ],
                                    }),
                                    Statement::Let {
                                        lhs: "x".into(),
                                        index: None,
                                        rhs: Expr::Binary(
                                            BinaryOp::Add,
                                            Term::Var("x".into()),
                                            Box::new(Expr::Term(Term::IntConst(2)))
                                        ),
                                    },
                                    Statement::Do(SubroutineCall {
                                        receiver: Some("Screen".into()),
                                        subroutine: "setColor".into(),
                                        args: vec![Expr::Term(Term::KeywordConst(
                                            KeywordConst::True
                                        ))],
                                    }),
                                    Statement::Do(SubroutineCall {
                                        receiver: Some("Screen".into()),
                                        subroutine: "drawRectangle".into(),
                                        args: vec![
                                            Expr::Binary(
                                                BinaryOp::Subtract,
                                                Term::Bracketed(Box::new(Expr::Binary(
                                                    BinaryOp::Add,
                                                    Term::Var("x".into()),
                                                    Box::new(Expr::Term(Term::Var("size".into())))
                                                ))),
                                                Box::new(Expr::Term(Term::IntConst(1)))
                                            ),
                                            Expr::Term(Term::Var("y".into())),
                                            Expr::Binary(
                                                BinaryOp::Add,
                                                Term::Var("x".into()),
                                                Box::new(Expr::Term(Term::Var("size".into())))
                                            ),
                                            Expr::Binary(
                                                BinaryOp::Add,
                                                Term::Var("y".into()),
                                                Box::new(Expr::Term(Term::Var("size".into())))
                                            ),
                                        ],
                                    }),
                                ],
                                else_body: None,
                            },
                            Statement::Return(None),
                        ],
                    },
                },
            ],
        })
    );
}

#[test]
fn square_game_jack_test() {
    let source = read_test_file("SquareGame.jack");
    let tokenizer = Tokenizer::new(&source);
    let mut parser = Parser::new(tokenizer);

    assert_eq!(
        parser.parse(),
        Some(Class {
            name: "SquareGame".into(),
            vars: vec![
                ClassVars {
                    kind: ClassVarKind::Field,
                    typ: VarType::ClassName("Square".into()),
                    names: vec!["square".into()],
                },
                ClassVars {
                    kind: ClassVarKind::Field,
                    typ: VarType::Int,
                    names: vec!["direction".into()],
                }
            ],
            subs: vec![
                Subroutine {
                    kind: SubroutineKind::Constructor,
                    typ: SubroutineType::NonVoid(VarType::ClassName("SquareGame".into())),
                    name: "new".into(),
                    params: vec![],
                    body: SubroutineBody {
                        vars: vec![],
                        statements: vec![
                            Statement::Let {
                                lhs: "square".into(),
                                index: None,
                                rhs: Expr::Term(Term::SubroutineCall(SubroutineCall {
                                    receiver: Some("Square".into()),
                                    subroutine: "new".into(),
                                    args: vec![
                                        Expr::Term(Term::IntConst(0)),
                                        Expr::Term(Term::IntConst(0)),
                                        Expr::Term(Term::IntConst(30)),
                                    ]
                                }))
                            },
                            Statement::Let {
                                lhs: "direction".into(),
                                index: None,
                                rhs: Expr::Term(Term::IntConst(0)),
                            },
                            Statement::Return(Some(Expr::Term(Term::KeywordConst(
                                KeywordConst::This
                            ))))
                        ]
                    }
                },
                Subroutine {
                    kind: SubroutineKind::Method,
                    typ: SubroutineType::Void,
                    name: "dispose".into(),
                    params: vec![],
                    body: SubroutineBody {
                        vars: vec![],
                        statements: vec![
                            Statement::Do(SubroutineCall {
                                receiver: Some("square".into()),
                                subroutine: "dispose".into(),
                                args: vec![],
                            }),
                            Statement::Do(SubroutineCall {
                                receiver: Some("Memory".into()),
                                subroutine: "deAlloc".into(),
                                args: vec![Expr::Term(Term::KeywordConst(KeywordConst::This))]
                            }),
                            Statement::Return(None)
                        ]
                    }
                },
                Subroutine {
                    kind: SubroutineKind::Method,
                    typ: SubroutineType::Void,
                    name: "moveSquare".into(),
                    params: vec![],
                    body: SubroutineBody {
                        vars: vec![],
                        statements: vec![
                            Statement::If {
                                condition: Expr::Binary(
                                    BinaryOp::Equal,
                                    Term::Var("direction".into()),
                                    Box::new(Expr::Term(Term::IntConst(1)))
                                ),
                                if_body: vec![Statement::Do(SubroutineCall {
                                    receiver: Some("square".into()),
                                    subroutine: "moveUp".into(),
                                    args: vec![]
                                })],
                                else_body: None,
                            },
                            Statement::If {
                                condition: Expr::Binary(
                                    BinaryOp::Equal,
                                    Term::Var("direction".into()),
                                    Box::new(Expr::Term(Term::IntConst(2)))
                                ),
                                if_body: vec![Statement::Do(SubroutineCall {
                                    receiver: Some("square".into()),
                                    subroutine: "moveDown".into(),
                                    args: vec![]
                                })],
                                else_body: None,
                            },
                            Statement::If {
                                condition: Expr::Binary(
                                    BinaryOp::Equal,
                                    Term::Var("direction".into()),
                                    Box::new(Expr::Term(Term::IntConst(3)))
                                ),
                                if_body: vec![Statement::Do(SubroutineCall {
                                    receiver: Some("square".into()),
                                    subroutine: "moveLeft".into(),
                                    args: vec![]
                                })],
                                else_body: None,
                            },
                            Statement::If {
                                condition: Expr::Binary(
                                    BinaryOp::Equal,
                                    Term::Var("direction".into()),
                                    Box::new(Expr::Term(Term::IntConst(4)))
                                ),
                                if_body: vec![Statement::Do(SubroutineCall {
                                    receiver: Some("square".into()),
                                    subroutine: "moveRight".into(),
                                    args: vec![]
                                })],
                                else_body: None,
                            },
                            Statement::Do(SubroutineCall {
                                receiver: Some("Sys".into()),
                                subroutine: "wait".into(),
                                args: vec![Expr::Term(Term::IntConst(5)),]
                            }),
                            Statement::Return(None),
                        ]
                    }
                },
                Subroutine {
                    kind: SubroutineKind::Method,
                    typ: SubroutineType::Void,
                    name: "run".into(),
                    params: vec![],
                    body: SubroutineBody {
                        vars: vec![
                            LocalVars {
                                typ: VarType::Char,
                                names: vec!["key".into()],
                            },
                            LocalVars {
                                typ: VarType::Boolean,
                                names: vec!["exit".into()],
                            }
                        ],
                        statements: vec![
                            Statement::Let {
                                lhs: "exit".into(),
                                index: None,
                                rhs: Expr::Term(Term::KeywordConst(KeywordConst::False))
                            },
                            Statement::While {
                                condition: Expr::Term(Term::Unary(
                                    UnaryOp::Not,
                                    Box::new(Term::Var("exit".into()))
                                )),
                                body: vec![
                                    Statement::While {
                                        condition: Expr::Binary(
                                            BinaryOp::Equal,
                                            Term::Var("key".into()),
                                            Box::new(Expr::Term(Term::IntConst(0)))
                                        ),
                                        body: vec![
                                            Statement::Let {
                                                lhs: "key".into(),
                                                index: None,
                                                rhs: Expr::Term(Term::SubroutineCall(
                                                    SubroutineCall {
                                                        receiver: Some("Keyboard".into()),
                                                        subroutine: "keyPressed".into(),
                                                        args: vec![]
                                                    }
                                                ))
                                            },
                                            Statement::Do(SubroutineCall {
                                                receiver: None,
                                                subroutine: "moveSquare".into(),
                                                args: vec![]
                                            })
                                        ],
                                    },
                                    Statement::If {
                                        condition: Expr::Binary(
                                            BinaryOp::Equal,
                                            Term::Var("key".into()),
                                            Box::new(Expr::Term(Term::IntConst(81)))
                                        ),
                                        if_body: vec![Statement::Let {
                                            lhs: "exit".into(),
                                            index: None,
                                            rhs: Expr::Term(Term::KeywordConst(KeywordConst::True))
                                        }],
                                        else_body: None
                                    },
                                    Statement::If {
                                        condition: Expr::Binary(
                                            BinaryOp::Equal,
                                            Term::Var("key".into()),
                                            Box::new(Expr::Term(Term::IntConst(90)))
                                        ),
                                        if_body: vec![Statement::Do(SubroutineCall {
                                            receiver: Some("square".into()),
                                            subroutine: "decSize".into(),
                                            args: vec![]
                                        })],
                                        else_body: None
                                    },
                                    Statement::If {
                                        condition: Expr::Binary(
                                            BinaryOp::Equal,
                                            Term::Var("key".into()),
                                            Box::new(Expr::Term(Term::IntConst(88)))
                                        ),
                                        if_body: vec![Statement::Do(SubroutineCall {
                                            receiver: Some("square".into()),
                                            subroutine: "incSize".into(),
                                            args: vec![]
                                        })],
                                        else_body: None
                                    },
                                    Statement::If {
                                        condition: Expr::Binary(
                                            BinaryOp::Equal,
                                            Term::Var("key".into()),
                                            Box::new(Expr::Term(Term::IntConst(131)))
                                        ),
                                        if_body: vec![Statement::Let {
                                            lhs: "direction".into(),
                                            index: None,
                                            rhs: Expr::Term(Term::IntConst(1))
                                        }],
                                        else_body: None
                                    },
                                    Statement::If {
                                        condition: Expr::Binary(
                                            BinaryOp::Equal,
                                            Term::Var("key".into()),
                                            Box::new(Expr::Term(Term::IntConst(133)))
                                        ),
                                        if_body: vec![Statement::Let {
                                            lhs: "direction".into(),
                                            index: None,
                                            rhs: Expr::Term(Term::IntConst(2))
                                        }],
                                        else_body: None
                                    },
                                    Statement::If {
                                        condition: Expr::Binary(
                                            BinaryOp::Equal,
                                            Term::Var("key".into()),
                                            Box::new(Expr::Term(Term::IntConst(130)))
                                        ),
                                        if_body: vec![Statement::Let {
                                            lhs: "direction".into(),
                                            index: None,
                                            rhs: Expr::Term(Term::IntConst(3))
                                        }],
                                        else_body: None
                                    },
                                    Statement::If {
                                        condition: Expr::Binary(
                                            BinaryOp::Equal,
                                            Term::Var("key".into()),
                                            Box::new(Expr::Term(Term::IntConst(132)))
                                        ),
                                        if_body: vec![Statement::Let {
                                            lhs: "direction".into(),
                                            index: None,
                                            rhs: Expr::Term(Term::IntConst(4))
                                        }],
                                        else_body: None
                                    },
                                    Statement::While {
                                        condition: Expr::Term(Term::Unary(
                                            UnaryOp::Not,
                                            Box::new(Term::Bracketed(Box::new(Expr::Binary(
                                                BinaryOp::Equal,
                                                Term::Var("key".into()),
                                                Box::new(Expr::Term(Term::IntConst(0)))
                                            ))))
                                        )),
                                        body: vec![
                                            Statement::Let {
                                                lhs: "key".into(),
                                                index: None,
                                                rhs: Expr::Term(Term::SubroutineCall(
                                                    SubroutineCall {
                                                        receiver: Some("Keyboard".into()),
                                                        subroutine: "keyPressed".into(),
                                                        args: vec![]
                                                    }
                                                ))
                                            },
                                            Statement::Do(SubroutineCall {
                                                receiver: None,
                                                subroutine: "moveSquare".into(),
                                                args: vec![],
                                            }),
                                        ]
                                    },
                                ]
                            },
                            Statement::Return(None),
                        ]
                    }
                },
            ],
        })
    );
}
