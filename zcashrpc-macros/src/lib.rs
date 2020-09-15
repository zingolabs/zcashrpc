#[proc_macro]
pub fn declare_rpc_client_methods(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let calls_with_args = conjoin_calls_with_args(
        proc_macro2::TokenStream::from(input).into_iter(),
        Vec::new(),
    );
    let calls_as_code: Vec<proc_macro2::TokenStream> =
        calls_with_args.into_iter().map(make_call).collect();
    quote::quote!(#(#calls_as_code)*).into()
}

type CallsWithArgs = Vec<(proc_macro2::Ident, proc_macro2::Group)>;

fn conjoin_calls_with_args(
    mut iter: proc_macro2::token_stream::IntoIter,
    mut vec: CallsWithArgs,
) -> CallsWithArgs {
    match iter.next() {
        Some(proc_macro2::TokenTree::Ident(i)) => {
            vec.push((
                i,
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Parenthesis,
                    proc_macro2::TokenStream::new(),
                ),
            ));
            conjoin_calls_with_args(iter, vec)
        }
        Some(proc_macro2::TokenTree::Group(n)) => {
            let ident = vec.last().unwrap().0.clone();
            vec.pop();
            vec.push((ident, n));
            conjoin_calls_with_args(iter, vec)
        }
        Some(proc_macro2::TokenTree::Punct(_)) => {
            conjoin_calls_with_args(iter, vec)
        }
        Some(proc_macro2::TokenTree::Literal(l)) => {
            panic!("Unexpected literal '{}' in macro input.", l)
        }
        None => vec,
    }
}

fn make_call(
    input: (proc_macro2::Ident, proc_macro2::Group),
) -> proc_macro2::TokenStream {
    let (ident, typed_args) = input;
    let mut call_ident_string = ident.to_string().to_lowercase();
    if call_ident_string.starts_with('z') {
        call_ident_string.insert(1, '_');
    }
    let call_ident = proc_macro2::Ident::new(&call_ident_string, ident.span());
    let response_ident = proc_macro2::Ident::new(
        &format!("{}Response", ident.to_string()),
        ident.span(),
    );
    let typed_args_as_stream = typed_args.stream();
    let untyped_args_as_stream: proc_macro2::TokenStream =
        typed_args.stream().into_iter().step_by(4).collect();
    quote::quote!(
        pub fn #call_ident(&mut self, #typed_args_as_stream) -> impl Future<Output = ResponseResult<#response_ident>> {
            rpc_call!(self.#call_ident(#untyped_args_as_stream))
        }
    ).into()
}
