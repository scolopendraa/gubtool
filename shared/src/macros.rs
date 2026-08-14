#[macro_export]
macro_rules! declare_command {
    ($( $struct_name:ident $(=> $display_name:expr)? ),* $(,)?) => {
        $(
            pub struct $struct_name;

            impl std::fmt::Display for $struct_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    shared::declare_command!(@display f, $struct_name $(, $display_name)?)
                }
            }
        )*
    };

    (@display $f:ident, $struct_name:ident, $display_name:expr) => {
        write!($f, "{}", $display_name)
    };

    (@display $f:ident, $struct_name:ident) => {{
        let raw_name = stringify!($struct_name);
        let mut formatted = String::new();

        for (i, ch) in raw_name.chars().enumerate() {
            if i > 0 && ch.is_uppercase() {
                formatted.push(' ');
            }
            formatted.push(ch);
        }

        write!($f, "{}", formatted)
    }};
}
