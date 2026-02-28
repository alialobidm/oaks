use crate::{ValkyrieLanguage, ValkyrieParser, ast::*, builder::text, kind::{ValkyrieSyntaxKind, ValkyrieKeywords}};
use oak_core::{OakError, RedNode, RedTree, source::SourceText};

impl<'config> ValkyrieParser<'config> {
    pub(crate) fn build_let(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Statement, OakError> {
        let span = node.span();
        let mut children_iter = node
            .children()
            .filter(|c| match c {
                RedTree::Leaf(l) => !matches!(l.kind, ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment),
                RedTree::Node(n) => !matches!(n.green.kind, ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment),
            })
            .peekable();

        let mut annotations = Vec::new();
        while let Some(child) = children_iter.peek() {
            if let RedTree::Node(n) = child {
                if n.green.kind == ValkyrieSyntaxKind::Attribute {
                    annotations.push(self.build_attribute(n.clone(), source)?);
                    children_iter.next();
                    continue;
                }
            }
            break;
        }

        let let_keyword = children_iter.next().ok_or_else(|| source.syntax_error("Missing 'let' keyword", span.start))?;
        match let_keyword {
            RedTree::Leaf(t) if t.kind == ValkyrieSyntaxKind::Keyword(crate::lexer::ValkyrieKeywords::Let) => {}
            _ => {
                return Err(source.syntax_error("Expected 'let' keyword", let_keyword.span().start));
            }
        }

        let mut is_mutable = false;
        if let Some(RedTree::Leaf(t)) = children_iter.peek() {
            if t.kind == ValkyrieSyntaxKind::Keyword(crate::lexer::ValkyrieKeywords::Mut) {
                is_mutable = true;
                children_iter.next();
            }
        }

        let pattern_node = children_iter.next().ok_or_else(|| source.syntax_error("Missing pattern in let statement", span.start))?;
        let pattern = match pattern_node {
            RedTree::Node(n) => self.build_pattern(n, source)?,
            RedTree::Leaf(t) if t.kind == ValkyrieSyntaxKind::Identifier => {
                let t_text = text(source, t.span);
                Pattern::Variable { name: Identifier { name: t_text, span: t.span }, span: t.span }
            }
            _ => {
                return Err(source.syntax_error("Expected pattern in let statement", pattern_node.span().start));
            }
        };

        let mut expr: Option<Expr> = None;

        if let Some(RedTree::Leaf(t)) = children_iter.peek() {
            if t.kind == ValkyrieSyntaxKind::Eq{
                children_iter.next();

                let expr_node = children_iter.next().ok_or_else(|| source.syntax_error("Missing expression after '=' in let statement", span.end))?;

                expr = Some(match expr_node {
                    RedTree::Node(n) => self.build_expr(n, source)?,
                    RedTree::Leaf(t) => {
                        return Err(source.syntax_error("Expected an expression, found a token after '=' in let statement", t.span.start));
                    }
                });
            }
        }

        while let Some(unexpected_child) = children_iter.next() {
            match unexpected_child {
                RedTree::Leaf(t) if t.kind == ValkyrieSyntaxKind::Semicolon => {}
                _ => {
                    let child_span = unexpected_child.span();
                    if child_span.start == child_span.end {
                        continue;
                    }
                    return Err(source.syntax_error("Unexpected token or expression after let statement", unexpected_child.span().start));
                }
            }
        }

        let expr = expr.ok_or_else(|| source.syntax_error("Missing expression in let statement", span.start))?;

        Ok(Statement::Let { annotations, is_mutable, pattern, expr, ty: None, span })
    }

    pub(crate) fn build_expr_stmt(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Statement, OakError> {
        let span = node.span();
        let mut children_iter = node
            .children()
            .filter(|c| match c {
                RedTree::Leaf(l) => !matches!(l.kind, ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment),
                RedTree::Node(n) => !matches!(n.green.kind, ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment),
            })
            .peekable();

        let mut annotations = Vec::new();
        while let Some(child) = children_iter.peek() {
            if let RedTree::Node(n) = child {
                if n.green.kind == ValkyrieSyntaxKind::Attribute {
                    annotations.push(self.build_attribute(n.clone(), source)?);
                    children_iter.next();
                    continue;
                }
            }
            break;
        }

        let expr_node = children_iter.next().ok_or_else(|| source.syntax_error("Missing expression in expression statement", span.start))?;

        let expr = match expr_node {
            RedTree::Node(n) => self.build_expr(n, source)?,
            RedTree::Leaf(t) => {
                return Err(source.syntax_error("Expected an expression, found a token", t.span.start));
            }
        };

        let mut semi = false;
        while let Some(RedTree::Leaf(t)) = children_iter.peek() {
            if t.kind == ValkyrieSyntaxKind::Semicolon{
                semi = true;
                children_iter.next();
                continue;
            }
            break;
        }

        while let Some(unexpected_child) = children_iter.next() {
            let child_span = unexpected_child.span();
            if child_span.start == child_span.end {
                continue;
            }
            return Err(source.syntax_error("Unexpected token or expression after semicolon", unexpected_child.span().start));
        }

        Ok(Statement::ExprStmt { annotations, expr, semi, span })
    }

    pub(crate) fn build_using(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Using, OakError> {
        let span = node.span();
        let mut path = NamePath { parts: Vec::new(), span: Default::default() };
        let mut alias = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Identifier => {
                        if !path.parts.is_empty() && alias.is_none() {
                            alias = Some(Identifier { name: text(source, t.span), span: t.span });
                        }
                    }
                    _ => {}
                },
                RedTree::Node(n) => {
                    if n.green.kind == ValkyrieSyntaxKind::NamePath {
                        path = self.build_name_path(n, source)?;
                    }
                }
            }
        }
        Ok(Using { path, alias, span })
    }

