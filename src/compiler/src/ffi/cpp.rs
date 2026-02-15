use crate::ast::GenericParam;
use crate::sema::Type;

pub struct CppTemplateInterop {
    pub namespace_prefix: String,
}

impl CppTemplateInterop {
    pub fn new() -> Self {
        Self {
            namespace_prefix: "korlang_cpp".to_string(),
        }
    }

    /// generates the C++ mangled name for a generic instantiation
    pub fn mangle_instantiation(&self, name: &str, args: &[Type]) -> String {
        let mut mangled = format!("{}::{}", self.namespace_prefix, name);
        mangled.push('<');
        for (i, arg) in args.iter().enumerate() {
            if i > 0 { mangled.push_str(", "); }
            mangled.push_str(&self.type_to_cpp(arg));
        }
        mangled.push('>');
        mangled
    }

    pub fn generate_header(&self, structs: &[crate::ast::StructDecl], interfaces: &[crate::ast::InterfaceDecl]) -> String {
        let mut header = format!("#pragma once\n#include <string>\n#include <vector>\n#include <cstdint>\n\nnamespace {} {{\n\n", self.namespace_prefix);
        
        for s in structs {
            header.push_str(&format!("struct {} {{\n", s.name));
            for field in &s.fields {
                header.push_str(&format!("    {} {};\n", self.type_ref_to_cpp(&field.ty), field.name));
            }
            header.push_str("};\n\n");
        }

        for i in interfaces {
            header.push_str(&format!("class {} {{\npublic:\n    virtual ~{}() = default;\n", i.name, i.name));
            for method in &i.methods {
                header.push_str(&format!("    virtual void {}() = 0;\n", method.name));
            }
            header.push_str("};\n\n");
        }

        header.push_str(&format!("}} // namespace {}\n", self.namespace_prefix));
        header
    }

    fn type_ref_to_cpp(&self, tr: &crate::ast::TypeRef) -> String {
        match tr {
            crate::ast::TypeRef::Named(n, _, _) => {
                match n.as_str() {
                    "Int" => "int64_t".to_string(),
                    "Float" => "double".to_string(),
                    "String" => "std::string".to_string(),
                    _ => n.clone(),
                }
            }
            _ => "void*".to_string(),
        }
    }

    fn type_to_cpp(&self, ty: &Type) -> String {
        match ty {
            Type::Int => "int64_t".to_string(),
            Type::Float => "double".to_string(),
            Type::String => "std::string".to_string(),
            Type::Named(n) => n.clone(),
            _ => "void*".to_string(),
        }
    }
}
