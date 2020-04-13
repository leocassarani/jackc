use super::parser::VarType;
use std::collections::HashMap;
use std::convert::From;

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Int,
    Char,
    Boolean,
    ClassName(String),
}

impl From<&VarType> for Type {
    fn from(typ: &VarType) -> Self {
        match typ {
            VarType::Int => Type::Int,
            VarType::Char => Type::Char,
            VarType::Boolean => Type::Boolean,
            VarType::ClassName(s) => Type::ClassName(s.clone()),
        }
    }
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

pub struct SymbolTable {
    globals: HashMap<String, Symbol>,
    locals: HashMap<String, Symbol>,
    indices: HashMap<Kind, u16>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            globals: HashMap::new(),
            locals: HashMap::new(),
            indices: HashMap::new(),
        }
    }

    pub fn start_subroutine(&mut self) {
        self.locals.clear();
        self.indices.remove(&Kind::Argument);
        self.indices.remove(&Kind::LocalVar);
    }

    pub fn define(&mut self, name: String, typ: Type, kind: Kind) {
        let index = self.next_index(kind);

        let symbols = match kind {
            Kind::Static | Kind::Field => &mut self.globals,
            Kind::Argument | Kind::LocalVar => &mut self.locals,
        };

        symbols.insert(
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

    pub fn get<S: AsRef<str>>(&self, name: S) -> Option<&Symbol> {
        let key = name.as_ref();
        self.locals.get(key).or_else(|| self.globals.get(key))
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
        let mut symbols = SymbolTable::new();
        symbols.start_subroutine();

        symbols.define("Ax".into(), Type::Int, Kind::Argument);
        symbols.define("Ay".into(), Type::Int, Kind::Argument);
        symbols.define("Asize".into(), Type::Int, Kind::Argument);

        symbols.define("a".into(), Type::ClassName("Array".into()), Kind::LocalVar);
        symbols.define("length".into(), Type::Int, Kind::LocalVar);
        symbols.define("i".into(), Type::Int, Kind::LocalVar);
        symbols.define("sum".into(), Type::Int, Kind::LocalVar);

        assert_eq!(
            symbols.get("Ax"),
            Some(&Symbol {
                name: "Ax".into(),
                typ: Type::Int,
                kind: Kind::Argument,
                index: 0,
            })
        );

        assert_eq!(
            symbols.get("Ay"),
            Some(&Symbol {
                name: "Ay".into(),
                typ: Type::Int,
                kind: Kind::Argument,
                index: 1,
            })
        );

        assert_eq!(
            symbols.get("Asize"),
            Some(&Symbol {
                name: "Asize".into(),
                typ: Type::Int,
                kind: Kind::Argument,
                index: 2,
            })
        );

        assert_eq!(
            symbols.get("a"),
            Some(&Symbol {
                name: "a".into(),
                typ: Type::ClassName("Array".into()),
                kind: Kind::LocalVar,
                index: 0,
            })
        );

        assert_eq!(
            symbols.get("length"),
            Some(&Symbol {
                name: "length".into(),
                typ: Type::Int,
                kind: Kind::LocalVar,
                index: 1,
            })
        );

        assert_eq!(
            symbols.get("i"),
            Some(&Symbol {
                name: "i".into(),
                typ: Type::Int,
                kind: Kind::LocalVar,
                index: 2,
            })
        );

        assert_eq!(
            symbols.get("sum"),
            Some(&Symbol {
                name: "sum".into(),
                typ: Type::Int,
                kind: Kind::LocalVar,
                index: 3,
            })
        );

        assert_eq!(symbols.get("nope"), None);
    }

    #[test]
    fn test_nested_symbol_tables() {
        let mut symbols = SymbolTable::new();

        symbols.define("nAccounts".into(), Type::Int, Kind::Static);
        symbols.define("id".into(), Type::Int, Kind::Field);
        symbols.define("name".into(), Type::Int, Kind::Field);
        symbols.define("balance".into(), Type::Int, Kind::Field);

        symbols.start_subroutine();

        symbols.define("sum".into(), Type::Int, Kind::Argument);
        symbols.define("status".into(), Type::Boolean, Kind::LocalVar);

        assert_eq!(
            symbols.get("sum"),
            Some(&Symbol {
                name: "sum".into(),
                typ: Type::Int,
                kind: Kind::Argument,
                index: 0,
            })
        );

        assert_eq!(
            symbols.get("status"),
            Some(&Symbol {
                name: "status".into(),
                typ: Type::Boolean,
                kind: Kind::LocalVar,
                index: 0,
            })
        );

        assert_eq!(
            symbols.get("name"),
            Some(&Symbol {
                name: "name".into(),
                typ: Type::Int,
                kind: Kind::Field,
                index: 1,
            })
        );

        symbols.start_subroutine();

        assert_eq!(symbols.get("sum"), None);
        assert_eq!(symbols.get("status"), None);

        assert_eq!(
            symbols.get("name"),
            Some(&Symbol {
                name: "name".into(),
                typ: Type::Int,
                kind: Kind::Field,
                index: 1,
            })
        );

        symbols.define("x".into(), Type::Int, Kind::Argument);

        symbols.define(
            "transactions".into(),
            Type::ClassName("Array".into()),
            Kind::LocalVar,
        );

        assert_eq!(
            symbols.get("x"),
            Some(&Symbol {
                name: "x".into(),
                typ: Type::Int,
                kind: Kind::Argument,
                index: 0,
            })
        );

        assert_eq!(
            symbols.get("transactions"),
            Some(&Symbol {
                name: "transactions".into(),
                typ: Type::ClassName("Array".into()),
                kind: Kind::LocalVar,
                index: 0,
            })
        );
    }
}
