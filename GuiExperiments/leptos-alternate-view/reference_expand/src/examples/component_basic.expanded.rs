mod component_basic {
    use leptos::*;
    ///Props for the [`Main`] component.
    ///
    ///# Required Props
    ///- **cx**: [`Scope`]
    ///- **some_int**: [`i32`]
    #[builder(doc)]
    struct MainProps {
        #[builder(setter(doc = "**some_int**: [`i32`]"))]
        #[builder()]
        some_int: i32,
    }
    impl MainProps {
        /**
                Create a builder for building `MainProps`.
                On the builder, call `.some_int(...)` to set the values of the fields.
                Finally, call `.build()` to create the instance of `MainProps`.
                */
        #[allow(dead_code, clippy::default_trait_access)]
        fn builder() -> MainPropsBuilder<((),)> {
            MainPropsBuilder {
                fields: ((),),
                phantom: ::core::default::Default::default(),
            }
        }
    }
    #[must_use]
    /**Builder for [`MainProps`] instances.

See [`MainProps::builder()`] for more info.*/
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    struct MainPropsBuilder<TypedBuilderFields = ((),)> {
        fields: TypedBuilderFields,
        phantom: (),
    }
    impl<TypedBuilderFields> Clone for MainPropsBuilder<TypedBuilderFields>
    where
        TypedBuilderFields: Clone,
    {
        #[allow(clippy::default_trait_access)]
        fn clone(&self) -> Self {
            Self {
                fields: self.fields.clone(),
                phantom: ::core::default::Default::default(),
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub trait MainPropsBuilder_Optional<T> {
        fn into_value<F: FnOnce() -> T>(self, default: F) -> T;
    }
    impl<T> MainPropsBuilder_Optional<T> for () {
        fn into_value<F: FnOnce() -> T>(self, default: F) -> T {
            default()
        }
    }
    impl<T> MainPropsBuilder_Optional<T> for (T,) {
        fn into_value<F: FnOnce() -> T>(self, _: F) -> T {
            self.0
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl MainPropsBuilder<((),)> {
        ///**some_int**: [`i32`]
        pub fn some_int(self, some_int: i32) -> MainPropsBuilder<((i32,),)> {
            let some_int = (some_int,);
            let (_,) = self.fields;
            MainPropsBuilder {
                fields: (some_int,),
                phantom: self.phantom,
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum MainPropsBuilder_Error_Repeated_field_some_int {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl MainPropsBuilder<((i32,),)> {
        #[deprecated(note = "Repeated field some_int")]
        pub fn some_int(
            self,
            _: MainPropsBuilder_Error_Repeated_field_some_int,
        ) -> MainPropsBuilder<((i32,),)> {
            self
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum MainPropsBuilder_Error_Missing_required_field_some_int {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
    impl MainPropsBuilder<((),)> {
        #[deprecated(note = "Missing required field some_int")]
        pub fn build(
            self,
            _: MainPropsBuilder_Error_Missing_required_field_some_int,
        ) -> MainProps {
            { ::std::rt::begin_panic("explicit panic") };
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl MainPropsBuilder<((i32,),)> {
        ///Finalise the builder and create its [`MainProps`] instance
        #[allow(clippy::default_trait_access)]
        pub fn build(self) -> MainProps {
            let (some_int,) = self.fields;
            let some_int = some_int.0;
            MainProps { some_int }.into()
        }
    }
    ///# Required Props
    ///- **cx**: [`Scope`]
    ///- **some_int**: [`i32`]
    #[allow(non_snake_case, clippy::too_many_arguments)]
    fn Main(
        #[allow(unused_variables)]
        cx: ::leptos::Scope,
        props: MainProps,
    ) -> impl IntoView {
        fn __Main(cx: Scope, some_int: i32) -> impl IntoView {
            leptos::leptos_dom::html::div(cx)
                .child((cx, #[allow(unused_braces)] "Hello world"))
                .with_view_marker("reference_expand-src-examples-component_basic.rs-5")
        }
        let MainProps { some_int } = props;
        ::leptos::leptos_dom::Component::new("Main", move |cx| { __Main(cx, some_int) })
    }
    fn main() {
        mount_to_body(|cx| {
            Main(cx, MainProps::builder().some_int(#[allow(unused_braces)] 42).build())
        })
    }
}
