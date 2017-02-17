// Build some with builder ><
// let window_main: gtk::Window = builder.get_object("window_main").expect("Could not find 'window_main' in glade file");
// let window_main: gtk::Window = build!(builder, "window_main");
#[macro_export] macro_rules! build {
    ($builder:ident, $e:expr) => { $builder.get_object($e).expect(&format!("Could not find '{}' in glade file", $e)) };
}

// make moving clones into closures more convenient
#[macro_export] macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

#[macro_export] macro_rules! Y { () => {} }
