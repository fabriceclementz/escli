#[macro_export]
macro_rules! commands_enum {
    ($($module:ident),*) => (
        paste::paste! {
            #[derive(Debug, Subcommand)]
            enum Commands {
                $(
                    #[doc = "Interacts with `" $module "`"]
                    [<$module:camel>]($module::Arguments),
                )*
            }

            impl Commands {
                async fn run(application: &Application) -> Result<()> {
                    match &application.args.sub_command {
                        $(
                            Commands::[<$module:camel>](args) => $module::handle_command(args, &application).await?,
                        )*
                    }

                    Ok(())
                }
            }
        }
    );
}
