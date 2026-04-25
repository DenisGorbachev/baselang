use crate::{Exp, InvalidApplicationError, Parse, Typ, Var, VarRc};
use derive_new::new;
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

    fn parse(&mut self, input: &str) -> impl Iterator<Item = Result<VarRc, Self::Error>> {
        let mut scope = Vec::new();
        let mut failed = false;
        let top = self.top.clone();
        let single_line_comment = self.single_line_comment.clone();
        let multi_line_comment_start = self.multi_line_comment_start.clone();
        let multi_line_comment_end = self.multi_line_comment_end.clone();
        let mut lines = input.lines().enumerate();
        let mut multi_line_comment_start_line_number = None;
        core::iter::from_fn(move || {
            if failed {
                return None;
            }
            loop {
                return match lines.next() {
                    Some((line_index, raw_line)) => {
                        let line_number = match line_index.checked_add(1) {
                            Some(line_number) => line_number,
                            None => {
                                failed = true;
                                return Some(Err(PlainParserParseError::LineNumberOverflowFailed {
                                    line_index,
                                }));
                            }
                        };
                        let line = match strip_comments_from_line(raw_line, single_line_comment.as_ref().as_ref(), multi_line_comment_start.as_ref().as_ref(), multi_line_comment_end.as_ref().as_ref(), &mut multi_line_comment_start_line_number, line_number) {
                            Ok(line) => line,
                            Err(source) => {
                                failed = true;
                                return Some(Err(PlainParserParseError::ParseLineFailed {
                                    source: Box::new(source),
                                    line_number,
                                    line: raw_line.to_string(),
                                }));
                            }
                        };
                        let line = match normalize_line(line.as_ref()) {
                            Some(line) => line,
                            None => continue,
                        };
                        let result = match parse_line(line, &scope, top.as_ref().as_ref()) {
                            Ok(var) => {
                                scope.push(var.clone());
                                Ok(var)
                            }
                            Err(source) => {
                                failed = true;
                                Err(PlainParserParseError::ParseLineFailed {
                                    source: Box::new(source),
                                    line_number,
                                    line: line.to_string(),
                                })
                            }
                        };
                        Some(result)
                    }
                    None => {
                        if let Some(line_number) = multi_line_comment_start_line_number.take() {
                            failed = true;
                            return Some(Err(PlainParserParseError::UnclosedMultiLineComment {
                                line_number,
                                start: multi_line_comment_start.as_ref().to_string(),
                                end: multi_line_comment_end.as_ref().to_string(),
                            }));
                        }
                        None
                    }
                };
            }
        })
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

fn strip_comments_from_line<'a>(line: &'a str, single_line_comment: &str, multi_line_comment_start: &str, multi_line_comment_end: &str, multi_line_comment_start_line_number: &mut Option<usize>, line_number: usize) -> Result<Cow<'a, str>, PlainParserParseError> {
    let mut output = None::<String>;
    let mut remaining = line;
    let mut line_changed = false;
    loop {
        if multi_line_comment_start_line_number.is_some() {
            line_changed = true;
            let end_index = match remaining.find(multi_line_comment_end) {
                Some(end_index) => end_index,
                None => {
                    return Ok(Cow::Owned(output.unwrap_or_default()));
                }
            };
            let (_comment, suffix) = split_text_at(remaining, end_index)?;
            remaining = strip_prefix_checked(suffix, multi_line_comment_end)?;
            *multi_line_comment_start_line_number = None;
            continue;
        }
        let next_comment = find_next_comment_start(remaining, single_line_comment, multi_line_comment_start);
        let (comment_index, is_multi_line_comment) = match next_comment {
            Some(next_comment) => next_comment,
            None => {
                if let Some(mut output) = output {
                    output.push_str(remaining);
                    return Ok(Cow::Owned(output));
                }
                if line_changed {
                    return Ok(Cow::Owned(remaining.to_string()));
                }
                return Ok(Cow::Borrowed(line));
            }
        };
        line_changed = true;
        let (prefix, suffix) = split_text_at(remaining, comment_index)?;
        push_visible_text(&mut output, prefix);
        if is_multi_line_comment {
            remaining = strip_prefix_checked(suffix, multi_line_comment_start)?;
            *multi_line_comment_start_line_number = Some(line_number);
            continue;
        }
        return Ok(Cow::Owned(output.unwrap_or_default()));
    }
}

fn push_visible_text(output: &mut Option<String>, visible_text: &str) {
    if visible_text.is_empty() {
        return;
    }
    match output {
        Some(output) => output.push_str(visible_text),
        None => {
            *output = Some(visible_text.to_string());
        }
    }
}

