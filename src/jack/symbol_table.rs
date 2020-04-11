use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Int,
    Char,
    Boolean,
    ClassName(String),
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Kind {
    Static,
    Field,
    Argument,
    LocalVar,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub typ: Type,
    pub kind: Kind,
    pub index: u16,
}

pub struct SymbolTable<'a> {
    symbols: HashMap<String, Symbol>,
    indices: HashMap<Kind, u16>,
    parent: Option<&'a mut SymbolTable<'a>>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            indices: HashMap::new(),
            parent: None,
        }
    }

    pub fn child(&'a mut self) -> SymbolTable {
        SymbolTable::new_with_parent(self)
    }

    fn new_with_parent(parent: &'a mut SymbolTable<'a>) -> Self {
        SymbolTable {
            parent: Some(parent),
            ..SymbolTable::new()
        }
    }

    pub fn define(&mut self, name: String, typ: Type, kind: Kind) {
        let index = self.next_index(kind);
        self.symbols.insert(
            name.clone(),
            Symbol {
                name,
                typ,
                kind,
                index,
            },
        );
    }

    fn next_index(&mut self, kind: Kind) -> u16 {
        let entry = self.indices.entry(kind).or_default();
        let index = *entry;
        *entry += 1;
        index
    }

    pub fn get<S: AsRef<str>>(&mut self, name: S) -> Option<&Symbol> {
        let sym = self.symbols.get(name.as_ref());

        if sym.is_some() {
            sym
        } else if let Some(p) = &mut self.parent {
            p.get(name)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_global_vars() {
        let mut symbols = SymbolTable::new();

        symbols.define(
            "square".into(),
            Type::ClassName("Square".into()),
            Kind::Field,
        );

        symbols.define("direction".into(), Type::Int, Kind::Field);

        symbols.define(
            "instance".into(),
            Type::ClassName("PongGame".into()),
            Kind::Static,
        );

        assert_eq!(
            symbols.get("square"),
            Some(&Symbol {
                name: "square".into(),
                typ: Type::ClassName("Square".into()),
                kind: Kind::Field,
                index: 0,
            })
        );

        assert_eq!(
            symbols.get("direction"),
            Some(&Symbol {
                name: "direction".into(),
                typ: Type::Int,
                kind: Kind::Field,
                index: 1,
            })
        );

        assert_eq!(
            symbols.get("instance"),
            Some(&Symbol {
                name: "instance".into(),
                typ: Type::ClassName("PongGame".into()),
                kind: Kind::Static,
                index: 0,
            })
        );

        assert_eq!(symbols.get("somethingElse"), None);
    }

    #[test]
    fn test_define_local_vars() {
        let mut globals = SymbolTable::new();
        let mut locals = globals.child();

        locals.define("Ax".into(), Type::Int, Kind::Argument);
        locals.define("Ay".into(), Type::Int, Kind::Argument);
        locals.define("Asize".into(), Type::Int, Kind::Argument);

        locals.define("a".into(), Type::ClassName("Array".into()), Kind::LocalVar);
        locals.define("length".into(), Type::Int, Kind::LocalVar);
        locals.define("i".into(), Type::Int, Kind::LocalVar);
        locals.define("sum".into(), Type::Int, Kind::LocalVar);

        assert_eq!(
            locals.get("Ax"),
            Some(&Symbol {
                name: "Ax".into(),
                typ: Type::Int,
                kind: Kind::Argument,
                index: 0,
            })
        );

        assert_eq!(
            locals.get("Ay"),
            Some(&Symbol {
                name: "Ay".into(),
                typ: Type::Int,
                kind: Kind::Argument,
                index: 1,
            })
        );

        assert_eq!(
            locals.get("Asize"),
            Some(&Symbol {
                name: "Asize".into(),
                typ: Type::Int,
                kind: Kind::Argument,
                index: 2,
            })
        );

        assert_eq!(
            locals.get("a"),
            Some(&Symbol {
                name: "a".into(),
                typ: Type::ClassName("Array".into()),
                kind: Kind::LocalVar,
                index: 0,
            })
        );

        assert_eq!(
            locals.get("length"),
            Some(&Symbol {
                name: "length".into(),
                typ: Type::Int,
                kind: Kind::LocalVar,
                index: 1,
            })
        );

        assert_eq!(
            locals.get("i"),
            Some(&Symbol {
                name: "i".into(),
                typ: Type::Int,
                kind: Kind::LocalVar,
                index: 2,
            })
        );

        assert_eq!(
            locals.get("sum"),
            Some(&Symbol {
                name: "sum".into(),
                typ: Type::Int,
                kind: Kind::LocalVar,
                index: 3,
            })
        );

        assert_eq!(locals.get("nope"), None);
    }

    #[test]
    fn test_nested_symbol_tables() {
        let mut globals = SymbolTable::new();

        globals.define("nAccounts".into(), Type::Int, Kind::Static);
        globals.define("id".into(), Type::Int, Kind::Field);
        globals.define("name".into(), Type::Int, Kind::Field);
        globals.define("balance".into(), Type::Int, Kind::Field);

        let mut locals = globals.child();

        locals.define("sum".into(), Type::Int, Kind::Argument);
        locals.define("status".into(), Type::Boolean, Kind::LocalVar);

        assert_eq!(
            locals.get("sum"),
            Some(&Symbol {
                name: "sum".into(),
                typ: Type::Int,
                kind: Kind::Argument,
                index: 0,
            })
        );

        assert_eq!(
            locals.get("status"),
            Some(&Symbol {
                name: "status".into(),
                typ: Type::Boolean,
                kind: Kind::LocalVar,
                index: 0,
            })
        );

        assert_eq!(
            locals.get("name"),
            Some(&Symbol {
                name: "name".into(),
                typ: Type::Int,
                kind: Kind::Field,
                index: 1,
            })
        );
    }
}
