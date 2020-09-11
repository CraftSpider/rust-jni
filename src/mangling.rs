
use crate::types::JType;

pub enum TypeSignature {
    Primitive(String),
    Class(String),
    Array(Box<TypeSignature>),
    Method(Vec<TypeSignature>, Box<TypeSignature>)
}

impl TypeSignature {
    pub fn mangled(&self) -> String {
        match self {
            TypeSignature::Primitive(name) => {
                primitive_symbol(name).into()
            }
            TypeSignature::Class(name) => {
                format!("L{};", name.replace(".", "/"))
            }
            TypeSignature::Array(name) => {
                let lower = name.mangled();
                format!("[{}", lower)
            }
            TypeSignature::Method(params, ret) => {
                let params: String = params.iter().map(|val| {
                    val.mangled()
                }).collect();
                let ret_mangle = ret.mangled();
                format!("({}){}", params, ret_mangle)
            }
        }
    }

    pub fn pretty(&self) -> String {
        match self {
            TypeSignature::Primitive(name) => {
                name.into()
            }
            TypeSignature::Class(name) => {
                name.into()
            }
            TypeSignature::Array(name) => {
                name.pretty() + "[]"
            }
            TypeSignature::Method(params, ret) => {
                let params: Vec<String> = params
                    .iter()
                    .map(|val| {val.pretty()})
                    .collect();

                format!(
                    "({}) -> {}",
                    params.join(", "),
                    ret.pretty()
                )
            }
        }
    }

    pub fn java_type(&self) -> JType {
        match self {
            TypeSignature::Primitive(name) => {
                match name.as_ref() {
                    "boolean" => {
                        JType::Boolean
                    }
                    "byte" => {
                        JType::Byte
                    }
                    "char" => {
                        JType::Char
                    }
                    "short" => {
                        JType::Short
                    }
                    "int" => {
                        JType::Int
                    }
                    "long" => {
                        JType::Long
                    }
                    "float" => {
                        JType::Float
                    }
                    "double" => {
                        JType::Double
                    }
                    "void" => {
                        JType::Void
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
            TypeSignature::Class(_) => {
                JType::Object
            }
            TypeSignature::Array(_) => {
                JType::Object
            }
            TypeSignature::Method(_, _) => {
                unimplemented!()
            }
        }
    }
}

fn is_primitive(name: &str) -> bool {
    name == "boolean" ||
        name == "byte" ||
        name == "char" ||
        name == "short" ||
        name == "int" ||
        name == "long" ||
        name == "float" ||
        name == "double" ||
        name == "void"
}

fn primitive_symbol(name: &str) -> &str {
    match name {
        "boolean" => {
            "Z"
        }
        "byte" => {
            "B"
        }
        "char" => {
            "C"
        }
        "short" => {
            "S"
        }
        "int" => {
            "I"
        }
        "long" => {
            "J"
        }
        "float" => {
            "F"
        }
        "double" => {
            "D"
        }
        "void" => {
            "V"
        }
        _ => {
            ""
        }
    }
}


fn handle_args(args: &str) -> Vec<TypeSignature> {
    if args.len() > 2 {
        (&args[1..(args.len() - 1)]).split(",").map(&mangle_class).collect()
    } else {
        Vec::new()
    }
}


pub fn mangle_class(name: &str) -> TypeSignature {
    let name = name.trim();

    if is_primitive(name) {
        TypeSignature::Primitive(String::from(name))
    } else if name.starts_with("(") {
        if let Some(pos) = name.find("->") {
            let (args, ret) = name.split_at(pos);
            let args = args.trim();
            let ret = ret.trim();
            let args = handle_args(args);

            let ret = mangle_class(&ret[2..]);

            TypeSignature::Method(args, Box::new(ret))
        } else {
            panic!("Invalid class to mangle")
        }
    } else if name.ends_with("[]") {
        TypeSignature::Array(
            Box::new(
                mangle_class(&name[..(name.len() - 2)])
            )
        )
    } else {
        TypeSignature::Class(String::from(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mangle() {
        assert_eq!(mangle_class("int").mangled(), "I");
        assert_eq!(mangle_class("int[]").mangled(), "[I");
        assert_eq!(mangle_class("java.lang.Object").mangled(), "Ljava/lang/Object;");
        assert_eq!(mangle_class("java.lang.String[]").mangled(), "[Ljava/lang/String;");
        assert_eq!(mangle_class("(java.lang.Object, int) -> java.lang.String").mangled(), "(Ljava/lang/Object;I)Ljava/lang/String;");
        assert_eq!(mangle_class("(int, long[], java.lang.ArrayList) -> void").mangled(), "(I[JLjava/lang/ArrayList;)V");
        assert_eq!(mangle_class("() -> int").mangled(), "()I");
    }

}
