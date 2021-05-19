macro_rules! build_rpc_methodcall_body {
    ( $self:ident . $rpcname:ident ( $( $arg:expr ),* ) ) => {
        {
            let args = vec![
                $(
                    match serde_json::to_value($arg) {
                        Ok(val) => val,
                        Err(_) => panic!("Invalid arg passed to {}", stringify!($rpcname)),
                    }
                ),*
            ];

            $self.make_request(stringify!($rpcname), args)
        }
    }
}
