use super::args::Args;
use super::commands::ArgListener;
use crate::Error;
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
    (create $r:ident, $c:expr; $( { $a:expr, $t:expr } ),+ $(,)?) => {
        let mut $r = ArgListener::new($c, None);
        create_listener!(add $r;
        $(
            { $a, $t },
        )+
        );
    };
}

pub fn setup() -> Result<ArgListener, Error> {
    let mut main_listener = ArgListener::new("yki", None);

    /* $ yki config */
    create_listener!(create config_listener, "config";
        /* $ yki config init */
        { "init", Task::ConfigInit },
        /* $ yki config create */
        { "create", Task::ConfigCreate },
    );

    main_listener.add_listener(config_listener)?;

    /* $ yki pkg */
    create_listener!(create pkg_listener, "pkg";
        /* $ yki pkg add */
        { "add", Task::Add },
        /* $ yki pkg remove */
        { "remove", Task::Remove },
    );

    main_listener.add_listener(pkg_listener)?;

    create_listener!(add main_listener;
        /* $ yki add */
        { "add", Task::Install },
        /* $ yki remove */
        { "remove", Task::Uninstall },
        /* $ yki show */
        { "show", Task::Show },
    );

    Ok(main_listener)
}

pub fn start() -> Result<(), Error> {
    let listener = setup()?;
    listener.init(&mut Args::build())?;
    Ok(())
}
