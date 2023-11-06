use futuresdr::runtime::Pmt;
use leptos::html::Input;
use leptos::*;

#[component]
pub fn Pmt(#[prop(into)] pmt: MaybeSignal<Pmt>) -> impl IntoView {
    let class = {
        let pmt = pmt.clone();
        move || match pmt() {
            Pmt::Ok => "pmt-ok",
            Pmt::InvalidValue => "pmt-invalidvalue",
            Pmt::Null => "pmt-null",
            Pmt::String(_) => "pmt-string",
            Pmt::Bool(_) => "pmt-bool",
            Pmt::Usize(_) => "pmt-usize",
            Pmt::U32(_) => "pmt-u32",
            Pmt::U64(_) => "pmt-u64",
            Pmt::F32(_) => "pmt-f32",
            Pmt::F64(_) => "pmt-f64",
            Pmt::VecF32(_) => "pmt-vecf32",
            Pmt::VecU64(_) => "pmt-vecu64",
            Pmt::Blob(_) => "pmt-blob",
            Pmt::VecPmt(_) => "pmt-vecpmt",
            Pmt::Finished => "pmt-finished",
            Pmt::MapStrPmt(_) => "pmt-mapstrpmt",
            Pmt::Any(_) => "pmt-any",
            _ => "",
        }
    };

    view! {
        <span class=class>{ move || pmt.get().to_string() }</span>
    }
}

const ENTER_KEY: u32 = 13;

#[component]
pub fn PmtInput(
    set_pmt: WriteSignal<Pmt>,
    #[prop(into, optional)] input_class: String,
    #[prop(into, optional)] error_class: String,
) -> impl IntoView {
    let (error, error_set) = create_signal(None);
    let error_memo = create_memo(move |_| error());

    let input_ref = create_node_ref::<Input>();
    let parse_pmt = move || {
        let input = input_ref.get().unwrap();
        let v = input.value();
        if let Ok(p) = v.parse::<Pmt>() {
            set_pmt(p);
        } else {
            error_set(Some(error_class.clone()));
        }
    };

    let on_input = move |ev: web_sys::KeyboardEvent| {
        ev.stop_propagation();
        error_set(None);
        let key_code = ev.key_code();
        if key_code == ENTER_KEY {
            parse_pmt();
        }
    };

    view! {
        <input class=input_class class=error_memo node_ref=input_ref on:keydown=on_input></input>
    }
}