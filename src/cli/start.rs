use super::args::Args;
use super::commands::ArgListener;
use crate::task::Task;

macro_rules! create_listener {
    ($r:ident, $a:expr, $t:expr) => {
        ArgListener::new($a, $t,).on(&mut $r)?;
    };
    ($r:ident, $( { $a:expr, $t:expr } ),+ $(,)?) => {
        $(
            create_listener!($r, $a, Some($t));
        )*
    };
}

pub fn setup() -> Result<ArgListener, String> {
    let mut main_listener = ArgListener::new("yuki", None);

    let mut pkg_listener = ArgListener::new("pkg", None);
    /* $ yuki pkg */
    create_listener!(pkg_listener,
        /* $ yuki pkg add */
        { "add", Task::Add },
        /* $ yuki pkg remove */
        { "remove", Task::Remove },
    );

    main_listener.add_listener(pkg_listener)?;

    /* $ yuki add */
    /* create_listener!(main_listener, "add", Some(Task::Install));
    /* $ yuki remove */
    create_listener!(main_listener, "remove", Some(Task::Uninstall));
    /* $ yuki show */
    create_listener!(main_listener, "show", Some(Task::Show)); */
    create_listener!(main_listener,
        { "add", Task::Install },
        { "remove", Task::Uninstall },
        { "show", Task::Show },
    );

    Ok(main_listener)
}

pub fn start() -> Result<(), String> {
    let listener = setup()?;
    listener.init(&mut Args::build())?;
    Ok(())
}
