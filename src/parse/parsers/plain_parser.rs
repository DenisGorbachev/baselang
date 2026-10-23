use crate::{Exp, Parse, Rewrite, Scope, ScopeApplyError, ScopeConvertibleError, ScopeDefineError, ScopeInsertRewriteError, ScopeNewSymError, ScopeNormalizeTypError, Sym, SymRc, Theory, TheoryNewError, Typ, is_identifier};
use derive_new::new;
use errgonomic::{handle, handle_bool, handle_opt};
use non_empty_str::cow::NonEmptyCowStr;
use std::borrow::Cow;
use thiserror::Error;

#[derive(new, Eq, PartialEq, Hash, Clone, Debug)]
pub struct PlainParser {
    /// The name of the [`Typ::Top`]
    #[new(into)]
    pub top: NonEmptyCowStr<'static>,
    /// Prefix for single-line comments.
    #[new(into)]
    pub single_line_comment: NonEmptyCowStr<'static>,
    /// Start delimiter for multi-line comments.
    #[new(into)]
    pub multi_line_comment_start: NonEmptyCowStr<'static>,
    /// End delimiter for multi-line comments.
    #[new(into)]
    pub multi_line_comment_end: NonEmptyCowStr<'static>,
}

impl Parse for PlainParser {
    type Error = PlainParserParseError;

    fn parse(&mut self, input: &str) -> Result<Theory, Self::Error> {
        use PlainParserParseError::*;
        let scope = handle!(self.parse_scope(input), ParseScopeFailed);
        Ok(handle!(Theory::new(scope), TheoryNewFailed))
    }
}

impl PlainParser {
    pub fn parse_scope(&self, input: &str) -> Result<Scope, PlainParserParseScopeError> {
        use PlainParserParseScopeError::*;
        let mut scope = Scope::default();
        let mut comment_lines = Vec::new();
        handle!(input.lines().enumerate().try_for_each(|(index, raw)| {
            use PlainParserParseScopeLineError::*;
            let line_number = handle_opt!(index.checked_add(1), LineNumberInvalid);
            let line = handle!(strip_comments_from_line(raw, self.single_line_comment.as_ref().as_ref(), self.multi_line_comment_start.as_ref().as_ref(), self.multi_line_comment_end.as_ref().as_ref(), &mut comment_lines, line_number), StripCommentsFromLineFailed, line_number, line: raw);
            if let Some(line) = normalize_line(&line) {
                handle!(parse_line(line, &mut scope, self.top.as_ref().as_ref()), ParseLineFailed, line_number, line);
            }
            Ok(())
        }), ParseScopeLineFailed, input: input);
        if let Some(&line_number) = comment_lines.first() {
            return Err(CommentUnclosedInvalid {
                line_number,
                input: input.to_owned(),
            });
        }
        Ok(scope)
    }

    pub fn expression(&self, input: &str, scope: &Scope) -> Result<Exp, PlainParserExpressionError> {
        use PlainParserExpressionError::*;
        Ok(handle!(parse_exp(input, scope, &[], self.top.as_ref().as_ref()), ParseExpFailed, input: input))
    }
}

impl Default for PlainParser {
    fn default() -> Self {
        Self::new(non_empty_str::non_empty_str!("Type"), non_empty_str::non_empty_str!("//"), non_empty_str::non_empty_str!("/*"), non_empty_str::non_empty_str!("*/"))
    }
}

fn normalize_line(line: &str) -> Option<&str> {
    let line = line.trim();
    if line.is_empty() || line == "{" || line == "}" {
        return None;
    }
    let line = line.strip_suffix('{').map_or(line, str::trim_end);
    if line.is_empty() { None } else { Some(line) }
}

