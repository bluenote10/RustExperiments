mod component_children {
    use leptos::*;
    ///Props for the [`TakesChildren`] component.
    ///
    ///# Required Props
    ///- **cx**: [`Scope`]
    ///- **children**: [`Children`]
    #[builder(doc)]
    struct TakesChildrenProps {
        #[builder(setter(doc = "**children**: [`Children`]"))]
        #[builder()]
        children: Children,
    }
    impl TakesChildrenProps {
        /**
                Create a builder for building `TakesChildrenProps`.
                On the builder, call `.children(...)` to set the values of the fields.
                Finally, call `.build()` to create the instance of `TakesChildrenProps`.
                */
        #[allow(dead_code, clippy::default_trait_access)]
        fn builder() -> TakesChildrenPropsBuilder<((),)> {
            TakesChildrenPropsBuilder {
                fields: ((),),
                phantom: ::core::default::Default::default(),
            }
        }
    }
    #[must_use]
    /**Builder for [`TakesChildrenProps`] instances.

See [`TakesChildrenProps::builder()`] for more info.*/
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    struct TakesChildrenPropsBuilder<TypedBuilderFields = ((),)> {
        fields: TypedBuilderFields,
        phantom: (),
    }
    impl<TypedBuilderFields> Clone for TakesChildrenPropsBuilder<TypedBuilderFields>
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
    pub trait TakesChildrenPropsBuilder_Optional<T> {
        fn into_value<F: FnOnce() -> T>(self, default: F) -> T;
    }
    impl<T> TakesChildrenPropsBuilder_Optional<T> for () {
        fn into_value<F: FnOnce() -> T>(self, default: F) -> T {
            default()
        }
    }
    impl<T> TakesChildrenPropsBuilder_Optional<T> for (T,) {
        fn into_value<F: FnOnce() -> T>(self, _: F) -> T {
            self.0
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl TakesChildrenPropsBuilder<((),)> {
        ///**children**: [`Children`]
        pub fn children(
            self,
            children: Children,
        ) -> TakesChildrenPropsBuilder<((Children,),)> {
            let children = (children,);
            let (_,) = self.fields;
            TakesChildrenPropsBuilder {
                fields: (children,),
                phantom: self.phantom,
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum TakesChildrenPropsBuilder_Error_Repeated_field_children {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl TakesChildrenPropsBuilder<((Children,),)> {
        #[deprecated(note = "Repeated field children")]
        pub fn children(
            self,
            _: TakesChildrenPropsBuilder_Error_Repeated_field_children,
        ) -> TakesChildrenPropsBuilder<((Children,),)> {
            self
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum TakesChildrenPropsBuilder_Error_Missing_required_field_children {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
    impl TakesChildrenPropsBuilder<((),)> {
        #[deprecated(note = "Missing required field children")]
        pub fn build(
            self,
            _: TakesChildrenPropsBuilder_Error_Missing_required_field_children,
        ) -> TakesChildrenProps {
            { ::std::rt::begin_panic("explicit panic") };
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl TakesChildrenPropsBuilder<((Children,),)> {
        ///Finalise the builder and create its [`TakesChildrenProps`] instance
        #[allow(clippy::default_trait_access)]
        pub fn build(self) -> TakesChildrenProps {
            let (children,) = self.fields;
            let children = children.0;
            TakesChildrenProps { children }.into()
        }
    }
    ///# Required Props
    ///- **cx**: [`Scope`]
    ///- **children**: [`Children`]
    #[allow(non_snake_case, clippy::too_many_arguments)]
    fn TakesChildren(
        #[allow(unused_variables)]
        cx: ::leptos::Scope,
        props: TakesChildrenProps,
    ) -> impl IntoView {
        fn __TakesChildren(cx: Scope, children: Children) -> impl IntoView {
            leptos::leptos_dom::html::div(cx)
                .child((cx, #[allow(unused_braces)] { children(cx) }))
                .with_view_marker(
                    "reference_expand-src-examples-component_children.rs-5",
                )
        }
        let TakesChildrenProps { children } = props;
        ::leptos::leptos_dom::Component::new(
            "TakesChildren",
            move |cx| { __TakesChildren(cx, children) },
        )
    }
    fn main() {
        mount_to_body(|cx| {
            TakesChildren(
                cx,
                TakesChildrenProps::builder()
                    .children({
                        Box::new(move |cx| {
                            {
                                leptos::Fragment::lazy(|| <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        leptos::leptos_dom::html::p(cx)
                                            .child((cx, #[allow(unused_braces)] "first"))
                                            .into_view(cx),
                                        leptos::leptos_dom::html::p(cx)
                                            .child((cx, #[allow(unused_braces)] "second"))
                                            .into_view(cx),
                                    ]),
                                ))
                            }
                                .with_view_marker("<TakesChildren/>-children")
                        })
                    })
                    .build(),
            )
        })
    }
}
