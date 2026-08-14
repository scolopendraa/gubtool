#[macro_export]
macro_rules! parallel_scan {
    ($strategy:expr, { $($name:ident : $pattern:ident),* $(,)?  }) => {
        let scan_results = {
            use std::sync::OnceLock;
            $(let $name = OnceLock::new();)*

            let strategy = &$strategy;

            rayon::scope(|scope| {
                $(
                    let slot = &$name;

                    scope.spawn(move |_| {
                        let result = gubtool_core::aob_scanner::AobScanner::from_strategy(&$pattern, *strategy)
                            .and_then(|scanner| scanner.scan());

                        slot.set(result).unwrap();
                    });
                )*
            });

            ($($name.into_inner().unwrap(),)*)
        };

        let ($($name,)*) = scan_results;

        $(let $name = $name?;)*
    };
}
