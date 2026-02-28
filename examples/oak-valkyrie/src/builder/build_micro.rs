use crate::{ValkyrieLanguage, ValkyrieParser, ast::*, builder::text, kind::ValkyrieSyntaxKind};
use oak_core::{OakError, RedNode, RedTree, source::SourceText};

impl<'config> ValkyrieParser<'config> {
    pub(crate) fn build_mezzo(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<TypeFunction, OakError> {
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
                        name.name = text(source, t.span);
                        name.span = t.span;
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
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

        let body = body.ok_or_else(|| source.syntax_error(format!("Missing mezzo body at {:?}", span), span.start))?;

        Ok(TypeFunction { name, generics, annotations, params, return_type, body, span })
    }

    pub(crate) fn build_micro(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<MicroDefinition, OakError> {
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
                    _ => {
                        return Err(source.syntax_error(format!("Unexpected item in micro definition: {:?}", n.green.kind), n.span().start));
                    }
                },
            }
        }

        let body = body.ok_or_else(|| source.syntax_error(format!("Missing micro body at {:?}", span), span.start))?;

        Ok(MicroDefinition { name, generics, annotations, params, return_type, body, span })
    }

    pub(crate) fn build_lambda_expr(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<LambdaExpr, OakError> {
        let span = node.span();
        let mut params = Vec::new();
        let mut return_type = None;
        let mut body = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::ParameterList => params = self.build_params(n, source)?,
                    ValkyrieSyntaxKind::Type => return_type = Some(self.build_type(n, source)?),
                    ValkyrieSyntaxKind::BlockExpression => body = Some(self.build_block(n, source)?),
                    _ => {}
                },
            }
        }

        let body = body.ok_or_else(|| source.syntax_error(format!("Missing lambda body at {:?}", span), span.start))?;

        Ok(LambdaExpr { params, return_type, body, span })
    }

    pub(crate) fn build_generic_params(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Vec<GenericParam>, OakError> {
        let mut params = Vec::new();
        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::GenericParameter => params.push(self.build_generic_param(n, source)?),
                    _ => {}
                },
            }
        }
        Ok(params)
    }

    pub(crate) fn build_generic_param(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<GenericParam, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut constraints = Vec::new();
        let mut default = None;

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
                    ValkyrieSyntaxKind::Type => {
                        if default.is_none() {
                            constraints.push(self.build_type(n, source)?);
                        }
                    }
                    _ => {}
                },
            }
        }

        Ok(GenericParam { name, constraints, default, span })
    }

    pub(crate) fn build_type(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Type, OakError> {
        let span = node.span();
        
        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::NamePath => {
                        let path = self.build_name_path(n, source)?;
                        return Ok(Type::Named { path, span });
                    }
                    _ => {}
                },
            }
        }

        Ok(Type::Named { path: NamePath { parts: Vec::new(), span }, span })
    }

    pub(crate) fn build_params(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Vec<Param>, OakError> {
        let mut params = Vec::new();
        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Parameter => params.push(self.build_param(n, source)?),
                    _ => {}
                },
            }
        }
        Ok(params)
    }

    pub(crate) fn build_param(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Param, OakError> {
        let span = node.span();
        let mut name: Option<Identifier> = None;
        let mut ty = None;
        let mut default = None;
        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Identifier => {
                        if name.is_none() {
                            name = Some(Identifier { name: text(source, t.span), span: t.span });
                        }
                    }
                    ValkyrieSyntaxKind::Colon => continue,
                    ValkyrieSyntaxKind::Equal => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Type => ty = Some(self.build_type(n, source)?),
                    _ => {
                        if default.is_none() {
                            default = Some(self.build_expr(n, source)?);
                        }
                    }
                },
            }
        }
        if let Some(name) = name { 
            Ok(Param { name, ty, default, span }) 
        } else { 
            Err(source.syntax_error(format!("Missing name in parameter at {:?}", span), span.start)) 
        }
    }
}
