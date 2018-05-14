#![no_std]

#[macro_use]
extern crate linux_kernel_module;

struct StaticFileSystemModule {
    _fs: linux_kernel_module::filesystem::FileSystemRegistration<StaticFileSystem>,
}

impl linux_kernel_module::KernelModule for StaticFileSystemModule {
    fn init() -> Result<Self, linux_kernel_module::Error> {
        Ok(StaticFileSystemModule {
            fs: linux_kernel_module::filesystem::register::<StaticFileSystem>(),
        })
    }
}

struct StaticFileSystem;

impl linux_kernel_module::filesystem::FileSystem for StaticFileSystem {}

kernel_module!(
    StaticFileSystemModule,
    author: "Alex Gaynor and Geoffrey Thomas",
    description: "An example Rust kernel module that implements a static file system",
    license: "GPL"
);