fn strip_comments_from_line<'a>(line: &'a str, single_line_comment: &str, multi_line_comment_start: &str, multi_line_comment_end: &str, comment_lines: &mut Vec<usize>, line_number: usize) -> Result<Cow<'a, str>, PlainParserParseError> {
    use PlainParserParseError::*;
    let mut remaining = line;
    let mut output = (!comment_lines.is_empty()).then(String::new);
    let mut quoted = false;
    let mut escaped = false;
    while !remaining.is_empty() {
        if !comment_lines.is_empty() {
            if let Some(suffix) = remaining.strip_prefix(multi_line_comment_end) {
                comment_lines.pop();
                remaining = suffix;
                continue;
            }
            if let Some(suffix) = remaining.strip_prefix(multi_line_comment_start) {
                comment_lines.push(line_number);
                remaining = suffix;
                continue;
            }
        } else if !quoted {
            let single = remaining.starts_with(single_line_comment);
            let multi = remaining.starts_with(multi_line_comment_start);
            if single || multi {
                if output.is_none() {
                    let length = handle_opt!(line.len().checked_sub(remaining.len()), TextSliceFailed, input: line);
                    output = Some(handle_opt!(line.get(..length), TextSliceFailed, input: line).to_owned());
                }
                if single && (!multi || single_line_comment.len() >= multi_line_comment_start.len()) {
                    return Ok(Cow::Owned(output.unwrap_or_default()));
                }
                if let Some(output) = output.as_mut() {
                    output.push(' ');
                }
                comment_lines.push(line_number);
                remaining = handle_opt!(remaining.strip_prefix(multi_line_comment_start), TextSliceFailed, input: line);
                continue;
            }
        }
        let ch = handle_opt!(remaining.chars().next(), TextSliceFailed, input: line);
        if comment_lines.is_empty() {
            if let Some(output) = output.as_mut() {
                output.push(ch);
            }
            if escaped {
                escaped = false;
            } else if quoted && ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                quoted = !quoted;
            }
        }
        remaining = handle_opt!(remaining.get(ch.len_utf8()..), TextSliceFailed, input: line);
    }
    Ok(output.map_or(Cow::Borrowed(line), Cow::Owned))
}

fn parse_line(line: &str, scope: &mut Scope, top: &str) -> Result<SymRc, PlainParserParseError> {
    use PlainParserParseError::*;
    if let Some((lhs, rhs)) = line.split_once("=>") {
        return Ok(handle!(parse_rewrite(lhs.trim(), rhs.trim(), scope, top), ParseRewriteFailed));
    }
    handle_bool!(line.contains('='), DefinitionOperatorInvalid, input: line);
    let colon_index = match find_first_top_level_colon(line) {
        Ok(Some(colon_index)) => colon_index,
        Ok(None) => {
            return Err(MissingTypeSeparator {
                input: line.to_string(),
            });
        }
        Err(source) => {
            return Err(ScanTextFailed {
                source: Box::new(source),
                input: line.to_string(),
            });
        }
    };
    let name = match line.get(..colon_index) {
        Some(name) => name.trim(),
        None => {
            return Err(TextSliceFailed {
                input: line.to_string(),
            });
        }
    };
    let typ = match line.get(colon_index..) {
        Some(suffix) => match suffix.strip_prefix(':') {
            Some(typ) => typ.trim(),
            None => {
                return Err(TextSliceFailed {
                    input: line.to_string(),
                });
            }
        },
        None => {
            return Err(TextSliceFailed {
                input: line.to_string(),
            });
        }
    };
    if !is_identifier(name) {
        return Err(InvalidDefinitionName {
            input: name.to_string(),
        });
    }
    let typ = match parse_typ(typ, scope, &[], top) {
        Ok(typ) => typ,
        Err(source) => {
            return Err(ParseTypeFailed {
                source: Box::new(source),
                input: typ.to_string(),
            });
        }
    };
    let sym = handle!(scope.new_sym(name, typ), NewSymFailed, input: line);
    Ok(sym)
}

