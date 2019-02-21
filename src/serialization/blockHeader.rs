// This file is generated by rust-protobuf 2.0.4. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct BlockHeader {
    // message fields
    pub previousHash: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    pub merkleRoot: ::std::vec::Vec<u8>,
    pub stateRoot: ::std::vec::Vec<u8>,
    pub difficulty: f64,
    pub timeStamp: u64,
    pub nonce: u64,
    pub miner: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl BlockHeader {
    pub fn new() -> BlockHeader {
        ::std::default::Default::default()
    }

    // repeated bytes previousHash = 1;

    pub fn clear_previousHash(&mut self) {
        self.previousHash.clear();
    }

    // Param is passed by value, moved
    pub fn set_previousHash(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.previousHash = v;
    }

    // Mutable pointer to the field.
    pub fn mut_previousHash(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.previousHash
    }

    // Take field
    pub fn take_previousHash(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.previousHash, ::protobuf::RepeatedField::new())
    }

    pub fn get_previousHash(&self) -> &[::std::vec::Vec<u8>] {
        &self.previousHash
    }

    // bytes merkleRoot = 2;

    pub fn clear_merkleRoot(&mut self) {
        self.merkleRoot.clear();
    }

    // Param is passed by value, moved
    pub fn set_merkleRoot(&mut self, v: ::std::vec::Vec<u8>) {
        self.merkleRoot = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_merkleRoot(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.merkleRoot
    }

    // Take field
    pub fn take_merkleRoot(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.merkleRoot, ::std::vec::Vec::new())
    }

    pub fn get_merkleRoot(&self) -> &[u8] {
        &self.merkleRoot
    }

    // bytes stateRoot = 3;

    pub fn clear_stateRoot(&mut self) {
        self.stateRoot.clear();
    }

    // Param is passed by value, moved
    pub fn set_stateRoot(&mut self, v: ::std::vec::Vec<u8>) {
        self.stateRoot = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stateRoot(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.stateRoot
    }

    // Take field
    pub fn take_stateRoot(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.stateRoot, ::std::vec::Vec::new())
    }

    pub fn get_stateRoot(&self) -> &[u8] {
        &self.stateRoot
    }

    // double difficulty = 4;

    pub fn clear_difficulty(&mut self) {
        self.difficulty = 0.;
    }

    // Param is passed by value, moved
    pub fn set_difficulty(&mut self, v: f64) {
        self.difficulty = v;
    }

    pub fn get_difficulty(&self) -> f64 {
        self.difficulty
    }

    // uint64 timeStamp = 5;

    pub fn clear_timeStamp(&mut self) {
        self.timeStamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timeStamp(&mut self, v: u64) {
        self.timeStamp = v;
    }

    pub fn get_timeStamp(&self) -> u64 {
        self.timeStamp
    }

    // uint64 nonce = 6;

    pub fn clear_nonce(&mut self) {
        self.nonce = 0;
    }

    // Param is passed by value, moved
    pub fn set_nonce(&mut self, v: u64) {
        self.nonce = v;
    }

    pub fn get_nonce(&self) -> u64 {
        self.nonce
    }

    // bytes miner = 7;

    pub fn clear_miner(&mut self) {
        self.miner.clear();
    }

    // Param is passed by value, moved
    pub fn set_miner(&mut self, v: ::std::vec::Vec<u8>) {
        self.miner = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_miner(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.miner
    }

    // Take field
    pub fn take_miner(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.miner, ::std::vec::Vec::new())
    }

    pub fn get_miner(&self) -> &[u8] {
        &self.miner
    }
}

