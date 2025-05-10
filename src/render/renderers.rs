use crate::{Exp, Preset, Render, Typ, Var};
use derive_more::From;

macro_rules! renderer {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident($inner:ty)),+$(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $($variant($inner)),+
        }

        impl Render for Renderer {
            fn render_var(&self, var: &Var) -> Option<String> {
                match self {
                    $(Self::$variant(inner) => inner.render_var(var)),+
                }
            }

            fn render_typ(&self, typ: &Typ) -> Option<String> {
                match self {
                    $(Self::$variant(inner) => inner.render_typ(typ)),+
                }
            }

            fn render_exp(&self, exp: &Exp) -> Option<String> {
                match self {
                    $(Self::$variant(inner) => inner.render_exp(exp)),+
                }
            }
        }
    };
}

renderer!(
    #[derive(From, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
    pub enum Renderer {
        Base(PlainRenderer),
        English(EnglishRenderer),
    }
);

impl Renderer {}

impl From<Preset> for Renderer {
    fn from(discriminant: Preset) -> Self {
        use Preset::*;
        match discriminant {
            EnglishShort => EnglishRenderer::short().into(),
            EnglishLong => EnglishRenderer::long().into(),
            PlainDefault => PlainRenderer::default().into(),
            PlainIdea => PlainRenderer::idea().into(),
        }
    }
}

mod plain_renderer;

pub use plain_renderer::*;

mod english_renderer;

pub use english_renderer::*;