fn parse_typ(input: &str, scope: &Scope, locals: &[SymRc], top: &str) -> Result<Typ, PlainParserParseError> {
    use PlainParserParseError::*;
    let input = input.trim();
    if input.is_empty() {
        return Err(EmptyType {
            input: input.to_string(),
        });
    }
    if let Some(inner) = match strip_outer_parens(input) {
        Ok(inner) => inner,
        Err(source) => {
            return Err(ScanTextFailed {
                source: Box::new(source),
                input: input.to_string(),
            });
        }
    } {
        let inner_has_arrow = match find_first_top_level_arrow(inner) {
            Ok(found) => found,
            Err(source) => {
                return Err(ScanTextFailed {
                    source: Box::new(source),
                    input: inner.to_string(),
                });
            }
        };
        let inner_has_colon = match find_first_top_level_colon(inner) {
            Ok(found) => found,
            Err(source) => {
                return Err(ScanTextFailed {
                    source: Box::new(source),
                    input: inner.to_string(),
                });
            }
        };
        if inner_has_arrow.is_some() || inner_has_colon.is_none() {
            return parse_typ(inner, scope, locals, top);
        }
        return Err(StandaloneBinderTypeInvalid {
            input: input.to_string(),
        });
    }

    let arrow_index = match find_first_top_level_arrow(input) {
        Ok(arrow_index) => arrow_index,
        Err(source) => {
            return Err(ScanTextFailed {
                source: Box::new(source),
                input: input.to_string(),
            });
        }
    };
    if let Some(arrow_index) = arrow_index {
        let domain = match input.get(..arrow_index) {
            Some(domain) => domain.trim(),
            None => {
                return Err(TextSliceFailed {
                    input: input.to_string(),
                });
            }
        };
        let codomain = match input.get(arrow_index..) {
            Some(suffix) => match suffix.strip_prefix("->") {
                Some(codomain) => codomain.trim(),
                None => {
                    return Err(TextSliceFailed {
                        input: input.to_string(),
                    });
                }
            },
            None => {
                return Err(TextSliceFailed {
                    input: input.to_string(),
                });
            }
        };
        let wrapped_domain = match strip_outer_parens(domain) {
            Ok(wrapped_domain) => wrapped_domain,
            Err(source) => {
                return Err(ScanTextFailed {
                    source: Box::new(source),
                    input: domain.to_string(),
                });
            }
        };
        if let Some(domain_inner) = wrapped_domain {
            let binder_colon_index = match find_first_top_level_colon(domain_inner) {
                Ok(binder_colon_index) => binder_colon_index,
                Err(source) => {
                    return Err(ScanTextFailed {
                        source: Box::new(source),
                        input: domain_inner.to_string(),
                    });
                }
            };
            if let Some(binder_colon_index) = binder_colon_index {
                let names_text = match domain_inner.get(..binder_colon_index) {
                    Some(names_text) => names_text.trim(),
                    None => {
                        return Err(TextSliceFailed {
                            input: domain_inner.to_string(),
                        });
                    }
                };
                let binder_typ_text = match domain_inner.get(binder_colon_index..) {
                    Some(suffix) => match suffix.strip_prefix(':') {
                        Some(binder_typ_text) => binder_typ_text.trim(),
                        None => {
                            return Err(TextSliceFailed {
                                input: domain_inner.to_string(),
                            });
                        }
                    },
                    None => {
                        return Err(TextSliceFailed {
                            input: domain_inner.to_string(),
                        });
                    }
                };
                if names_text.is_empty() {
                    return Err(InvalidBinderName {
                        input: names_text.to_string(),
                    });
                }
                let names = names_text.split_whitespace().collect::<Vec<_>>();
                if names.is_empty() {
                    return Err(InvalidBinderName {
                        input: names_text.to_string(),
                    });
                }
                let binder_typ = match parse_typ(binder_typ_text, scope, locals, top) {
                    Ok(binder_typ) => binder_typ,
                    Err(source) => {
                        return Err(ParseBinderTypeFailed {
                            source: Box::new(source),
                            input: binder_typ_text.to_string(),
                        });
                    }
                };
                let binders = names
                    .into_iter()
                    .map(|name| Sym::new_rc(name.to_string(), binder_typ.clone()))
                    .collect::<Vec<_>>();
                let mut inner_locals = locals.to_vec();
                inner_locals.extend(binders.iter().cloned());
                let codomain = match parse_typ(codomain, scope, &inner_locals, top) {
                    Ok(codomain) => codomain,
                    Err(source) => {
                        return Err(ParseCodomainFailed {
                            source: Box::new(source),
                            input: codomain.to_string(),
                        });
                    }
                };
                return Ok(binders
                    .into_iter()
                    .rev()
                    .fold(codomain, |acc, binder| Typ::fun(&binder, acc)));
            }
        }
        let domain_typ = match parse_typ(wrapped_domain.unwrap_or(domain), scope, locals, top) {
            Ok(domain_typ) => domain_typ,
            Err(source) => {
                return Err(ParseDomainTypeFailed {
                    source: Box::new(source),
                    input: domain.to_string(),
                });
            }
        };
        let codomain = match parse_typ(codomain, scope, locals, top) {
            Ok(codomain) => codomain,
            Err(source) => {
                return Err(ParseCodomainFailed {
                    source: Box::new(source),
                    input: codomain.to_string(),
                });
            }
        };
        let binder = Sym::new_rc("_", domain_typ);
        return Ok(Typ::fun(&binder, codomain));
    }

    if (input == top || input == "Type") && find_sym(scope, locals, input).is_none() {
        return Ok(Typ::top());
    }
    let exp = match parse_exp(input, scope, locals, top) {
        Ok(exp) => exp,
        Err(source) => {
            return Err(ParseExpressionFailed {
                source: Box::new(source),
                input: input.to_string(),
            });
        }
    };
    Ok(Typ::one(exp))
}

