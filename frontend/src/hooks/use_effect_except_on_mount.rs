use yew_functional::{use_effect_with_deps, use_ref};

pub fn use_effect_except_on_mount<F, Dependents>(callback: F, deps: Dependents)
where
    F: FnOnce(&Dependents) + 'static,
    Dependents: PartialEq + 'static,
{
    let is_mounted = use_ref(|| false);

    use_effect_with_deps(
        move |deps| {
            if !*is_mounted.borrow() {
                *is_mounted.borrow_mut() = true;
            } else {
                callback(deps);
            }
            || {}
        },
        deps,
    );
}
