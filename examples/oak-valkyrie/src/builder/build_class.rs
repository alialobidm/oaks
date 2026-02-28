use crate::{
    ValkyrieLanguage, ValkyrieParser,
    ast::{Class, Enums, Flags, Identifier, Item, Trait, Variant, VariantCase, Widget, EnumVariant, Field},
    builder::text,
    kind::ValkyrieSyntaxKind,
};
use oak_core::{OakError, RedNode, RedTree, source::SourceText};

impl<'config> ValkyrieParser<'config> {
    pub(crate) fn build_class(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Class, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut generics = Vec::new();
        let mut annotations = Vec::new();
        let mut parents = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = Identifier { name: t_text, span: t.span.clone() };
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
                    ValkyrieSyntaxKind::NamePath => {
                        parents.push(self.build_name_path(n, source)?);
                    }
                    ValkyrieSyntaxKind::Type => {
                        for child in n.children() {
                            if let RedTree::Node(inner) = child {
                                if inner.green.kind == ValkyrieSyntaxKind::NamePath {
                                    parents.push(self.build_name_path(inner, source)?);
                                }
                            }
                        }
                    }
                    ValkyrieSyntaxKind::Namespace => {
                        let ns = self.build_namespace(n, source)?;
                        items.push(Item::Namespace(ns));
                    }
                    ValkyrieSyntaxKind::Class => {
                        let class = self.build_class(n, source)?;
                        items.push(Item::Class(class));
                    }
                    ValkyrieSyntaxKind::Flags => {
                        let flags = self.build_flags(n, source)?;
                        items.push(Item::Flags(flags));
                    }
                    ValkyrieSyntaxKind::Enums => {
                        let enums = self.build_enums(n, source)?;
                        items.push(Item::Enums(enums));
                    }
                    ValkyrieSyntaxKind::Trait => {
                        let trait_node = self.build_trait(n, source)?;
                        items.push(Item::Trait(trait_node));
                    }
                    ValkyrieSyntaxKind::Widget => {
                        let widget = self.build_widget(n, source)?;
                        items.push(Item::Widget(widget));
                    }
                    ValkyrieSyntaxKind::UsingStatement => {
                        let us = self.build_using(n, source)?;
                        items.push(Item::Using(us));
                    }
                    ValkyrieSyntaxKind::Micro => {
                        let micro = self.build_micro(n, source)?;
                        items.push(Item::Micro(micro));
                    }
                    ValkyrieSyntaxKind::LetStatement => {
                        let stmt = self.build_let(n, source)?;
                        items.push(Item::Statement(stmt));
                    }
                    ValkyrieSyntaxKind::ExpressionStatement => {
                        let stmt = self.build_expr_stmt(n, source)?;
                        items.push(Item::Statement(stmt));
                    }
                    ValkyrieSyntaxKind::Variant => {
                        let variant = self.build_variant_decl(n, source)?;
                        items.push(Item::Variant(variant));
                    }
                    ValkyrieSyntaxKind::BlockExpression => {
                        for inner_child in n.children() {
                            if let RedTree::Node(inner_n) = inner_child {
                                if let Ok(item) = self.build_item(inner_n, source) {
                                    items.push(item);
                                }
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
        Ok(Class { name, generics, annotations, parents, items, span })
    }

    pub(crate) fn build_flags(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Flags, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut annotations = Vec::new();
        let mut variants = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = Identifier { name: t_text, span: t.span.clone() };
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Attribute => annotations.push(self.build_attribute(n, source)?),
                    ValkyrieSyntaxKind::Variant => {
                        let variant = self.build_enum_variant(n, source)?;
                        variants.push(variant);
                    }
                    ValkyrieSyntaxKind::BlockExpression => {
                        for inner_child in n.children() {
                            if let RedTree::Node(inner_n) = inner_child {
                                if inner_n.green.kind == ValkyrieSyntaxKind::Variant {
                                    if let Ok(v) = self.build_enum_variant(inner_n, source) {
                                        variants.push(v);
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
        Ok(Flags { name, variants, annotations, span })
    }

    pub(crate) fn build_enums(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Enums, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut generics = Vec::new();
        let mut annotations = Vec::new();
        let mut variants = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = Identifier { name: t_text, span: t.span.clone() }
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Attribute => annotations.push(self.build_attribute(n, source)?),
                    ValkyrieSyntaxKind::GenericParameterList => {
                        generics = self.build_generic_params(n, source)?;
                    }
                    ValkyrieSyntaxKind::Variant => {
                        let variant = self.build_enum_variant(n, source)?;
                        variants.push(variant)
                    }
                    ValkyrieSyntaxKind::BlockExpression => {
                        for inner_child in n.children() {
                            if let RedTree::Node(inner_n) = inner_child {
                                if inner_n.green.kind == ValkyrieSyntaxKind::Variant {
                                    if let Ok(v) = self.build_enum_variant(inner_n, source) {
                                        variants.push(v)
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
        Ok(Enums { name, generics, variants, annotations, span })
    }

    pub(crate) fn build_enum_variant(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<EnumVariant, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut fields = Vec::new();
        let mut annotations = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = Identifier { name: t_text, span: t.span.clone() };
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Attribute => annotations.push(self.build_attribute(n, source)?),
                    ValkyrieSyntaxKind::Field => {
                        if let Ok(field) = self.build_field(n, source) {
                            fields.push(field);
                        }
                    }
                    _ => {}
                },
            }
        }
        Ok(EnumVariant { name, fields, annotations, span })
    }

    pub(crate) fn build_field(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Field, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut ty = None;
        let mut default = None;
        let mut annotations = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Identifier => {
                        if name.name.is_empty() {
                            name = Identifier { name: text(source, t.span.clone().into()), span: t.span.clone() };
                        }
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Attribute => annotations.push(self.build_attribute(n, source)?),
                    ValkyrieSyntaxKind::Type => ty = Some(self.build_type(n, source)?),
                    _ => {
                        if default.is_none() {
                            default = Some(self.build_expr(n, source)?);
                        }
                    }
                },
            }
        }

        let ty = ty.ok_or_else(|| source.syntax_error("Missing type for field", span.start))?;
        Ok(Field { name, ty, default, annotations, span })
    }

    pub(crate) fn build_variant_decl(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Variant, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut generics = Vec::new();
        let mut annotations = Vec::new();
        let mut cases = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = Identifier { name: t_text, span: t.span.clone() };
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Attribute => annotations.push(self.build_attribute(n, source)?),
                    ValkyrieSyntaxKind::GenericParameterList => {
                        generics = self.build_generic_params(n, source)?;
                    }
                    ValkyrieSyntaxKind::MatchArm => {
                        if let Ok(case) = self.build_variant_case(n, source) {
                            cases.push(case);
                        }
                    }
                    _ => {}
                },
            }
        }
        Ok(Variant { name, generics, cases, annotations, span })
    }

    pub(crate) fn build_variant_case(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<VariantCase, OakError> {
        let span = node.span();
        let mut pattern = None;
        let mut body = None;

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Pattern => {
                        pattern = Some(self.build_pattern(n, source)?);
                    }
                    _ => {
                        if body.is_none() {
                            body = Some(self.build_expr(n, source)?);
                        }
                    }
                },
            }
        }

        let pattern = pattern.ok_or_else(|| source.syntax_error("Missing pattern in variant case", span.start))?;
        let body = body.ok_or_else(|| source.syntax_error("Missing body in variant case", span.start))?;
        Ok(VariantCase { pattern, body, span })
    }

    pub(crate) fn build_trait(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Trait, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut generics = Vec::new();
        let mut annotations = Vec::new();
        let mut methods = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Identifier => {
                        let t_text = text(source, t.span.clone().into());
                        name = Identifier { name: t_text, span: t.span.clone() };
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Attribute => annotations.push(self.build_attribute(n, source)?),
                    ValkyrieSyntaxKind::GenericParameterList => {
                        generics = self.build_generic_params(n, source)?;
                    }
                    ValkyrieSyntaxKind::NamePath => {},
                    ValkyrieSyntaxKind::Type => {},
                    ValkyrieSyntaxKind::Micro => {
                        if let Ok(func) = self.build_function(n, source) {
                            methods.push(func);
                        }
                    }
                    ValkyrieSyntaxKind::BlockExpression => {
                        for inner_child in n.children() {
                            if let RedTree::Node(inner_n) = inner_child {
                                if inner_n.green.kind == ValkyrieSyntaxKind::Micro {
                                    if let Ok(func) = self.build_function(inner_n, source) {
                                        methods.push(func);
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
        Ok(Trait { name, generics, methods, annotations, span })
    }

    pub(crate) fn build_widget(&self, node: RedNode<ValkyrieLanguage>, source: &SourceText) -> Result<Widget, OakError> {
        let span = node.span();
        let mut name = Identifier { name: String::new(), span: Default::default() };
        let mut generics = Vec::new();
        let mut annotations = Vec::new();
        let mut items = Vec::new();

        for child in node.children() {
            match child {
                RedTree::Leaf(t) => match t.kind {
                    ValkyrieSyntaxKind::Whitespace | ValkyrieSyntaxKind::Newline | ValkyrieSyntaxKind::LineComment | ValkyrieSyntaxKind::BlockComment => continue,
                    ValkyrieSyntaxKind::Identifier => {
                        name = Identifier { name: text(source, t.span.clone().into()), span: t.span.clone() };
                    }
                    _ => {}
                },
                RedTree::Node(n) => match n.green.kind {
                    ValkyrieSyntaxKind::Attribute => annotations.push(self.build_attribute(n, source)?),
                    ValkyrieSyntaxKind::GenericParameterList => {
                        generics = self.build_generic_params(n, source)?;
                    }
                    ValkyrieSyntaxKind::Namespace => {
                        let ns = self.build_namespace(n, source)?;
                        items.push(Item::Namespace(ns))
                    }
                    ValkyrieSyntaxKind::Class => {
                        let class = self.build_class(n, source)?;
                        items.push(Item::Class(class))
                    }
                    ValkyrieSyntaxKind::Flags => {
                        let flags = self.build_flags(n, source)?;
                        items.push(Item::Flags(flags))
                    }
                    ValkyrieSyntaxKind::Trait => {
                        let trait_node = self.build_trait(n, source)?;
                        items.push(Item::Trait(trait_node))
                    }
                    ValkyrieSyntaxKind::Widget => {
                        let widget = self.build_widget(n, source)?;
                        items.push(Item::Widget(widget))
                    }
                    ValkyrieSyntaxKind::UsingStatement => {
                        let us = self.build_using(n, source)?;
                        items.push(Item::Using(us))
                    }
                    ValkyrieSyntaxKind::Micro => {
                        let micro = self.build_micro(n, source)?;
                        items.push(Item::Micro(micro))
                    }
                    ValkyrieSyntaxKind::LetStatement => {
                        let stmt = self.build_let(n, source)?;
                        items.push(Item::Statement(stmt))
                    }
                    ValkyrieSyntaxKind::ExpressionStatement => {
                        let stmt = self.build_expr_stmt(n, source)?;
                        items.push(Item::Statement(stmt))
                    }
                    _ => {}
                },
            }
        }
        Ok(Widget { name, generics, items, annotations, span })
    }
}