fn parse_exp(input: &str, scope: &Scope, locals: &[SymRc], top: &str) -> Result<Exp, PlainParserParseError> {
    use PlainParserParseError::*;
    let input = input.trim();
    if input.is_empty() {
        return Err(EmptyExpression {
            input: input.to_string(),
        });
    }
    if let Some(inner) = match strip_outer_parens(input) {
        Ok(inner) => inner,
        Err(source) => {
            return Err(ScanTextFailed {
                source: Box::new(source),
                input: input.to_string(),
            });
        }
    } {
        return parse_exp(inner, scope, locals, top);
    }
    let terms = match split_application_terms(input) {
        Ok(terms) => terms,
        Err(source) => {
            return Err(ScanTextFailed {
                source: Box::new(source),
                input: input.to_string(),
            });
        }
    };
    let mut terms = terms.into_iter();
    let first = match terms.next() {
        Some(first) => first,
        None => {
            return Err(EmptyExpression {
                input: input.to_string(),
            });
        }
    };
    let first = match parse_exp_atom(first, scope, locals, top) {
        Ok(first) => first,
        Err(source) => {
            return Err(ParseExpressionAtomFailed {
                source: Box::new(source),
                input: first.to_string(),
            });
        }
    };
    terms.try_fold(first, |fun, arg| {
        let arg = match parse_exp_atom(arg, scope, locals, top) {
            Ok(arg) => arg,
            Err(source) => {
                return Err(ParseExpressionAtomFailed {
                    source: Box::new(source),
                    input: arg.to_string(),
                });
            }
        };
        let exp = handle!(scope.apply(fun, arg), InvalidApplicationFailed, input);
        Ok(exp)
    })
}

fn parse_exp_atom(input: &str, scope: &Scope, locals: &[SymRc], top: &str) -> Result<Exp, PlainParserParseError> {
    use PlainParserParseError::*;
    let input = input.trim();
    if let Some(inner) = match strip_outer_parens(input) {
        Ok(inner) => inner,
        Err(source) => {
            return Err(ScanTextFailed {
                source: Box::new(source),
                input: input.to_string(),
            });
        }
    } {
        return parse_exp(inner, scope, locals, top);
    }
    if (input == top || input == "Type") && find_sym(scope, locals, input).is_none() {
        return Err(TopTypeUsedAsExpression {
            input: input.to_string(),
        });
    }
    match find_sym(scope, locals, input) {
        Some(sym) => Ok(Exp::sol(&sym)),
        None => Err(UnknownIdentifier {
            name: input.to_string(),
            input: input.to_string(),
        }),
    }
}

fn find_sym(scope: &Scope, locals: &[SymRc], name: &str) -> Option<SymRc> {
    if name == "_" {
        return None;
    }
    locals
        .iter()
        .rev()
        .find(|sym| sym.nym().identifier() == name)
        .or_else(|| scope.sym(name))
        .cloned()
}