    pub(crate) fn build_effect(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Effect, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut operations = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Identifier => name = Identifier { name: text(source, t.span), span: t.span },
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Attribute => annotations.push(self.build_attribute(n, source)?),
                    ValkyrieSyntaxKind::BlockExpression => {
                        for inner_child in n.children() {
                            if let RedTree::Node(inner_n) = inner_child {
                                if inner_n.green.kind == ValkyrieSyntaxKind::Micro {
                                    if let Ok(func) = self.build_function(inner_n, source) {
                                        operations.push(func);
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
        Ok(Effect { name, operations, annotations, span })
    }

    pub(crate) fn build_function(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Function, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut generics = Vec::new();
        let mut annotations = Vec::new();
        let mut params = Vec::new();
        let mut return_type = None;
        let mut body = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Identifier => {
                        if name.name.is_empty() {
                            name.name = text(source, t.span);
                            name.span = t.span;
                        }
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Attribute => {
                        annotations.push(self.build_attribute(n, source)?);
                    }
                    ValkyrieSyntaxKind::GenericParameterList => {
                        generics = self.build_generic_params(n, source)?;
                    }
                    ValkyrieSyntaxKind::ParameterList => {
                        params = self.build_params(n, source)?;
                    }
                    ValkyrieSyntaxKind::Type => {
                        return_type = Some(self.build_type(n, source)?);
                    }
                    ValkyrieSyntaxKind::BlockExpression => {
                        body = Some(self.build_block(n, source)?);
                    }
                    _ => {}
                },
            }
        }

        Ok(Function { name, generics, params, return_type, body, annotations, span })
    }

    pub(crate) fn build_attribute(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Attribute, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut args = Vec::new();
        let mut seen_name = false;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Identifier => {
                        if !seen_name {
                            name.name = text(source, t.span);
                            name.span = t.span;
                            seen_name = true;
                        }
                    }
                    ValkyrieSyntaxKind::At => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::NamePath => {
                        if !seen_name {
                            let path = self.build_name_path(n, source)?;
                            name.name = path.parts.iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join("::");
                            name.span = path.span;
                            seen_name = true;
                            continue;
                        }
                        args.push(self.build_expr(n, source)?);
                    }
                    _ => args.push(self.build_expr(n, source)?),
                },
            }
        }

        Ok(Attribute { name, args, span })
    }

    pub(crate) fn build_name_path(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<NamePath, OakError> {
        let span = node.span();
        let mut parts = Vec::new();

        for child in node.children() {
            if let RedTree::Leaf(t) = child {
                match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Identifier => parts.push(Identifier { name: text(source, t.span), span: t.span }),
                    _ => {}
                }
            }
        }
        Ok(NamePath { parts, span })
    }
}
