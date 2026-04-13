use rustc_span::Symbol;

pub trait IntoSymbol {
    fn into_symbol(self) -> Symbol;
}

impl IntoSymbol for Symbol {
    fn into_symbol(self) -> Symbol {
        self
    }
}

impl IntoSymbol for &str {
    fn into_symbol(self) -> Symbol {
        Symbol::intern(self)
    }
}

impl IntoSymbol for String {
    fn into_symbol(self) -> Symbol {
        self.as_str().into_symbol()
    }
}

impl IntoSymbol for &String {
    fn into_symbol(self) -> Symbol {
        self.as_str().into_symbol()
    }
}