fn strip_outer_parens(input: &str) -> Result<Option<&str>, PlainParserParseError> {
    use PlainParserParseError::*;
    if !input.starts_with('(') {
        return Ok(None);
    }
    let mut depth = 0usize;
    for (index, ch) in input.char_indices() {
        match ch {
            '(' => {
                depth = match depth.checked_add(1) {
                    Some(depth) => depth,
                    None => {
                        return Err(ParenthesisDepthOverflowFailed {
                            input: input.to_string(),
                        });
                    }
                };
            }
            ')' => {
                depth = match depth.checked_sub(1) {
                    Some(depth) => depth,
                    None => {
                        return Err(UnmatchedClosingParenthesis {
                            input: input.to_string(),
                        });
                    }
                };
                if depth == 0 {
                    if input.get(index..).is_some_and(|suffix| suffix == ")") {
                        return Ok(input.get(1..index));
                    }
                    return Ok(None);
                }
            }
            _ => {}
        }
    }
    Err(UnclosedParenthesis {
        input: input.to_string(),
    })
}

fn find_first_top_level_colon(input: &str) -> Result<Option<usize>, PlainParserParseError> {
    find_first_top_level_char(input, ':')
}

fn find_first_top_level_char(input: &str, target: char) -> Result<Option<usize>, PlainParserParseError> {
    use PlainParserParseError::*;
    let mut depth = 0usize;
    for (index, ch) in input.char_indices() {
        match ch {
            '(' => {
                depth = match depth.checked_add(1) {
                    Some(depth) => depth,
                    None => {
                        return Err(ParenthesisDepthOverflowFailed {
                            input: input.to_string(),
                        });
                    }
                };
            }
            ')' => {
                depth = match depth.checked_sub(1) {
                    Some(depth) => depth,
                    None => {
                        return Err(UnmatchedClosingParenthesis {
                            input: input.to_string(),
                        });
                    }
                };
            }
            _ if ch == target && depth == 0 => return Ok(Some(index)),
            _ => {}
        }
    }
    if depth == 0 {
        Ok(None)
    } else {
        Err(UnclosedParenthesis {
            input: input.to_string(),
        })
    }
}

fn find_first_top_level_arrow(input: &str) -> Result<Option<usize>, PlainParserParseError> {
    use PlainParserParseError::*;
    let mut depth = 0usize;
    for (index, ch) in input.char_indices() {
        match ch {
            '(' => {
                depth = match depth.checked_add(1) {
                    Some(depth) => depth,
                    None => {
                        return Err(ParenthesisDepthOverflowFailed {
                            input: input.to_string(),
                        });
                    }
                };
            }
            ')' => {
                depth = match depth.checked_sub(1) {
                    Some(depth) => depth,
                    None => {
                        return Err(UnmatchedClosingParenthesis {
                            input: input.to_string(),
                        });
                    }
                };
            }
            '-' if depth == 0
                && input
                    .get(index..)
                    .is_some_and(|suffix| suffix.starts_with("->")) =>
            {
                return Ok(Some(index));
            }
            _ => {}
        }
    }
    if depth == 0 {
        Ok(None)
    } else {
        Err(UnclosedParenthesis {
            input: input.to_string(),
        })
    }
}

fn split_application_terms(input: &str) -> Result<Vec<&str>, PlainParserParseError> {
    use PlainParserParseError::*;
    let mut depth = 0usize;
    let mut start = None;
    let mut terms = Vec::new();
    for (index, ch) in input.char_indices() {
        match ch {
            '(' => {
                if start.is_none() {
                    start = Some(index);
                }
                depth = match depth.checked_add(1) {
                    Some(depth) => depth,
                    None => {
                        return Err(ParenthesisDepthOverflowFailed {
                            input: input.to_string(),
                        });
                    }
                };
            }
            ')' => {
                depth = match depth.checked_sub(1) {
                    Some(depth) => depth,
                    None => {
                        return Err(UnmatchedClosingParenthesis {
                            input: input.to_string(),
                        });
                    }
                };
            }
            _ if ch.is_whitespace() && depth == 0 => {
                if let Some(start_index) = start {
                    let term = match input.get(start_index..index) {
                        Some(term) => term,
                        None => {
                            return Err(TextSliceFailed {
                                input: input.to_string(),
                            });
                        }
                    };
                    terms.push(term);
                    start = None;
                }
            }
            _ => {
                if start.is_none() {
                    start = Some(index);
                }
            }
        }
    }
    if depth != 0 {
        return Err(UnclosedParenthesis {
            input: input.to_string(),
        });
    }
    if let Some(start_index) = start {
        let term = match input.get(start_index..) {
            Some(term) => term,
            None => {
                return Err(TextSliceFailed {
                    input: input.to_string(),
                });
            }
        };
        terms.push(term);
    }
    Ok(terms)
}

