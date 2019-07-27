pub mod bindings;

pub fn connect(name: *const bindings::std_string, address: *const bindings::std_string, rpc_port: ::std::os::raw::c_uint, stream_port: ::std::os::raw::c_uint) -> bindings::krpc_Client {
	bindings::krpc_connect(name, address, rpc_port, stream_port)
}