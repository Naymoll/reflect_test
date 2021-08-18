extern crate proc_macro;
use proc_macro::TokenStream;

reflect::library! {
    extern crate test_trait {
        trait TestTrait {
            fn do_something(&self) -> std::string::String;
        }
    }

    extern crate std {
        mod string {
            type String;

            impl String {
                fn new() -> String;
                fn push_str(&mut self, &str);
            }
        }
    }
}

#[proc_macro_derive(TestTrait)]
pub fn derive(input: TokenStream) -> TokenStream {
    reflect::derive(input, |ex| {
        ex.make_trait_impl(RUNTIME::test_trait::TestTrait, ex.target_type(), |block| {
            block.make_function(RUNTIME::test_trait::TestTrait::do_something, do_something)
        });
    })
}

fn do_something(m: reflect::MakeFunction) -> reflect::Value {
    let target = m.arg(0);

    match target.data() {
        reflect::Data::Struct(receiver) => match receiver {
            reflect::Struct::Struct(a) => {
                let string = RUNTIME::std::string::String::new
                    .INVOKE()
                    .reference_mut();
                
                for field in a.fields() { 
                    RUNTIME::std::string::String::push_str
                        .INVOKE(string, field.get_name());
                }

                string.dereference()
            },
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
} 
