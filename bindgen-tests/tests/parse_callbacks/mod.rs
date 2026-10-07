mod add_derives_callback;
mod item_discovery_callback;

use bindgen::callbacks::*;
use bindgen::{CodeGenAttributes, FieldVisibilityKind};

#[derive(Debug)]
pub struct RemovePrefixParseCallback {
    pub remove_prefix: Option<String>,
}

impl RemovePrefixParseCallback {
    pub fn new(prefix: &str) -> Self {
        RemovePrefixParseCallback {
            remove_prefix: Some(prefix.to_string()),
        }
    }
}

impl ParseCallbacks for RemovePrefixParseCallback {
    fn generated_name_override(&self, item_info: ItemInfo) -> Option<String> {
        if let Some(prefix) = &self.remove_prefix {
            let (expected_prefix, expected_suffix) = match item_info.kind {
                ItemKind::Function => ("function_", "_name"),
                ItemKind::Var => ("var_", "_name"),
                _ => todo!(),
            };
            if let Some(name) = item_info.name.strip_prefix(prefix) {
                assert!(name.starts_with(expected_prefix));
                assert!(name.ends_with(expected_suffix));
                return Some(name.to_string());
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct PrefixLinkNameParseCallback {
    pub prefix: Option<String>,
}

impl PrefixLinkNameParseCallback {
    pub fn new(prefix: &str) -> Self {
        PrefixLinkNameParseCallback {
            prefix: Some(prefix.to_string()),
        }
    }
}

impl ParseCallbacks for PrefixLinkNameParseCallback {
    fn generated_link_name_override(
        &self,
        item_info: ItemInfo,
    ) -> Option<String> {
        self.prefix
            .as_deref()
            .map(|prefix| format!("{prefix}{}", item_info.name))
    }
}

#[derive(Debug)]
struct EnumVariantRename;

impl ParseCallbacks for EnumVariantRename {
    fn enum_variant_name(
        &self,
        _enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: EnumVariantValue,
    ) -> Option<String> {
        Some(format!("RENAMED_{original_variant_name}"))
    }
}

#[derive(Debug)]
struct StructFieldRename;

impl ParseCallbacks for StructFieldRename {
    fn field_name(&self, info: FieldInfo<'_>) -> Option<String> {
        if info.type_name == "RenameMe" {
            if info.field_type_name == Some("char") {
                return Some(format!("char_{}", info.field_name));
            }
            match info.field_name {
                "renamed_member" => Some("rust_friendly".into()),
                "bitfield_uGlyName" => Some("bitfield_less_ugly_name".into()),
                "bitfieldWorse_name" => Some("bitfield_better_name".into()),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct BlocklistedTypeImplementsTrait;

impl ParseCallbacks for BlocklistedTypeImplementsTrait {
    fn blocklisted_type_implements_trait(
        &self,
        _name: &str,
        derive_trait: DeriveTrait,
    ) -> Option<ImplementsTrait> {
        if derive_trait == DeriveTrait::Hash {
            Some(ImplementsTrait::No)
        } else {
            Some(ImplementsTrait::Yes)
        }
    }
}

#[derive(Debug)]
struct FieldVisibility {
    default: FieldVisibilityKind,
}

/// Implements the `field_visibility` function of the trait by checking if the
/// field name starts with `private_`. If it does, it makes it private, if it
/// doesn't, it makes it public, taking into account the default visibility.
impl ParseCallbacks for FieldVisibility {
    fn field_visibility(
        &self,
        FieldInfo { field_name, .. }: FieldInfo,
    ) -> Option<FieldVisibilityKind> {
        match (self.default, field_name.starts_with("private_")) {
            (FieldVisibilityKind::Private, false) => {
                Some(FieldVisibilityKind::Public)
            }
            (FieldVisibilityKind::Public, true) => {
                Some(FieldVisibilityKind::Private)
            }
            (FieldVisibilityKind::PublicCrate, _) => unimplemented!(),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct TypeVisibility;

/// Implements the `field_visibility` function of the trait by checking the
/// type name. Depending on name prefix, it will return a different visibility.
impl ParseCallbacks for TypeVisibility {
    fn field_visibility(
        &self,
        FieldInfo { type_name, .. }: FieldInfo,
    ) -> Option<FieldVisibilityKind> {
        if type_name.starts_with("private_") {
            Some(FieldVisibilityKind::Private)
        } else if type_name.starts_with("pubcrate_") {
            Some(FieldVisibilityKind::PublicCrate)
        } else if type_name.starts_with("pub_") {
            Some(FieldVisibilityKind::Public)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub(super) struct WrapAsVariadicFn;

impl ParseCallbacks for WrapAsVariadicFn {
    fn wrap_as_variadic_fn(&self, name: &str) -> Option<String> {
        Some(name.to_owned() + "_wrapped")
    }
}

/// Marks the comments that it processes, and adds the comments that it parses
/// for attributes as documentation, to show which form of the comment each
/// callback gets.
#[derive(Debug)]
struct CommentAttributes;

impl ParseCallbacks for CommentAttributes {
    fn process_comment(&self, comment: &str) -> Option<String> {
        Some(format!("{comment} (processed)"))
    }

    fn parse_comments_for_attributes(
        &self,
        comment: &str,
    ) -> Vec<CodeGenAttributes> {
        vec![CodeGenAttributes::Doc(format!("Parsed:{comment}"))]
    }
}

/// Adds an always-false `cfg` and a `cfg_attr` to items whose comment contains
/// `cfg-guarded`. Compiling the expectation catches items which use a guarded
/// item without being guarded themselves.
#[derive(Debug)]
struct CommentCfgAttributes;

impl ParseCallbacks for CommentCfgAttributes {
    fn parse_comments_for_attributes(
        &self,
        comment: &str,
    ) -> Vec<CodeGenAttributes> {
        if comment.contains("cfg-guarded") {
            vec![
                CodeGenAttributes::Cfg("any()".to_string()),
                CodeGenAttributes::CfgAttr(
                    "all(), allow(dead_code)".to_string(),
                ),
            ]
        } else {
            vec![]
        }
    }
}

#[derive(Debug)]
struct ResultErrorEnumRename;

impl ParseCallbacks for ResultErrorEnumRename {
    fn result_error_enum_name(
        &self,
        original_enum_name: &str,
    ) -> Option<String> {
        original_enum_name
            .strip_suffix("Result")
            .map(|base| format!("{base}ErrorCode"))
    }
}

#[derive(Debug)]
pub(super) struct OperatorRename;

impl ParseCallbacks for OperatorRename {
    fn generated_name_override(&self, info: ItemInfo) -> Option<String> {
        if info.name == "operator=" {
            Some("operatorequals".to_string())
        } else {
            None
        }
    }
}

pub fn lookup(cb: &str) -> Box<dyn ParseCallbacks> {
    match cb {
        "comment-attributes" => Box::new(CommentAttributes),
        "comment-cfg-attributes" => Box::new(CommentCfgAttributes),
        "result-error-enum-rename" => Box::new(ResultErrorEnumRename),
        "enum-variant-rename" => Box::new(EnumVariantRename),
        "struct-field-rename" => Box::new(StructFieldRename),
        "blocklisted-type-implements-trait" => {
            Box::new(BlocklistedTypeImplementsTrait)
        }
        "wrap-as-variadic-fn" => Box::new(WrapAsVariadicFn),
        "type-visibility" => Box::new(TypeVisibility),
        "operator-rename" => Box::new(OperatorRename),
        call_back => {
            if let Some(prefix) =
                call_back.strip_prefix("remove-function-prefix-")
            {
                let lnopc = RemovePrefixParseCallback::new(prefix);
                Box::new(lnopc)
            } else if let Some(prefix) =
                call_back.strip_prefix("prefix-link-name-")
            {
                let plnpc = PrefixLinkNameParseCallback::new(prefix);
                Box::new(plnpc)
            } else if let Some(default) =
                call_back.strip_prefix("field-visibility-default-")
            {
                Box::new(FieldVisibility {
                    default: default.parse().expect(
                        "unable to parse field-visibility-default callback",
                    ),
                })
            } else {
                panic!("Couldn't find name ParseCallbacks: {cb}")
            }
        }
    }
}