fn find_next_comment_start(input: &str, single_line_comment: &str, multi_line_comment_start: &str) -> Option<(usize, bool)> {
    let single_line_index = input.find(single_line_comment);
    let multi_line_index = input.find(multi_line_comment_start);
    match (single_line_index, multi_line_index) {
        (Some(single_line_index), Some(multi_line_index)) if single_line_index < multi_line_index => Some((single_line_index, false)),
        (Some(single_line_index), Some(multi_line_index)) if multi_line_index < single_line_index => Some((multi_line_index, true)),
        (Some(single_line_index), Some(_multi_line_index)) => {
            if multi_line_comment_start.len() > single_line_comment.len() {
                Some((single_line_index, true))
            } else {
                Some((single_line_index, false))
            }
        }
        (Some(single_line_index), None) => Some((single_line_index, false)),
        (None, Some(multi_line_index)) => Some((multi_line_index, true)),
        (None, None) => None,
    }
}

fn split_text_at(input: &str, index: usize) -> Result<(&str, &str), PlainParserParseError> {
    use PlainParserParseError::*;
    let prefix = match input.get(..index) {
        Some(prefix) => prefix,
        None => {
            return Err(TextSliceFailed {
                input: input.to_string(),
            });
        }
    };
    let suffix = match input.get(index..) {
        Some(suffix) => suffix,
        None => {
            return Err(TextSliceFailed {
                input: input.to_string(),
            });
        }
    };
    Ok((prefix, suffix))
}

fn strip_prefix_checked<'a>(input: &'a str, prefix: &str) -> Result<&'a str, PlainParserParseError> {
    use PlainParserParseError::*;
    match input.strip_prefix(prefix) {
        Some(input) => Ok(input),
        None => Err(TextSliceFailed {
            input: input.to_string(),
        }),
    }
}

fn parse_line(line: &str, scope: &[VarRc], top: &str) -> Result<VarRc, PlainParserParseError> {
    use PlainParserParseError::*;
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
    if name.is_empty() || name.chars().any(char::is_whitespace) {
        return Err(InvalidDefinitionName {
            input: name.to_string(),
        });
    }
    let typ = match parse_typ(typ, scope, top) {
        Ok(typ) => typ,
        Err(source) => {
            return Err(ParseTypeFailed {
                source: Box::new(source),
                input: typ.to_string(),
            });
        }
    };
    Ok(new_var_rc(name, typ))
}

