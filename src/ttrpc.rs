// This file is generated by rust-protobuf 2.14.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `ttrpc.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_14_0;

#[derive(PartialEq,Clone,Default)]
pub struct Request {
    // message fields
    pub service: ::std::string::String,
    pub method: ::std::string::String,
    pub payload: ::std::vec::Vec<u8>,
    pub timeout_nano: i64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Request {
    fn default() -> &'a Request {
        <Request as ::protobuf::Message>::default_instance()
    }
}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    // string service = 1;


    pub fn get_service(&self) -> &str {
        &self.service
    }
    pub fn clear_service(&mut self) {
        self.service.clear();
    }

    // Param is passed by value, moved
    pub fn set_service(&mut self, v: ::std::string::String) {
        self.service = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service(&mut self) -> &mut ::std::string::String {
        &mut self.service
    }

    // Take field
    pub fn take_service(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.service, ::std::string::String::new())
    }

    // string method = 2;


    pub fn get_method(&self) -> &str {
        &self.method
    }
    pub fn clear_method(&mut self) {
        self.method.clear();
    }

    // Param is passed by value, moved
    pub fn set_method(&mut self, v: ::std::string::String) {
        self.method = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_method(&mut self) -> &mut ::std::string::String {
        &mut self.method
    }

    // Take field
    pub fn take_method(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.method, ::std::string::String::new())
    }

    // bytes payload = 3;


    pub fn get_payload(&self) -> &[u8] {
        &self.payload
    }
    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.payload
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.payload, ::std::vec::Vec::new())
    }

    // int64 timeout_nano = 4;


    pub fn get_timeout_nano(&self) -> i64 {
        self.timeout_nano
    }
    pub fn clear_timeout_nano(&mut self) {
        self.timeout_nano = 0;
    }

    // Param is passed by value, moved
    pub fn set_timeout_nano(&mut self, v: i64) {
        self.timeout_nano = v;
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.service)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.method)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.payload)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.timeout_nano = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.service.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.service);
        }
        if !self.method.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.method);
        }
        if !self.payload.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.payload);
        }
        if self.timeout_nano != 0 {
            my_size += ::protobuf::rt::value_size(4, self.timeout_nano, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.service.is_empty() {
            os.write_string(1, &self.service)?;
        }
        if !self.method.is_empty() {
            os.write_string(2, &self.method)?;
        }
        if !self.payload.is_empty() {
            os.write_bytes(3, &self.payload)?;
        }
        if self.timeout_nano != 0 {
            os.write_int64(4, self.timeout_nano)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "service",
                    |m: &Request| { &m.service },
                    |m: &mut Request| { &mut m.service },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "method",
                    |m: &Request| { &m.method },
                    |m: &mut Request| { &mut m.method },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "payload",
                    |m: &Request| { &m.payload },
                    |m: &mut Request| { &mut m.payload },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "timeout_nano",
                    |m: &Request| { &m.timeout_nano },
                    |m: &mut Request| { &mut m.timeout_nano },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(Request::new)
        }
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.service.clear();
        self.method.clear();
        self.payload.clear();
        self.timeout_nano = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Request {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Any {
    // message fields
    pub type_url: ::std::string::String,
    pub value: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Any {
    fn default() -> &'a Any {
        <Any as ::protobuf::Message>::default_instance()
    }
}

impl Any {
    pub fn new() -> Any {
        ::std::default::Default::default()
    }

    // string type_url = 1;


    pub fn get_type_url(&self) -> &str {
        &self.type_url
    }
    pub fn clear_type_url(&mut self) {
        self.type_url.clear();
    }

    // Param is passed by value, moved
    pub fn set_type_url(&mut self, v: ::std::string::String) {
        self.type_url = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_type_url(&mut self) -> &mut ::std::string::String {
        &mut self.type_url
    }

    // Take field
    pub fn take_type_url(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.type_url, ::std::string::String::new())
    }

    // bytes value = 2;


    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Any {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.type_url)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.type_url.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.type_url);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.type_url.is_empty() {
            os.write_string(1, &self.type_url)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(2, &self.value)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Any {
        Any::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type_url",
                    |m: &Any| { &m.type_url },
                    |m: &mut Any| { &mut m.type_url },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    |m: &Any| { &m.value },
                    |m: &mut Any| { &mut m.value },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<Any>(
                    "Any",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Any {
        static mut instance: ::protobuf::lazy::Lazy<Any> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(Any::new)
        }
    }
}

impl ::protobuf::Clear for Any {
    fn clear(&mut self) {
        self.type_url.clear();
        self.value.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Any {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Any {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Status {
    // message fields
    pub code: Code,
    pub message: ::std::string::String,
    pub details: ::protobuf::RepeatedField<Any>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Status {
    fn default() -> &'a Status {
        <Status as ::protobuf::Message>::default_instance()
    }
}

impl Status {
    pub fn new() -> Status {
        ::std::default::Default::default()
    }

    // .grpc.Code code = 1;


    pub fn get_code(&self) -> Code {
        self.code
    }
    pub fn clear_code(&mut self) {
        self.code = Code::OK;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: Code) {
        self.code = v;
    }

    // string message = 2;


    pub fn get_message(&self) -> &str {
        &self.message
    }
    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }

    // repeated .grpc.Any details = 3;


    pub fn get_details(&self) -> &[Any] {
        &self.details
    }
    pub fn clear_details(&mut self) {
        self.details.clear();
    }

    // Param is passed by value, moved
    pub fn set_details(&mut self, v: ::protobuf::RepeatedField<Any>) {
        self.details = v;
    }

    // Mutable pointer to the field.
    pub fn mut_details(&mut self) -> &mut ::protobuf::RepeatedField<Any> {
        &mut self.details
    }

    // Take field
    pub fn take_details(&mut self) -> ::protobuf::RepeatedField<Any> {
        ::std::mem::replace(&mut self.details, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Status {
    fn is_initialized(&self) -> bool {
        for v in &self.details {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.code, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.message)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.details)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.code != Code::OK {
            my_size += ::protobuf::rt::enum_size(1, self.code);
        }
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.message);
        }
        for value in &self.details {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.code != Code::OK {
            os.write_enum(1, self.code.value())?;
        }
        if !self.message.is_empty() {
            os.write_string(2, &self.message)?;
        }
        for v in &self.details {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Status {
        Status::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Code>>(
                    "code",
                    |m: &Status| { &m.code },
                    |m: &mut Status| { &mut m.code },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    |m: &Status| { &m.message },
                    |m: &mut Status| { &mut m.message },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Any>>(
                    "details",
                    |m: &Status| { &m.details },
                    |m: &mut Status| { &mut m.details },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<Status>(
                    "Status",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Status {
        static mut instance: ::protobuf::lazy::Lazy<Status> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(Status::new)
        }
    }
}

impl ::protobuf::Clear for Status {
    fn clear(&mut self) {
        self.code = Code::OK;
        self.message.clear();
        self.details.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Status {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Status {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Response {
    // message fields
    pub status: ::protobuf::SingularPtrField<Status>,
    pub payload: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Response {
    fn default() -> &'a Response {
        <Response as ::protobuf::Message>::default_instance()
    }
}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    // .grpc.Status status = 1;


    pub fn get_status(&self) -> &Status {
        self.status.as_ref().unwrap_or_else(|| Status::default_instance())
    }
    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut Status {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> Status {
        self.status.take().unwrap_or_else(|| Status::new())
    }

    // bytes payload = 2;


    pub fn get_payload(&self) -> &[u8] {
        &self.payload
    }
    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.payload
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.payload, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        for v in &self.status {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.status)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.payload)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.status.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.payload.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.payload);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.status.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.payload.is_empty() {
            os.write_bytes(2, &self.payload)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Status>>(
                    "status",
                    |m: &Response| { &m.status },
                    |m: &mut Response| { &mut m.status },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "payload",
                    |m: &Response| { &m.payload },
                    |m: &mut Response| { &mut m.payload },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<Response>(
                    "Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Response {
        static mut instance: ::protobuf::lazy::Lazy<Response> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(Response::new)
        }
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.status.clear();
        self.payload.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Code {
    OK = 0,
    CANCELLED = 1,
    UNKNOWN = 2,
    INVALID_ARGUMENT = 3,
    DEADLINE_EXCEEDED = 4,
    NOT_FOUND = 5,
    ALREADY_EXISTS = 6,
    PERMISSION_DENIED = 7,
    UNAUTHENTICATED = 16,
    RESOURCE_EXHAUSTED = 8,
    FAILED_PRECONDITION = 9,
    ABORTED = 10,
    OUT_OF_RANGE = 11,
    UNIMPLEMENTED = 12,
    INTERNAL = 13,
    UNAVAILABLE = 14,
    DATA_LOSS = 15,
}

impl ::protobuf::ProtobufEnum for Code {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Code> {
        match value {
            0 => ::std::option::Option::Some(Code::OK),
            1 => ::std::option::Option::Some(Code::CANCELLED),
            2 => ::std::option::Option::Some(Code::UNKNOWN),
            3 => ::std::option::Option::Some(Code::INVALID_ARGUMENT),
            4 => ::std::option::Option::Some(Code::DEADLINE_EXCEEDED),
            5 => ::std::option::Option::Some(Code::NOT_FOUND),
            6 => ::std::option::Option::Some(Code::ALREADY_EXISTS),
            7 => ::std::option::Option::Some(Code::PERMISSION_DENIED),
            16 => ::std::option::Option::Some(Code::UNAUTHENTICATED),
            8 => ::std::option::Option::Some(Code::RESOURCE_EXHAUSTED),
            9 => ::std::option::Option::Some(Code::FAILED_PRECONDITION),
            10 => ::std::option::Option::Some(Code::ABORTED),
            11 => ::std::option::Option::Some(Code::OUT_OF_RANGE),
            12 => ::std::option::Option::Some(Code::UNIMPLEMENTED),
            13 => ::std::option::Option::Some(Code::INTERNAL),
            14 => ::std::option::Option::Some(Code::UNAVAILABLE),
            15 => ::std::option::Option::Some(Code::DATA_LOSS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Code] = &[
            Code::OK,
            Code::CANCELLED,
            Code::UNKNOWN,
            Code::INVALID_ARGUMENT,
            Code::DEADLINE_EXCEEDED,
            Code::NOT_FOUND,
            Code::ALREADY_EXISTS,
            Code::PERMISSION_DENIED,
            Code::UNAUTHENTICATED,
            Code::RESOURCE_EXHAUSTED,
            Code::FAILED_PRECONDITION,
            Code::ABORTED,
            Code::OUT_OF_RANGE,
            Code::UNIMPLEMENTED,
            Code::INTERNAL,
            Code::UNAVAILABLE,
            Code::DATA_LOSS,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new_pb_name::<Code>("Code", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Code {
}

impl ::std::default::Default for Code {
    fn default() -> Self {
        Code::OK
    }
}

impl ::protobuf::reflect::ProtobufValue for Code {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bttrpc.proto\x12\x04grpc\"[\n\x07Request\x12\x11\n\x07service\x18\
    \x01\x20\x01(\tB\0\x12\x10\n\x06method\x18\x02\x20\x01(\tB\0\x12\x11\n\
    \x07payload\x18\x03\x20\x01(\x0cB\0\x12\x16\n\x0ctimeout_nano\x18\x04\
    \x20\x01(\x03B\0:\0\",\n\x03Any\x12\x12\n\x08type_url\x18\x01\x20\x01(\t\
    B\0\x12\x0f\n\x05value\x18\x02\x20\x01(\x0cB\0:\0\"W\n\x06Status\x12\x1a\
    \n\x04code\x18\x01\x20\x01(\x0e2\n.grpc.CodeB\0\x12\x11\n\x07message\x18\
    \x02\x20\x01(\tB\0\x12\x1c\n\x07details\x18\x03\x20\x03(\x0b2\t.grpc.Any\
    B\0:\0\"?\n\x08Response\x12\x1e\n\x06status\x18\x01\x20\x01(\x0b2\x0c.gr\
    pc.StatusB\0\x12\x11\n\x07payload\x18\x02\x20\x01(\x0cB\0:\0*\xb9\x02\n\
    \x04Code\x12\x06\n\x02OK\x10\0\x12\r\n\tCANCELLED\x10\x01\x12\x0b\n\x07U\
    NKNOWN\x10\x02\x12\x14\n\x10INVALID_ARGUMENT\x10\x03\x12\x15\n\x11DEADLI\
    NE_EXCEEDED\x10\x04\x12\r\n\tNOT_FOUND\x10\x05\x12\x12\n\x0eALREADY_EXIS\
    TS\x10\x06\x12\x15\n\x11PERMISSION_DENIED\x10\x07\x12\x13\n\x0fUNAUTHENT\
    ICATED\x10\x10\x12\x16\n\x12RESOURCE_EXHAUSTED\x10\x08\x12\x17\n\x13FAIL\
    ED_PRECONDITION\x10\t\x12\x0b\n\x07ABORTED\x10\n\x12\x10\n\x0cOUT_OF_RAN\
    GE\x10\x0b\x12\x11\n\rUNIMPLEMENTED\x10\x0c\x12\x0c\n\x08INTERNAL\x10\r\
    \x12\x0f\n\x0bUNAVAILABLE\x10\x0e\x12\r\n\tDATA_LOSS\x10\x0f\x1a\0B\0b\
    \x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