fn parse_rewrite(lhs: &str, rhs: &str, scope: &mut Scope, top: &str) -> Result<SymRc, ParseRewriteError> {
    use ParseRewriteError::*;
    let terms = handle!(split_application_terms(lhs), SplitApplicationTermsFailed, lhs: lhs, rhs: rhs);
    let mut terms = terms.into_iter();
    let name = handle_opt!(terms.next(), HeadNotFound, lhs: lhs, rhs: rhs);
    handle_bool!(!is_identifier(name), NameInvalid, name: name);
    let terms = terms.collect::<Vec<_>>();
    if terms.is_empty() && scope.sym(name).is_none() {
        let body = handle!(parse_exp(rhs, scope, &[], top), ParseExpFailed, rhs: rhs);
        return Ok(handle!(scope.define(name, body), DefineFailed));
    }
    let sym = handle_opt!(scope.sym(name).cloned(), SymbolNotFound, name: name);
    let mut locals = Vec::new();
    let (_, patterns) = handle!(
        terms
            .into_iter()
            .try_fold((Exp::sol(&sym), Vec::new()), |(fun, mut patterns), input| {
                use ParseRewritePatternError::*;
                let typ = handle!(scope.normalize_typ(fun.typ()), NormalizeTypFailed);
                let Typ::Fun(param, _) = typ else {
                    return Err(ArityInvalid {
                        input: input.to_owned(),
                    });
                };
                let pattern = handle!(parse_pattern(input, param.typ(), scope, &mut locals, top), ParsePatternFailed);
                let fun = handle!(scope.apply(fun, &pattern), ApplyFailed);
                patterns.push(pattern);
                Ok((fun, patterns))
            }),
        ParseRewritePatternFailed
    );
    let body = handle!(parse_exp(rhs, scope, &locals, top), ParseExpFailed, rhs: rhs);
    let head = sym.clone();
    handle!(scope.insert_rewrite(head, Rewrite::new(patterns, body)), InsertRewriteFailed);
    Ok(sym)
}

fn parse_pattern(input: &str, expected: &Typ, scope: &Scope, locals: &mut Vec<SymRc>, top: &str) -> Result<Exp, ParsePatternError> {
    use ParsePatternError::*;
    let input = input.trim();
    if let Some(inner) = handle!(strip_outer_parens(input), StripOuterParensFailed, input: input) {
        return Ok(handle!(parse_pattern(inner, expected, scope, locals, top), ParsePatternInnerFailed));
    }
    let terms = handle!(split_application_terms(input), SplitApplicationTermsFailed, input: input);
    let (first, rest) = handle_opt!(terms.split_first(), PatternEmptyInvalid, input: input);
    let first = if let Some(sym) = find_sym(scope, locals, first) {
        Exp::sol(&sym)
    } else {
        handle_bool!(!rest.is_empty() || !is_identifier(first) || *first == top || *first == "Type", VariableTypeInvalid, input: input);
        let sym = Sym::new_rc(*first, expected.clone());
        if *first != "_" {
            locals.push(sym.clone());
        }
        Exp::Sol(sym)
    };
    let pattern = handle!(
        rest.iter().try_fold(first, |fun, input| {
            use ParsePatternArgumentError::*;
            let typ = handle!(scope.normalize_typ(fun.typ()), NormalizeTypFailed);
            let Typ::Fun(param, _) = typ else {
                return Err(ArityInvalid {
                    input: (*input).to_owned(),
                });
            };
            let arg = handle!(parse_pattern(input, param.typ(), scope, locals, top), ParsePatternFailed);
            Ok(handle!(scope.apply(fun, arg), ApplyFailed))
        }),
        ParsePatternArgumentFailed
    );
    let valid = handle!(scope.convertible(pattern.typ(), expected), ConvertibleFailed);
    handle_bool!(!valid, TypeInvalid, input: input);
    Ok(pattern)
}

