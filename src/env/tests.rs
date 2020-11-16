use super::*;
use crate::tests::with_env;

#[test]
fn test_get_version() {
    with_env(|env| {
        assert_eq!(env.get_version(), JNIVersion::Ver18);
    })
}

#[test]
fn test_define_class() {
    with_env(|env| {
        let cls_ldr_cls = env.find_class("java.lang.ClassLoader").unwrap();
        let get_ldr_id = env.get_static_method_id(&cls_ldr_cls, "getSystemClassLoader", "() -> java.lang.ClassLoader").unwrap();
        let cls_ldr = env.call_method(&cls_ldr_cls.downcast(), &get_ldr_id, &vec![])
            .unwrap()
            .unwrap()
            .into_obj()
            .unwrap()
            .unwrap();

        env.define_class(
            "TestClass",
            &cls_ldr,
            b"\xca\xfe\xba\xbe\x00\x00\x00\x34\x00\x0d\x0a\x00\x03\x00\x0a\x07\x00\x0b\x07\
\x00\x0c\x01\x00\x06\x3c\x69\x6e\x69\x74\x3e\x01\x00\x03\x28\x29\x56\x01\x00\x04\x43\x6f\x64\x65\
\x01\x00\x0f\x4c\x69\x6e\x65\x4e\x75\x6d\x62\x65\x72\x54\x61\x62\x6c\x65\x01\x00\x0a\x53\x6f\x75\
\x72\x63\x65\x46\x69\x6c\x65\x01\x00\x0e\x54\x65\x73\x74\x43\x6c\x61\x73\x73\x2e\x6a\x61\x76\x61\
\x0c\x00\x04\x00\x05\x01\x00\x09\x54\x65\x73\x74\x43\x6c\x61\x73\x73\x01\x00\x10\x6a\x61\x76\x61\
\x2f\x6c\x61\x6e\x67\x2f\x4f\x62\x6a\x65\x63\x74\x00\x21\x00\x02\x00\x03\x00\x00\x00\x00\x00\x01\
\x00\x01\x00\x04\x00\x05\x00\x01\x00\x06\x00\x00\x00\x1d\x00\x01\x00\x01\x00\x00\x00\x05\x2a\xb7\
\x00\x01\xb1\x00\x00\x00\x01\x00\x07\x00\x00\x00\x06\x00\x01\x00\x00\x00\x02\x00\x01\x00\x08\x00\
\x00\x00\x02\x00\x09"
        ).expect("Couldn't define new test class");
    })
}

#[test]
fn test_find_class() {
    with_env(|env| {
        env.find_class("java.lang.String").expect("Couldn't get java.lang.String in test");
        env.find_class("java.lang.Integer[]").expect("Couldn't get java.lang.Integer[] in test");
        env.find_class("int[]").expect("Couldn't get int[] in test");
    })
}

#[test]
fn test_from_reflected_method() {
    with_env(|env| {
        let cls = env.find_class("java.lang.String").unwrap();
        let id = env.get_method_id(&cls, "isEmpty", "() -> boolean").unwrap();
        let method = env.to_reflected_method(&cls, &id, false).unwrap();

        let new_id = env.from_reflected_method(&method).expect("Couldn't make ID from reflected method");

        assert_eq!(id, new_id);
    });
}

#[test]
fn test_from_reflected_field() {
    with_env(|env| {
        let cls = env.find_class("java.lang.String").unwrap();
        let id = env.get_static_field_id(&cls, "CASE_INSENSITIVE_ORDER", "java.util.Comparator").unwrap();
        let field = env.to_reflected_field(&cls, &id, true).unwrap();

        let new_id = env.from_reflected_field(&field).expect("Couldn't make ID from reflected field");

        assert_eq!(id, new_id);
    });
}

#[test]
fn test_to_reflected_method() {
    with_env(|env| {
        let cls = env.find_class("java.lang.String").unwrap();
        let id = env.get_method_id(&cls, "isEmpty", "() -> boolean").unwrap();

        env.to_reflected_method(&cls, &id, false).expect("Couldn't get reflected method of String.isEmpty()");
    });
}

#[test]
fn test_to_reflected_field() {
    with_env(|env| {
        let cls = env.find_class("java.lang.String").unwrap();
        let id = env.get_static_field_id(&cls, "CASE_INSENSITIVE_ORDER", "java.util.Comparator").unwrap();

        env.to_reflected_field(&cls, &id, true).expect("Couldn't get reflected field of String.CASE_INSENSITIVE_ORDER");
    });
}

#[test]
fn test_get_superclass() {
    with_env(|env| {
        let cls = env.find_class("java.lang.String").unwrap();
        let obj_cls = env.find_class("java.lang.Object").unwrap();

        let super_cls = env.get_superclass(&cls).expect("Couldn't get String superclass");
        assert!(env.is_same_object(&obj_cls.downcast(), &super_cls.downcast()));
    });
}

#[test]
fn test_is_assignable_from() {
    with_env(|env| {
        let obj_cls = env.find_class("java.lang.Object").unwrap();
        let str_cls = env.find_class("java.lang.String").unwrap();

        assert!(env.is_assignable_from(&str_cls, &obj_cls));
        assert!(!env.is_assignable_from(&obj_cls, &str_cls));
    })
}

#[test]
fn test_throw_family() {
    with_env(|env| {
        let exc_cls = env.find_class("java.lang.RuntimeException").unwrap();
        let con_id = env.get_method_id(&exc_cls, "<init>", "(java.lang.String) -> void").unwrap();
        let str = env.new_string_utf("Example Exception").unwrap();
        let exc: JThrowable = unsafe { env.new_object(&exc_cls, &con_id, &vec![str.downcast().into()]).unwrap().upcast_raw() };

        env.throw(&exc).expect("Couldn't throw exception");
        assert!(env.exception_check());
        let new_exc = env.exception_occurred().unwrap();
        assert!(env.is_same_object(&exc.downcast(), &new_exc.downcast()));
        env.exception_clear().expect("Couldn't clear exception");

        assert!(!env.exception_check());

        env.throw_new(&exc_cls, "Example Exception").expect("Couldn't throw new exception");
        assert!(env.exception_check());
        env.exception_clear().expect("Couldn't clear exception");

        assert!(!env.exception_check());
    });
}

// Can't test fatal_error, it exits the program?

#[test]
fn test_ensure_local_capacity() {
    with_env(|env| {
        env.ensure_local_capacity(100).expect("Couldn't ensure capacity");
    });
}

#[test]
#[ignore = "Not yet implemented"]
fn test_local_frame() {
    todo!()
}

#[test]
#[ignore = "Not yet implemented"]
fn test_global_ref() {
    todo!()
}

#[test]
#[ignore = "Not yet implemented"]
fn test_local_ref() {
    todo!()
}