impl ::protobuf::Message for BlockHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.previousHash)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.merkleRoot)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.stateRoot)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.difficulty = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timeStamp = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.nonce = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.miner)?;
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
        for value in &self.previousHash {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        if !self.merkleRoot.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.merkleRoot);
        }
        if !self.stateRoot.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.stateRoot);
        }
        if true {
            my_size += 9;
        }
        if true {
            my_size += ::protobuf::rt::value_size(5, self.timeStamp, ::protobuf::wire_format::WireTypeVarint);
        }
        if true {
            my_size += ::protobuf::rt::value_size(6, self.nonce, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.miner.is_empty() {
            my_size += ::protobuf::rt::bytes_size(7, &self.miner);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.previousHash {
            os.write_bytes(1, &v)?;
        };
        if !self.merkleRoot.is_empty() {
            os.write_bytes(2, &self.merkleRoot)?;
        }
        if !self.stateRoot.is_empty() {
            os.write_bytes(3, &self.stateRoot)?;
        }
        if true {
            os.write_double(4, self.difficulty)?;
        }
        if true {
            os.write_uint64(5, self.timeStamp)?;
        }
        if true {
            os.write_uint64(6, self.nonce)?;
        }
        if !self.miner.is_empty() {
            os.write_bytes(7, &self.miner)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> BlockHeader {
        BlockHeader::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "previousHash",
                    |m: &BlockHeader| { &m.previousHash },
                    |m: &mut BlockHeader| { &mut m.previousHash },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "merkleRoot",
                    |m: &BlockHeader| { &m.merkleRoot },
                    |m: &mut BlockHeader| { &mut m.merkleRoot },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "stateRoot",
                    |m: &BlockHeader| { &m.stateRoot },
                    |m: &mut BlockHeader| { &mut m.stateRoot },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "difficulty",
                    |m: &BlockHeader| { &m.difficulty },
                    |m: &mut BlockHeader| { &mut m.difficulty },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timeStamp",
                    |m: &BlockHeader| { &m.timeStamp },
                    |m: &mut BlockHeader| { &mut m.timeStamp },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "nonce",
                    |m: &BlockHeader| { &m.nonce },
                    |m: &mut BlockHeader| { &mut m.nonce },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "miner",
                    |m: &BlockHeader| { &m.miner },
                    |m: &mut BlockHeader| { &mut m.miner },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockHeader>(
                    "BlockHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static BlockHeader {
        static mut instance: ::protobuf::lazy::Lazy<BlockHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockHeader,
        };
        unsafe {
            instance.get(BlockHeader::new)
        }
    }
}

impl ::protobuf::Clear for BlockHeader {
    fn clear(&mut self) {
        self.clear_previousHash();
        self.clear_merkleRoot();
        self.clear_stateRoot();
        self.clear_difficulty();
        self.clear_timeStamp();
        self.clear_nonce();
        self.clear_miner();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GenesisHeader {
    // message fields
    pub previousHash: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    pub merkleRoot: ::std::vec::Vec<u8>,
    pub stateRoot: ::std::vec::Vec<u8>,
    pub difficulty: f64,
    pub timeStamp: u64,
    pub miner: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl GenesisHeader {
    pub fn new() -> GenesisHeader {
        ::std::default::Default::default()
    }

    // repeated bytes previousHash = 1;

    pub fn clear_previousHash(&mut self) {
        self.previousHash.clear();
    }

    // Param is passed by value, moved
    pub fn set_previousHash(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.previousHash = v;
    }

    // Mutable pointer to the field.
    pub fn mut_previousHash(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.previousHash
    }

    // Take field
    pub fn take_previousHash(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.previousHash, ::protobuf::RepeatedField::new())
    }

    pub fn get_previousHash(&self) -> &[::std::vec::Vec<u8>] {
        &self.previousHash
    }

    // bytes merkleRoot = 2;

    pub fn clear_merkleRoot(&mut self) {
        self.merkleRoot.clear();
    }

    // Param is passed by value, moved
    pub fn set_merkleRoot(&mut self, v: ::std::vec::Vec<u8>) {
        self.merkleRoot = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_merkleRoot(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.merkleRoot
    }

    // Take field
    pub fn take_merkleRoot(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.merkleRoot, ::std::vec::Vec::new())
    }

    pub fn get_merkleRoot(&self) -> &[u8] {
        &self.merkleRoot
    }

    // bytes stateRoot = 3;

    pub fn clear_stateRoot(&mut self) {
        self.stateRoot.clear();
    }

    // Param is passed by value, moved
    pub fn set_stateRoot(&mut self, v: ::std::vec::Vec<u8>) {
        self.stateRoot = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stateRoot(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.stateRoot
    }

    // Take field
    pub fn take_stateRoot(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.stateRoot, ::std::vec::Vec::new())
    }

    pub fn get_stateRoot(&self) -> &[u8] {
        &self.stateRoot
    }

    // double difficulty = 4;

    pub fn clear_difficulty(&mut self) {
        self.difficulty = 0.;
    }

    // Param is passed by value, moved
    pub fn set_difficulty(&mut self, v: f64) {
        self.difficulty = v;
    }

    pub fn get_difficulty(&self) -> f64 {
        self.difficulty
    }

    // uint64 timeStamp = 5;

    pub fn clear_timeStamp(&mut self) {
        self.timeStamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timeStamp(&mut self, v: u64) {
        self.timeStamp = v;
    }

    pub fn get_timeStamp(&self) -> u64 {
        self.timeStamp
    }

    // bytes miner = 7;

    pub fn clear_miner(&mut self) {
        self.miner.clear();
    }

    // Param is passed by value, moved
    pub fn set_miner(&mut self, v: ::std::vec::Vec<u8>) {
        self.miner = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_miner(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.miner
    }

    // Take field
    pub fn take_miner(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.miner, ::std::vec::Vec::new())
    }

    pub fn get_miner(&self) -> &[u8] {
        &self.miner
    }
}

impl ::protobuf::Message for GenesisHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.previousHash)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.merkleRoot)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.stateRoot)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.difficulty = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timeStamp = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.miner)?;
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
        for value in &self.previousHash {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        if !self.merkleRoot.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.merkleRoot);
        }
        if !self.stateRoot.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.stateRoot);
        }
        if true {
            my_size += 9;
        }
        if true {
            my_size += ::protobuf::rt::value_size(5, self.timeStamp, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.miner.is_empty() {
            my_size += ::protobuf::rt::bytes_size(7, &self.miner);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.previousHash {
            os.write_bytes(1, &v)?;
        };
        if !self.merkleRoot.is_empty() {
            os.write_bytes(2, &self.merkleRoot)?;
        }
        if !self.stateRoot.is_empty() {
            os.write_bytes(3, &self.stateRoot)?;
        }
        if true {
            os.write_double(4, self.difficulty)?;
        }
        if true {
            os.write_uint64(5, self.timeStamp)?;
        }
        if !self.miner.is_empty() {
            os.write_bytes(7, &self.miner)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GenesisHeader {
        GenesisHeader::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "previousHash",
                    |m: &GenesisHeader| { &m.previousHash },
                    |m: &mut GenesisHeader| { &mut m.previousHash },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "merkleRoot",
                    |m: &GenesisHeader| { &m.merkleRoot },
                    |m: &mut GenesisHeader| { &mut m.merkleRoot },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "stateRoot",
                    |m: &GenesisHeader| { &m.stateRoot },
                    |m: &mut GenesisHeader| { &mut m.stateRoot },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "difficulty",
                    |m: &GenesisHeader| { &m.difficulty },
                    |m: &mut GenesisHeader| { &mut m.difficulty },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timeStamp",
                    |m: &GenesisHeader| { &m.timeStamp },
                    |m: &mut GenesisHeader| { &mut m.timeStamp },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "miner",
                    |m: &GenesisHeader| { &m.miner },
                    |m: &mut GenesisHeader| { &mut m.miner },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GenesisHeader>(
                    "GenesisHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static GenesisHeader {
        static mut instance: ::protobuf::lazy::Lazy<GenesisHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GenesisHeader,
        };
        unsafe {
            instance.get(GenesisHeader::new)
        }
    }
}

impl ::protobuf::Clear for GenesisHeader {
    fn clear(&mut self) {
        self.clear_previousHash();
        self.clear_merkleRoot();
        self.clear_stateRoot();
        self.clear_difficulty();
        self.clear_timeStamp();
        self.clear_miner();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GenesisHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GenesisHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HeaderPrehash {
    // message fields
    pub previousHash: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    pub merkleRoot: ::std::vec::Vec<u8>,
    pub stateRoot: ::std::vec::Vec<u8>,
    pub difficulty: f64,
    pub timeStamp: u64,
    pub miner: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl HeaderPrehash {
    pub fn new() -> HeaderPrehash {
        ::std::default::Default::default()
    }

    // repeated bytes previousHash = 1;

    pub fn clear_previousHash(&mut self) {
        self.previousHash.clear();
    }

    // Param is passed by value, moved
    pub fn set_previousHash(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.previousHash = v;
    }

    // Mutable pointer to the field.
    pub fn mut_previousHash(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.previousHash
    }

    // Take field
    pub fn take_previousHash(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.previousHash, ::protobuf::RepeatedField::new())
    }

    pub fn get_previousHash(&self) -> &[::std::vec::Vec<u8>] {
        &self.previousHash
    }

    // bytes merkleRoot = 2;

    pub fn clear_merkleRoot(&mut self) {
        self.merkleRoot.clear();
    }

    // Param is passed by value, moved
    pub fn set_merkleRoot(&mut self, v: ::std::vec::Vec<u8>) {
        self.merkleRoot = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_merkleRoot(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.merkleRoot
    }

    // Take field
    pub fn take_merkleRoot(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.merkleRoot, ::std::vec::Vec::new())
    }

    pub fn get_merkleRoot(&self) -> &[u8] {
        &self.merkleRoot
    }

    // bytes stateRoot = 3;

    pub fn clear_stateRoot(&mut self) {
        self.stateRoot.clear();
    }

    // Param is passed by value, moved
    pub fn set_stateRoot(&mut self, v: ::std::vec::Vec<u8>) {
        self.stateRoot = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stateRoot(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.stateRoot
    }

    // Take field
    pub fn take_stateRoot(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.stateRoot, ::std::vec::Vec::new())
    }

    pub fn get_stateRoot(&self) -> &[u8] {
        &self.stateRoot
    }

    // double difficulty = 4;

    pub fn clear_difficulty(&mut self) {
        self.difficulty = 0.;
    }

    // Param is passed by value, moved
    pub fn set_difficulty(&mut self, v: f64) {
        self.difficulty = v;
    }

    pub fn get_difficulty(&self) -> f64 {
        self.difficulty
    }

    // uint64 timeStamp = 5;

    pub fn clear_timeStamp(&mut self) {
        self.timeStamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timeStamp(&mut self, v: u64) {
        self.timeStamp = v;
    }

    pub fn get_timeStamp(&self) -> u64 {
        self.timeStamp
    }

    // bytes miner = 7;

    pub fn clear_miner(&mut self) {
        self.miner.clear();
    }

    // Param is passed by value, moved
    pub fn set_miner(&mut self, v: ::std::vec::Vec<u8>) {
        self.miner = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_miner(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.miner
    }

    // Take field
    pub fn take_miner(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.miner, ::std::vec::Vec::new())
    }

    pub fn get_miner(&self) -> &[u8] {
        &self.miner
    }
}

impl ::protobuf::Message for HeaderPrehash {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.previousHash)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.merkleRoot)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.stateRoot)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.difficulty = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timeStamp = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.miner)?;
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
        for value in &self.previousHash {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        if !self.merkleRoot.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.merkleRoot);
        }
        if !self.stateRoot.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.stateRoot);
        }
        if true {
            my_size += 9;
        }
        if true {
            my_size += ::protobuf::rt::value_size(5, self.timeStamp, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.miner.is_empty() {
            my_size += ::protobuf::rt::bytes_size(7, &self.miner);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.previousHash {
            os.write_bytes(1, &v)?;
        };
        if !self.merkleRoot.is_empty() {
            os.write_bytes(2, &self.merkleRoot)?;
        }
        if !self.stateRoot.is_empty() {
            os.write_bytes(3, &self.stateRoot)?;
        }
        if true {
            os.write_double(4, self.difficulty)?;
        }
        if true {
            os.write_uint64(5, self.timeStamp)?;
        }
        if !self.miner.is_empty() {
            os.write_bytes(7, &self.miner)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> HeaderPrehash {
        HeaderPrehash::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "previousHash",
                    |m: &HeaderPrehash| { &m.previousHash },
                    |m: &mut HeaderPrehash| { &mut m.previousHash },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "merkleRoot",
                    |m: &HeaderPrehash| { &m.merkleRoot },
                    |m: &mut HeaderPrehash| { &mut m.merkleRoot },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "stateRoot",
                    |m: &HeaderPrehash| { &m.stateRoot },
                    |m: &mut HeaderPrehash| { &mut m.stateRoot },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "difficulty",
                    |m: &HeaderPrehash| { &m.difficulty },
                    |m: &mut HeaderPrehash| { &mut m.difficulty },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timeStamp",
                    |m: &HeaderPrehash| { &m.timeStamp },
                    |m: &mut HeaderPrehash| { &mut m.timeStamp },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "miner",
                    |m: &HeaderPrehash| { &m.miner },
                    |m: &mut HeaderPrehash| { &mut m.miner },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HeaderPrehash>(
                    "HeaderPrehash",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static HeaderPrehash {
        static mut instance: ::protobuf::lazy::Lazy<HeaderPrehash> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HeaderPrehash,
        };
        unsafe {
            instance.get(HeaderPrehash::new)
        }
    }
}

impl ::protobuf::Clear for HeaderPrehash {
    fn clear(&mut self) {
        self.clear_previousHash();
        self.clear_merkleRoot();
        self.clear_stateRoot();
        self.clear_difficulty();
        self.clear_timeStamp();
        self.clear_miner();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HeaderPrehash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HeaderPrehash {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11blockHeader.proto\"\xd9\x01\n\x0bBlockHeader\x12\"\n\x0cpreviousHa\
    sh\x18\x01\x20\x03(\x0cR\x0cpreviousHash\x12\x1e\n\nmerkleRoot\x18\x02\
    \x20\x01(\x0cR\nmerkleRoot\x12\x1c\n\tstateRoot\x18\x03\x20\x01(\x0cR\ts\
    tateRoot\x12\x1e\n\ndifficulty\x18\x04\x20\x01(\x01R\ndifficulty\x12\x1c\
    \n\ttimeStamp\x18\x05\x20\x01(\x04R\ttimeStamp\x12\x14\n\x05nonce\x18\
    \x06\x20\x01(\x04R\x05nonce\x12\x14\n\x05miner\x18\x07\x20\x01(\x0cR\x05\
    miner\"\xc5\x01\n\rGenesisHeader\x12\"\n\x0cpreviousHash\x18\x01\x20\x03\
    (\x0cR\x0cpreviousHash\x12\x1e\n\nmerkleRoot\x18\x02\x20\x01(\x0cR\nmerk\
    leRoot\x12\x1c\n\tstateRoot\x18\x03\x20\x01(\x0cR\tstateRoot\x12\x1e\n\n\
    difficulty\x18\x04\x20\x01(\x01R\ndifficulty\x12\x1c\n\ttimeStamp\x18\
    \x05\x20\x01(\x04R\ttimeStamp\x12\x14\n\x05miner\x18\x07\x20\x01(\x0cR\
    \x05miner\"\xc5\x01\n\rHeaderPrehash\x12\"\n\x0cpreviousHash\x18\x01\x20\
    \x03(\x0cR\x0cpreviousHash\x12\x1e\n\nmerkleRoot\x18\x02\x20\x01(\x0cR\n\
    merkleRoot\x12\x1c\n\tstateRoot\x18\x03\x20\x01(\x0cR\tstateRoot\x12\x1e\
    \n\ndifficulty\x18\x04\x20\x01(\x01R\ndifficulty\x12\x1c\n\ttimeStamp\
    \x18\x05\x20\x01(\x04R\ttimeStamp\x12\x14\n\x05miner\x18\x07\x20\x01(\
    \x0cR\x05minerb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

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
