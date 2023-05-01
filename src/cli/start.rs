use super::args::Args;
use super::commands::ArgListener;
use crate::task::Task;

macro_rules! create_listener {
    (on $r:ident; $a:expr, $t:expr) => {
        ArgListener::new($a, $t).on(&mut $r)?;
    };
    (add $r:ident; $( { $a:expr, $t:expr } ),+ $(,)?) => {
        $(
            create_listener!(on $r; $a, Some($t));
        )+
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

    create_listener!(add main_listener;
        /* $ yuki add */
        { "add", Task::Install },
        /* $ yuki remove */
        { "remove", Task::Uninstall },
        /* $ yuki show */
        { "show", Task::Show },
    );

    Ok(main_listener)
}

pub fn start() -> Result<(), String> {
    let listener = setup()?;
    listener.init(&mut Args::build())?;
    Ok(())
}