fn parse_typ(input: &str, scope: &[VarRc], top: &str) -> Result<Typ, PlainParserParseError> {
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
            return parse_typ(inner, scope, top);
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
                let binder_typ = match parse_typ(binder_typ_text, scope, top) {
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
                    .map(|name| new_var_rc(name, binder_typ.clone()))
                    .collect::<Vec<_>>();
                let mut inner_scope = scope.to_vec();
                inner_scope.extend(binders.iter().cloned());
                let output = match parse_output_var(codomain, &inner_scope, top) {
                    Ok(output) => output,
                    Err(source) => {
                        return Err(ParseCodomainFailed {
                            source: Box::new(source),
                            input: codomain.to_string(),
                        });
                    }
                };
                let mut vars = binders;
                vars.push(output);
                let typ = Typ::fun(vars);
                return Ok(typ);
            }
        }
        let domain_typ = match parse_typ(wrapped_domain.unwrap_or(domain), scope, top) {
            Ok(domain_typ) => domain_typ,
            Err(source) => {
                return Err(ParseDomainTypeFailed {
                    source: Box::new(source),
                    input: domain.to_string(),
                });
            }
        };
        let output = match parse_output_var(codomain, scope, top) {
            Ok(output) => output,
            Err(source) => {
                return Err(ParseCodomainFailed {
                    source: Box::new(source),
                    input: codomain.to_string(),
                });
            }
        };
        let binder = Var::new_anon_rc(domain_typ, None);
        return Ok(Typ::fun([binder, output]));
    }

    if input == top && find_var(scope, input).is_none() {
        return Ok(Typ::top());
    }
    let exp = match parse_exp(input, scope, top) {
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

fn parse_output_var(input: &str, scope: &[VarRc], top: &str) -> Result<VarRc, PlainParserParseError> {
    use PlainParserParseError::*;
    let input = input.trim();
    let has_arrow = match find_first_top_level_arrow(input) {
        Ok(has_arrow) => has_arrow,
        Err(source) => {
            return Err(ScanTextFailed {
                source: Box::new(source),
                input: input.to_string(),
            });
        }
    };
    if has_arrow.is_some() {
        let typ = match parse_typ(input, scope, top) {
            Ok(typ) => typ,
            Err(source) => {
                return Err(ParseTypeFailed {
                    source: Box::new(source),
                    input: input.to_string(),
                });
            }
        };
        return Ok(Var::new_anon_rc(typ, None));
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
        let binder_colon_index = match find_first_top_level_colon(inner) {
            Ok(binder_colon_index) => binder_colon_index,
            Err(source) => {
                return Err(ScanTextFailed {
                    source: Box::new(source),
                    input: inner.to_string(),
                });
            }
        };
        if let Some(binder_colon_index) = binder_colon_index {
            let name = match inner.get(..binder_colon_index) {
                Some(name) => name.trim(),
                None => {
                    return Err(TextSliceFailed {
                        input: inner.to_string(),
                    });
                }
            };
            let typ = match inner.get(binder_colon_index..) {
                Some(suffix) => match suffix.strip_prefix(':') {
                    Some(typ) => typ.trim(),
                    None => {
                        return Err(TextSliceFailed {
                            input: inner.to_string(),
                        });
                    }
                },
                None => {
                    return Err(TextSliceFailed {
                        input: inner.to_string(),
                    });
                }
            };
            if name.is_empty() || name.chars().any(char::is_whitespace) {
                return Err(InvalidBinderName {
                    input: name.to_string(),
                });
            }
            let typ = match parse_typ(typ, scope, top) {
                Ok(typ) => typ,
                Err(source) => {
                    return Err(ParseBinderTypeFailed {
                        source: Box::new(source),
                        input: typ.to_string(),
                    });
                }
            };
            return Ok(new_var_rc(name, typ));
        }
    }
    match parse_exp(input, scope, top)? {
        Exp::Sol(var) => Ok(var),
        _ => Err(OutputVarExpected {
            input: input.to_string(),
        }),
    }
}

fn parse_exp(input: &str, scope: &[VarRc], top: &str) -> Result<Exp, PlainParserParseError> {
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
        return parse_exp(inner, scope, top);
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
    let first = match parse_exp_atom(first, scope, top) {
        Ok(first) => first,
        Err(source) => {
            return Err(ParseExpressionAtomFailed {
                source: Box::new(source),
                input: first.to_string(),
            });
        }
    };
    terms.try_fold(first, |fun, arg| {
        let arg = match parse_exp_atom(arg, scope, top) {
            Ok(arg) => arg,
            Err(source) => {
                return Err(ParseExpressionAtomFailed {
                    source: Box::new(source),
                    input: arg.to_string(),
                });
            }
        };
        parse_application(fun, arg).map_err(|source| InvalidApplicationFailed {
            source,
            input: input.to_string(),
        })
    })
}

fn parse_exp_atom(input: &str, scope: &[VarRc], top: &str) -> Result<Exp, PlainParserParseError> {
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
        return parse_exp(inner, scope, top);
    }
    if input == top && find_var(scope, input).is_none() {
        return Err(TopTypeUsedAsExpression {
            input: input.to_string(),
        });
    }
    match find_var(scope, input) {
        Some(var) => Ok(Exp::sol(&var)),
        None => Err(UnknownIdentifier {
            name: input.to_string(),
            input: input.to_string(),
        }),
    }
}

fn parse_application(fun: Exp, arg: Exp) -> Result<Exp, InvalidApplicationError> {
    match Exp::app(fun.clone(), arg.clone()) {
        Ok(exp) => Ok(exp),
        Err(source) => match fun.typ().after_apply(&arg) {
            Some(typ) => Ok(Exp::App(Box::new(fun), Box::new(arg), Box::new(typ))),
            None => Err(source),
        },
    }
}

fn find_var(scope: &[VarRc], name: &str) -> Option<VarRc> {
    scope.iter().rev().find_map(|var| {
        let candidate = var.nym().as_ref()?.short.en.singular.canonical.as_str();
        if candidate == name { Some(var.clone()) } else { None }
    })
}

fn new_var_rc(name: &str, typ: Typ) -> VarRc {
    if name == "_" { Var::new_anon_rc(typ, None) } else { Var::new_rc(name.to_string(), typ, None) }
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
            '-' if depth == 0 => {
                if input
                    .get(index..)
                    .is_some_and(|suffix| suffix.starts_with("->"))
                {
                    return Ok(Some(index));
                }
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

#[derive(Error, Debug)]
pub enum PlainParserParseError {
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
    #[error("output '{input}' must be a named var")]
    OutputVarExpected { input: String },
    #[error("invalid binder name '{input}'")]
    InvalidBinderName { input: String },
    #[error("unknown identifier '{name}' in '{input}'")]
    UnknownIdentifier { name: String, input: String },
    #[error("top type '{input}' can't be used as an expression")]
    TopTypeUsedAsExpression { input: String },
    #[error("invalid application in '{input}'")]
    InvalidApplicationFailed { source: InvalidApplicationError, input: String },
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
    use itertools::Itertools;
    use non_empty_str::non_empty_str;

    #[test]
    #[ignore]
    fn must_parse() {
        let mut parser = PlainParser::new(non_empty_str!("^_^"), non_empty_str!("//"), non_empty_str!("/*"), non_empty_str!("*/"));
        let iter = parser.parse(include_str!("../../../samples/all.plain.base"));
        let _: Vec<_> = iter.try_collect().unwrap();
    }
}
