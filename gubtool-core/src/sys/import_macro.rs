#[macro_export]
macro_rules! declare_mem_functions {
    ($game:path) => {
        use {
            crate::offsets,
            pelite::Pod,
            $crate::{
                address::Address,
                attached,
                game_version::Game,
                slice_ops::*,
                sys::{
                    ipc::{
                        FfiValue,
                        X86CallingConvention,
                        ipc_error::IpcError,
                        request_nullary_function,
                        request_parameterized_function,
                        worker_thread_dll_load_code,
                    },
                    sys_error::{ProcResult, ProcessError},
                    *,
                },
            },
        };

        pub(crate) fn ensure_game() -> ProcResult {
            match attached::game() {
                Ok($game) => Ok(()),
                _ => {
                    Err(ProcessError::InvalidGame {
                        expected: $game,
                    })
                }
            }
        }

        #[track_caller]
        pub fn read<T: Pod>(address: impl Address) -> ProcResult<T> {
            ensure_game()?;
            read_unsafe(address)
        }

        #[track_caller]
        pub fn write<T: Pod>(address: impl Address, value: T) -> ProcResult {
            ensure_game()?;
            write_unsafe(address, value)
        }

        #[track_caller]
        pub fn write_bytes(address: impl Address, data: &[u8]) -> ProcResult {
            ensure_game()?;
            write_bytes_unsafe(address, data)
        }

        #[track_caller]
        pub fn spawn_thread_join(
            thread_start_address: impl Address,
            thread_code: Vec<u8>,
        ) -> ProcResult {
            ensure_game()?;
            #[cfg(unix)]
            $crate::sys::spawn_thread_join(
                offsets::code_cave::CaveAddress::RunThreadAsm,
                thread_start_address,
                thread_code,
                offsets::module_offsets::ExternalFunctionPointer::Kernel32CreateThread,
                offsets::module_offsets::ExternalFunctionPointer::Kernel32CloseHandle,
            )?;
            #[cfg(windows)]
            $crate::sys::spawn_thread_join(thread_start_address, thread_code)?;
            Ok(())
        }

        #[track_caller]
        fn spawn_thread_release(
            thread_start_address: impl Address,
            thread_code: Vec<u8>,
        ) -> ProcResult {
            ensure_game()?;
            #[cfg(unix)]
            $crate::sys::spawn_thread_release(
                offsets::code_cave::CaveAddress::RunThreadAsm,
                thread_start_address,
                thread_code,
                offsets::module_offsets::ExternalFunctionPointer::Kernel32CreateThread,
                offsets::module_offsets::ExternalFunctionPointer::Kernel32CloseHandle,
            )?;
            #[cfg(windows)]
            $crate::sys::spawn_thread_release(thread_start_address, thread_code)?;
            Ok(())
        }

        pub fn run_custom_function(function_code: Vec<u8>) -> anyhow::Result<()> {
            ensure_game()?;
            let attached_port = resolve_attached_port()?;

            let custom_function_address = offsets::code_cave::CaveAddress::CustomFunction;

            write_bytes(custom_function_address, &function_code)?;
            request_nullary_function(attached_port, custom_function_address)?;
            Ok(())
        }

        fn resolve_attached_port() -> anyhow::Result<u16> {
            let mut attached_port = attached::port()?;
            let attached_pid = attached::pid()?;

            if attached_port.is_none()
                && let Some(port) = lookup_attached_port()?
            {
                attached::set_port(port);
                attached_port = Some(port);
            }

            if attached_port.is_none() {
                let dll_code = worker_thread_dll_load_code(
                    offsets::module_offsets::ExternalFunctionPointer::Kernel32LoadLibraryW,
                    offsets::code_cave::CaveAddress::DllPath,
                )?;

                spawn_thread_join(offsets::code_cave::CaveAddress::DllInjectCode, dll_code)?;

                let start = std::time::Instant::now();
                let wait_for_port_timeout = std::time::Duration::from_millis(100);

                loop {
                    if let Some(port) = lookup_attached_port()? {
                        attached::set_port(port);
                        attached_port = Some(port);
                        break;
                    }

                    if start.elapsed() > wait_for_port_timeout {
                        return Err(IpcError::DllInjection)?;
                    }

                    std::thread::sleep(std::time::Duration::from_micros(200))
                }
            }

            Ok(attached_port.unwrap())
        }

        fn lookup_attached_port() -> ProcResult<Option<u16>> {
            let port = read::<u16>(offsets::code_cave::CaveAddress::WorkerThreadPort)?;
            if port != 0x0 {
                Ok(Some(port))
            } else {
                Ok(None)
            }
        }

        #[track_caller]
        pub fn is_bit_set(address: impl Address, mask: u8) -> ProcResult<bool> {
            read::<u8>(address).map(|byte| byte & mask != 0)
        }

        #[track_caller]
        pub fn set_bit(address: impl Address, mask: u8, value: bool) -> ProcResult {
            let current_byte = read::<u8>(address)?;
            let new_byte = match value {
                true => current_byte | mask,
                false => current_byte & !mask,
            };
            write::<u8>(address, new_byte)
        }

        #[track_caller]
        pub fn install_hook(
            code: &[u8],
            code_location: impl Address,
            hook_location: impl Address,
            original_instruction_size: u64,
        ) -> ProcResult {
            let hookbytes =
                get_hook_bytes(code_location, hook_location, original_instruction_size)?;
            write_bytes(code_location, &code)?;
            write_bytes(hook_location, &hookbytes)
        }

        #[track_caller]
        pub fn install_hook_without_code(
            code_location: impl Address,
            hook_location: impl Address,
            original_instruction_size: u64,
        ) -> ProcResult {
            let hookbytes =
                get_hook_bytes(code_location, hook_location, original_instruction_size)?;
            write_bytes(hook_location, &hookbytes)
        }

        #[track_caller]
        pub fn read_address(address: impl Address) -> ProcResult<u64> {
            ensure_game()?;
            read_address_unsafe(address)
        }

        #[track_caller]
        pub fn follow_pointers(pointers: &[u64], read_final: bool) -> ProcResult<u64> {
            let mut pointer = 0u64;
            let (last, rest) = pointers.split_last().unwrap();
            for offset in rest {
                pointer = read_address(pointer + offset)?
            }

            if read_final {
                pointer = read_address(pointer + last)?
            } else {
                pointer += last
            }

            Ok(pointer)
        }
    };
}

#[macro_export]
macro_rules! declare_x86_specifics {
    () => {
        #[track_caller]
        pub fn run_game_function(
            function_address: impl Address,
            arguments: &[FfiValue],
            calling_convention: X86CallingConvention,
        ) -> anyhow::Result<()> {
            ensure_game()?;
            let attached_port = resolve_attached_port()?;

            request_parameterized_function(
                attached_port,
                function_address,
                arguments,
                Some(calling_convention),
            )?;
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! declare_x64_specifics {
    () => {
        #[track_caller]
        pub fn run_game_function(
            function_address: impl Address,
            arguments: &[FfiValue],
        ) -> anyhow::Result<()> {
            ensure_game()?;
            let attached_port = resolve_attached_port()?;

            request_parameterized_function(attached_port, function_address, arguments, None)?;
            Ok(())
        }
    };
}