#[derive(Error, Debug)]
pub enum PlainParserParseScopeError {
    #[error("failed to parse the source")]
    ParseScopeLineFailed { source: Box<PlainParserParseScopeLineError>, input: String },
    #[error("unclosed block comment opened on line {line_number}")]
    CommentUnclosedInvalid { line_number: usize, input: String },
}

#[derive(Error, Debug)]
pub enum PlainParserParseScopeLineError {
    #[error("source line number exceeds addressable memory")]
    LineNumberInvalid {},
    #[error("failed to strip comments on line {line_number}: '{line}'")]
    StripCommentsFromLineFailed { source: PlainParserParseError, line_number: usize, line: String },
    #[error("failed to parse line {line_number}: '{line}'")]
    ParseLineFailed { source: PlainParserParseError, line_number: usize, line: String },
}

#[derive(Error, Debug)]
pub enum PlainParserExpressionError {
    #[error("failed to parse expression '{input}'")]
    ParseExpFailed { source: PlainParserParseError, input: String },
}

#[derive(Error, Debug)]
pub enum ParseRewriteError {
    #[error("failed to split rule head '{lhs}'")]
    SplitApplicationTermsFailed { source: Box<PlainParserParseError>, lhs: String, rhs: String },
    #[error("rule has no head")]
    HeadNotFound { lhs: String, rhs: String },
    #[error("invalid rule head '{name}'")]
    NameInvalid { name: String },
    #[error("symbol '{name}' must be declared before its argument rules")]
    SymbolNotFound { name: String },
    #[error("failed to parse rule body '{rhs}'")]
    ParseExpFailed { source: Box<PlainParserParseError>, rhs: String },
    #[error("failed to introduce the definition")]
    DefineFailed { source: ScopeDefineError },
    #[error("failed to parse rule arguments")]
    ParseRewritePatternFailed { source: ParseRewritePatternError },
    #[error("failed to insert the rewrite rule")]
    InsertRewriteFailed { source: ScopeInsertRewriteError },
}

#[derive(Error, Debug)]
pub enum ParseRewritePatternError {
    #[error("failed to normalize the rule head's parameter type")]
    NormalizeTypFailed { source: ScopeNormalizeTypError },
    #[error("too many patterns before '{input}'")]
    ArityInvalid { input: String },
    #[error("failed to parse an argument pattern")]
    ParsePatternFailed { source: ParsePatternError },
    #[error("failed to apply the head to its pattern")]
    ApplyFailed { source: ScopeApplyError },
}

#[derive(Error, Debug)]
pub enum ParsePatternError {
    #[error("invalid parentheses in '{input}'")]
    StripOuterParensFailed { source: Box<PlainParserParseError>, input: String },
    #[error("failed to parse parenthesized pattern")]
    ParsePatternInnerFailed { source: Box<Self> },
    #[error("failed to split pattern '{input}'")]
    SplitApplicationTermsFailed { source: Box<PlainParserParseError>, input: String },
    #[error("empty argument pattern")]
    PatternEmptyInvalid { input: String },
    #[error("cannot infer a pattern variable type in '{input}'")]
    VariableTypeInvalid { input: String },
    #[error("failed to parse constructor arguments")]
    ParsePatternArgumentFailed { source: Box<ParsePatternArgumentError> },
    #[error("failed to compare pattern types")]
    ConvertibleFailed { source: ScopeConvertibleError },
    #[error("pattern '{input}' has the wrong type")]
    TypeInvalid { input: String },
}

