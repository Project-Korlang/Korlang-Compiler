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
            let s_name = crate::symbols::lookup(s.name);
            header.push_str(&format!("struct {} {{\n", s_name));
            for field in &s.fields {
                let f_name = crate::symbols::lookup(field.name);
                header.push_str(&format!("    {} {};\n", self.type_ref_to_cpp(&field.ty), f_name));
            }
            header.push_str("};\n\n");
        }

        for i in interfaces {
            let i_name = crate::symbols::lookup(i.name);
            header.push_str(&format!("class {} {{\npublic:\n    virtual ~{}() = default;\n", i_name, i_name));
            for method in &i.methods {
                let m_name = crate::symbols::lookup(method.name);
                header.push_str(&format!("    virtual void {}() = 0;\n", m_name));
            }
            header.push_str("};\n\n");
        }

        header.push_str(&format!("}} // namespace {}\n", self.namespace_prefix));
        header
    }

    fn type_ref_to_cpp(&self, tr: &crate::ast::TypeRef) -> String {
        match tr {
            crate::ast::TypeRef::Named(n, _, _) => {
                let name_s = crate::symbols::lookup(*n);
                match name_s.as_str() {
                    "Int" => "int64_t".to_string(),
                    "Float" => "double".to_string(),
                    "String" => "std::string".to_string(),
                    _ => name_s,
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
            Type::Named(n) => crate::symbols::lookup(*n),
            _ => "void*".to_string(),
        }
    }
}
