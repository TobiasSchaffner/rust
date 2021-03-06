// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use PanicStrategy;
use LinkerFlavor;
use target::{LinkArgs, TargetOptions};
use std::default::Default;
use std::ffi::OsString;
use std::env;

pub fn opts() -> TargetOptions {
    let l4re_lib_path = env::var_os("L4RE_LIB_PATH_RUST").unwrap_or(OsString::new()).into_string().unwrap();
    let mut pre_link_args = LinkArgs::new();
    pre_link_args.insert(LinkerFlavor::Ld, vec![
            format!("-T{}/main_stat.ld", l4re_lib_path),
            "--defsym=__executable_start=0x01000000".to_string(),
            "--defsym=__L4_KIP_ADDR__=0x6ffff000".to_string(),
            format!("{}/crt1.o", l4re_lib_path),
            format!("{}/crti.o", l4re_lib_path),
            format!("{}/crtbeginT.o", l4re_lib_path),
    ]);
    let mut post_link_args = LinkArgs::new();
    post_link_args.insert(LinkerFlavor::Ld, vec![
            format!("{}/l4f/libpthread.a", l4re_lib_path),
            format!("{}/l4f/libc_be_sig.a", l4re_lib_path),
            format!("{}/l4f/libc_be_sig_noop.a", l4re_lib_path),
            format!("{}/l4f/libc_be_socket_noop.a", l4re_lib_path),
            format!("{}/l4f/libc_be_fs_noop.a", l4re_lib_path),
            format!("{}/l4f/libc_be_sem_noop.a", l4re_lib_path),
            format!("{}/l4f/libl4re-vfs.o.a", l4re_lib_path),
            format!("{}/l4f/lib4re.a", l4re_lib_path),
            format!("{}/l4f/lib4re-util.a", l4re_lib_path),
            format!("{}/l4f/libc_support_misc.a", l4re_lib_path),
            format!("{}/l4f/libsupc++.a", l4re_lib_path),
            format!("{}/l4f/lib4shmc.a", l4re_lib_path),
            format!("{}/l4f/lib4re-c.a", l4re_lib_path),
            format!("{}/l4f/lib4re-c-util.a", l4re_lib_path),
            format!("{}/l4f/libgcc_eh.a", l4re_lib_path),
            format!("{}/l4f/libdl.a", l4re_lib_path),
            "--start-group".to_string(),
            format!("{}/l4f/libl4util.a", l4re_lib_path),
            format!("{}/l4f/libc_be_l4re.a", l4re_lib_path),
            format!("{}/l4f/libuc_c.a", l4re_lib_path),
            format!("{}/l4f/libc_be_l4refile.a", l4re_lib_path),
            "--end-group".to_string(),
            format!("{}/l4f/libl4sys.a", l4re_lib_path),
            "-gc-sections".to_string(),
            format!("{}/crtend.o", l4re_lib_path),
            format!("{}/crtn.o", l4re_lib_path),
    ]);

    TargetOptions {
        executables: true,
        has_elf_tls: false,
        exe_allocation_crate: None,
        panic_strategy: PanicStrategy::Abort,
        linker: "ld".to_string(),
        pre_link_args: pre_link_args,
        post_link_args: post_link_args,
        target_family: Some("unix".to_string()),
        .. Default::default()
    }
}