#[derive(Error, Debug)]
pub enum ParsePatternArgumentError {
    #[error("failed to normalize a constructor parameter type")]
    NormalizeTypFailed { source: ScopeNormalizeTypError },
    #[error("too many constructor arguments before '{input}'")]
    ArityInvalid { input: String },
    #[error("failed to parse a constructor argument")]
    ParsePatternFailed { source: Box<ParsePatternError> },
    #[error("failed to apply the constructor")]
    ApplyFailed { source: ScopeApplyError },
}

#[derive(Error, Debug)]
pub enum PlainParserParseError {
    #[error("failed to parse declarations and rewrite rules")]
    ParseScopeFailed { source: Box<PlainParserParseScopeError> },
    #[error("final theory validation failed")]
    TheoryNewFailed { source: Box<TheoryNewError> },
    #[error("failed to parse a rewrite rule")]
    ParseRewriteFailed { source: Box<ParseRewriteError> },
    #[error("'=' is not a definition operator; use '=>' in '{input}'")]
    DefinitionOperatorInvalid { input: String },
    #[error("failed to create a symbol from '{input}'")]
    NewSymFailed { source: ScopeNewSymError, input: String },
    #[error("line number overflow at index {line_index}")]
    LineNumberOverflowFailed { line_index: usize },
    #[error("failed to parse line {line_number}: '{line}'")]
    ParseLineFailed { source: Box<Self>, line_number: usize, line: String },
    #[error("multi-line comment opened on line {line_number} with '{start}' is missing closing delimiter '{end}'")]
    UnclosedMultiLineComment { line_number: usize, start: String, end: String },
    #[error("missing type separator ':' in '{input}'")]
    MissingTypeSeparator { input: String },
    #[error("invalid definition name '{input}'")]
    InvalidDefinitionName { input: String },
    #[error("failed to parse type '{input}'")]
    ParseTypeFailed { source: Box<Self>, input: String },
    #[error("failed to parse binder type '{input}'")]
    ParseBinderTypeFailed { source: Box<Self>, input: String },
    #[error("failed to parse domain type '{input}'")]
    ParseDomainTypeFailed { source: Box<Self>, input: String },
    #[error("failed to parse codomain '{input}'")]
    ParseCodomainFailed { source: Box<Self>, input: String },
    #[error("failed to parse expression '{input}'")]
    ParseExpressionFailed { source: Box<Self>, input: String },
    #[error("failed to parse expression atom '{input}'")]
    ParseExpressionAtomFailed { source: Box<Self>, input: String },
    #[error("failed to scan '{input}'")]
    ScanTextFailed { source: Box<Self>, input: String },
    #[error("type is empty")]
    EmptyType { input: String },
    #[error("expression is empty")]
    EmptyExpression { input: String },
    #[error("standalone binder type is invalid: '{input}'")]
    StandaloneBinderTypeInvalid { input: String },
    #[error("invalid binder name '{input}'")]
    InvalidBinderName { input: String },
    #[error("unknown identifier '{name}' in '{input}'")]
    UnknownIdentifier { name: String, input: String },
    #[error("top type '{input}' can't be used as an expression")]
    TopTypeUsedAsExpression { input: String },
    #[error("invalid application in '{input}'")]
    InvalidApplicationFailed { source: ScopeApplyError, input: String },
    #[error("unmatched closing parenthesis in '{input}'")]
    UnmatchedClosingParenthesis { input: String },
    #[error("unclosed parenthesis in '{input}'")]
    UnclosedParenthesis { input: String },
    #[error("parenthesis depth overflow while scanning '{input}'")]
    ParenthesisDepthOverflowFailed { input: String },
    #[error("failed to slice '{input}'")]
    TextSliceFailed { input: String },
}

#[cfg(test)]
mod tests {
    use crate::{Parse, PlainParser};
    use non_empty_str::non_empty_str;

    #[test]
    #[ignore]
    fn must_parse() {
        let mut parser = PlainParser::new(non_empty_str!("^_^"), non_empty_str!("//"), non_empty_str!("/*"), non_empty_str!("*/"));
        let _theory = parser
            .parse(include_str!("../../../samples/all.plain.base"))
            .unwrap();
    }
}
