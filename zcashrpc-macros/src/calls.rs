pub fn make_call(
    input: (proc_macro2::Ident, proc_macro2::Group),
) -> proc_macro2::TokenStream {
    let (call_ident, response_ident, param_stream, arg_id_stream) =
        crate::utils::format_input("Response", input);
    quote::quote!(
        pub fn #call_ident(
            &mut self,
            #param_stream
        ) -> impl Future<Output = ResponseResult<#response_ident>> {
            build_rpc_methodcall_body!(self.#call_ident(#arg_id_stream))
        }
    )
    .into()
}
