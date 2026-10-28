use windows_rdl::formatter::format;

#[test]
fn attributes_are_sorted_at_each_owner() {
    let source = r#"
        #[z] #[a] mod Test {
            #[library("test.dll")] #[arch(X64)]
            extern fn Call(
                #[out_ptr] #[out] result: *mut *mut i32,
                #[opt] #[r#in] value: i32
            ) -> #[z] #[a] i32;
            #[z] #[a] struct Outer {
                #[z] #[a] field: #[z] #[a] struct { value: u32, },
            }
        }
    "#;
    let expected = "\
#[a]
#[z]
mod Test {
    #[arch(X64)]
    #[library(\"test.dll\")]
    extern fn Call(#[out] #[out_ptr] result: *mut *mut i32, #[in] #[opt] value: i32) -> #[a] #[z] i32;
    #[a]
    #[z]
    struct Outer {
        #[a]
        #[z]
        field: #[a] #[z] struct {
            value: u32,
        },
    }
}
";
    assert_eq!(format(source), expected);
    assert_eq!(format(expected), expected);
}

#[test]
fn repeated_attributes_keep_their_order_and_arguments() {
    let source = r#"
        #[library("z.dll")] #[arch(X64)] #[library("a.dll")] #[arch(X86)]
        extern fn Call(
            #[writable(success, second)] #[readable(entry, first)]
            #[writable(entry, second)] value: *mut u8
        );
    "#;
    let expected = "\
#[arch(X64)]
#[arch(X86)]
#[library(\"z.dll\")]
#[library(\"a.dll\")]
extern fn Call(#[readable(entry, first)] #[writable(success, second)] #[writable(entry, second)] value: *mut u8);
";
    assert_eq!(format(source), expected);
    assert_eq!(format(expected), expected);
}

#[test]
fn qualified_names_and_parameter_boundaries() {
    let source = "extern fn Call(#[Z::A(2, 1)] #[A::Z] #[A::A] first: i32, \
                  #[out_ptr] #[out] second: *mut *mut i32);";
    assert_eq!(
        format(source),
        "extern fn Call(#[A::A] #[A::Z] #[Z::A(2, 1)] first: i32, \
         #[out] #[out_ptr] second: *mut *mut i32);\n"
    );
    assert_eq!(
        format("extern fn Call(#[out_ptr] #[out] value: *mut *mut i32);"),
        format("extern fn Call(#[out] #[out_ptr] value: *mut *mut i32);")
    );
}
